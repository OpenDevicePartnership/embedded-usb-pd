//! Types for SET_POWER_LEVEL command, see UCSI spec 6.5.19

use bitfield::bitfield;
use bytemuck::{Pod, Zeroable};

use crate::pdo::{MA50_UNIT, MV20_UNIT, MV25_UNIT, MW1000_UNIT, MW500_UNIT};
use crate::ucsi::v1_2::{CommandHeaderRaw, COMMAND_LEN};
use crate::{type_c, PortId, PowerRole};

/// Length of the raw argument bits, this command uses the entire payload
pub const ARG_BITS_LEN: usize = COMMAND_LEN - size_of::<CommandHeaderRaw>();

bitfield! {
    /// Raw argument bits
    ///
    /// Several fields cross byte boundaries, so this covers the entire payload.
    #[derive(Copy, Clone, Default, PartialEq, Eq)]
    pub struct ArgBitsRaw([u8]);
    impl Debug;

    /// Connector number
    pub u8, connector_number, set_connector_number: 6, 0;
    /// Power role, 1 = source
    pub bool, power_role, set_power_role: 7;
    /// Max PD power, in 0.5W/1W units depending on [`Self::lsb_control`]
    pub u8, max_power, set_max_power: 15, 8;
    /// Type-C current
    pub u8, type_c_current, set_type_c_current: 18, 16;
    /// Units for [`Self::max_power`] and [`Self::output_voltage`]
    pub bool, lsb_control, set_lsb_control: 19;
    /// Operating current in 50mA units
    pub u8, operating_current, set_operating_current: 27, 20;
    /// Output voltage in 20mV/25mV units depending on [`Self::lsb_control`]
    pub u16, output_voltage, set_output_voltage: 41, 30;
}

#[cfg(feature = "defmt")]
impl defmt::Format for ArgBitsRaw<[u8; ARG_BITS_LEN]> {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "ArgBitsRaw {{ .0: {}, \
            connector_number: {}, \
            power_role: {}, \
            max_power: {}, \
            type_c_current: {}, \
            lsb_control: {}, \
            operating_current: {}, \
            output_voltage: {} }}",
            self.0,
            self.connector_number(),
            self.power_role(),
            self.max_power(),
            self.type_c_current(),
            self.lsb_control(),
            self.operating_current(),
            self.output_voltage()
        )
    }
}

/// Type-C current
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Current {
    /// PPM default
    #[default]
    PpmDefault,
    /// Type-C current
    Current(type_c::Current),
}

/// Type-C current decode error, contains the invalid value
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct InvalidCurrent(pub u8);

impl TryFrom<u8> for Current {
    type Error = InvalidCurrent;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Current::PpmDefault),
            0x01 => Ok(Current::Current(type_c::Current::Current3A0)),
            0x02 => Ok(Current::Current(type_c::Current::Current1A5)),
            0x03 => Ok(Current::Current(type_c::Current::UsbDefault)),
            v => Err(InvalidCurrent(v)),
        }
    }
}

impl From<Current> for u8 {
    fn from(value: Current) -> Self {
        match value {
            Current::PpmDefault => 0x00,
            Current::Current(type_c::Current::Current3A0) => 0x01,
            Current::Current(type_c::Current::Current1A5) => 0x02,
            Current::Current(type_c::Current::UsbDefault) => 0x03,
        }
    }
}

/// Command arguments
///
/// Power, current and voltage are stored in mW, mA and mV. Converting to the raw representation
/// truncates them to the wire resolution selected by [`Self::lsb_control`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Args<T: PortId> {
    /// Connector number
    pub connector_number: T,
    /// Power role
    pub power_role: PowerRole,
    /// Units for [`Self::max_power`] and [`Self::output_voltage`]
    ///
    /// `false` selects 500 mW/20 mV units, `true` selects 1 W/25 mV units.
    pub lsb_control: bool,
    /// Max PD power in mW
    pub max_power: u32,
    /// Type-C current
    pub type_c_current: Current,
    /// Operating current in mA
    pub operating_current: u16,
    /// Output voltage in mV
    pub output_voltage: u32,
}

impl<T: PortId> Default for Args<T> {
    fn default() -> Self {
        Self {
            connector_number: T::from(0),
            power_role: PowerRole::default(),
            lsb_control: false,
            max_power: 0,
            type_c_current: Current::default(),
            operating_current: 0,
            output_voltage: 0,
        }
    }
}

impl<T: PortId> TryFrom<ArgBitsRaw<[u8; ARG_BITS_LEN]>> for Args<T> {
    type Error = InvalidCurrent;

    fn try_from(raw: ArgBitsRaw<[u8; ARG_BITS_LEN]>) -> Result<Self, Self::Error> {
        let lsb_control = raw.lsb_control();
        let (power_unit, voltage_unit) = if lsb_control {
            (MW1000_UNIT, MV25_UNIT)
        } else {
            (MW500_UNIT, MV20_UNIT)
        };

        Ok(Self {
            connector_number: raw.connector_number().into(),
            power_role: if raw.power_role() {
                PowerRole::Source
            } else {
                PowerRole::Sink
            },
            lsb_control,
            max_power: u32::from(raw.max_power()) * power_unit,
            type_c_current: Current::try_from(raw.type_c_current())?,
            operating_current: u16::from(raw.operating_current()) * MA50_UNIT,
            output_voltage: u32::from(raw.output_voltage()) * u32::from(voltage_unit),
        })
    }
}

impl<T: PortId> TryFrom<Args<T>> for ArgBitsRaw<[u8; ARG_BITS_LEN]> {
    type Error = OverflowError;

    fn try_from(args: Args<T>) -> Result<Self, Self::Error> {
        let (power_unit, voltage_unit) = if args.lsb_control {
            (MW1000_UNIT, MV25_UNIT)
        } else {
            (MW500_UNIT, MV20_UNIT)
        };

        let mut raw = ArgBitsRaw([0; ARG_BITS_LEN]);
        raw.set_connector_number(args.connector_number.into());
        raw.set_power_role(args.power_role == PowerRole::Source);
        raw.set_lsb_control(args.lsb_control);

        let max_power = args.max_power / power_unit;
        if max_power > u8::MAX as u32 {
            return Err(OverflowError::MaxPowerOverflow(args.max_power));
        }
        raw.set_max_power(max_power as u8);

        raw.set_type_c_current(args.type_c_current.into());
        let operating_current = args.operating_current / MA50_UNIT;
        if operating_current > u8::MAX as u16 {
            return Err(OverflowError::OperatingCurrentOverflow(args.operating_current));
        }
        raw.set_operating_current(operating_current as u8);

        let output_voltage = args.output_voltage / u32::from(voltage_unit);
        if output_voltage > 0x0FFF {
            return Err(OverflowError::OutputVoltageOverflow(args.output_voltage));
        }
        raw.set_output_voltage(output_voltage as u16);
        Ok(raw)
    }
}

/// Error type for [`Args`] conversion failures
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OverflowError {
    /// The max power value cannot fit in 8 bits
    MaxPowerOverflow(u32),
    /// The operating current value cannot fit in 8 bits
    OperatingCurrentOverflow(u16),
    /// The output voltage value cannot fit in 16 bits
    OutputVoltageOverflow(u32),
}

/// Raw wire format of [`Args`]
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Zeroable, Pod)]
pub struct ArgsRaw {
    /// Argument bits, see [`ArgBitsRaw`]
    pub bits: [u8; ARG_BITS_LEN],
}

impl ArgsRaw {
    /// Length of the raw arguments in bytes
    pub const LEN: usize = size_of::<Self>();
}

#[cfg(feature = "defmt")]
impl defmt::Format for ArgsRaw {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "ArgsRaw {{ bits: {} }}", ArgBitsRaw(self.bits))
    }
}

impl<T: PortId> TryFrom<Args<T>> for ArgsRaw {
    type Error = OverflowError;

    fn try_from(args: Args<T>) -> Result<Self, Self::Error> {
        Ok(Self {
            bits: ArgBitsRaw::try_from(args)?.0,
        })
    }
}

impl<T: PortId> TryFrom<ArgsRaw> for Args<T> {
    type Error = InvalidCurrent;

    fn try_from(raw: ArgsRaw) -> Result<Self, Self::Error> {
        ArgBitsRaw(raw.bits).try_into()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::GlobalPortId;

    #[test]
    fn test_raw_len() {
        assert_eq!(ArgsRaw::LEN, COMMAND_LEN - size_of::<CommandHeaderRaw>());
    }

    #[test]
    fn test_current_roundtrip() {
        for raw in 0..=u8::MAX {
            match Current::try_from(raw) {
                Ok(current) => assert_eq!(u8::from(current), raw),
                Err(err) => {
                    assert!(raw > 0x03);
                    assert_eq!(err, InvalidCurrent(raw));
                }
            }
        }
    }

    #[test]
    fn test_args_raw_roundtrip() {
        // Source on connector 3
        // 1W max power
        // 1.5A Type-C current
        // 100 mA operating current
        // 60 mV output voltage
        let encoded: [u8; ArgsRaw::LEN] = [0x83, 0x02, 0x22, 0xC0, 0x00, 0x00];
        let expected: Args<GlobalPortId> = Args {
            connector_number: GlobalPortId(3),
            power_role: PowerRole::Source,
            lsb_control: false,
            max_power: 1000,
            type_c_current: Current::Current(type_c::Current::Current1A5),
            operating_current: 100,
            output_voltage: 60,
        };

        assert_eq!(Args::try_from(bytemuck::must_cast::<_, ArgsRaw>(encoded)), Ok(expected));
        let bytes: [u8; ArgsRaw::LEN] = bytemuck::must_cast(ArgsRaw::try_from(expected).unwrap());
        assert_eq!(bytes, encoded);
    }

    #[test]
    fn test_args_raw_cross_byte_fields() {
        // Sink on connector 1, 1W/25mV units, every multi-byte field at its maximum
        // Operating current spans bytes 2-3, output voltage spans bytes 3-5
        let encoded: [u8; ArgsRaw::LEN] = [0x01, 0xFF, 0xF8, 0xCF, 0xFF, 0x03];
        let expected: Args<GlobalPortId> = Args {
            connector_number: GlobalPortId(1),
            power_role: PowerRole::Sink,
            lsb_control: true,
            max_power: 255 * MW1000_UNIT,
            type_c_current: Current::PpmDefault,
            operating_current: 255 * MA50_UNIT,
            output_voltage: 4095 * u32::from(MV25_UNIT),
        };

        assert_eq!(Args::try_from(bytemuck::must_cast::<_, ArgsRaw>(encoded)), Ok(expected));
        let bytes: [u8; ArgsRaw::LEN] = bytemuck::must_cast(ArgsRaw::try_from(expected).unwrap());
        assert_eq!(bytes, encoded);
    }

    #[test]
    fn test_args_raw_truncates_to_units() {
        let args: Args<GlobalPortId> = Args {
            lsb_control: false,
            max_power: 1499,
            operating_current: 149,
            output_voltage: 59,
            ..Default::default()
        };
        let expected: Args<GlobalPortId> = Args {
            lsb_control: false,
            max_power: 1000,
            operating_current: 100,
            output_voltage: 40,
            ..Default::default()
        };

        assert_eq!(Args::try_from(ArgsRaw::try_from(args).unwrap()), Ok(expected));
    }

    #[test]
    fn test_args_raw_invalid_current() {
        // Connector 0, invalid type_c_current value (0x4) at bits 18:16
        let encoded: [u8; ArgsRaw::LEN] = [0x00, 0x00, 0x04, 0x00, 0x00, 0x00];
        assert_eq!(
            Args::<GlobalPortId>::try_from(bytemuck::must_cast::<_, ArgsRaw>(encoded)),
            Err(InvalidCurrent(0x04))
        );
    }

    #[test]
    fn test_args_raw_invalid_max_power() {
        // Connector 0, max_power overflow
        let args: Args<GlobalPortId> = Args {
            // 256 * 500 mW
            max_power: 128000,
            ..Default::default()
        };
        assert_eq!(ArgsRaw::try_from(args), Err(OverflowError::MaxPowerOverflow(128000)));
    }

    #[test]
    fn test_args_raw_invalid_operating_current() {
        // Connector 0, operating_current overflow
        let args: Args<GlobalPortId> = Args {
            // 256 * 50 mA
            operating_current: 12800,
            ..Default::default()
        };
        assert_eq!(
            ArgsRaw::try_from(args),
            Err(OverflowError::OperatingCurrentOverflow(12800))
        );
    }

    #[test]
    fn test_args_raw_invalid_output_voltage() {
        // Connector 0, output_voltage overflow
        let args: Args<GlobalPortId> = Args {
            // 65535 * 20 mV
            output_voltage: 1310720,
            ..Default::default()
        };
        assert_eq!(
            ArgsRaw::try_from(args),
            Err(OverflowError::OutputVoltageOverflow(1310720))
        );
    }
}

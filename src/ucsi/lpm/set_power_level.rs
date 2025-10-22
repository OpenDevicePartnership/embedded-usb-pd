//! Types for SET_POWER_LEVEL command, see UCSI spec 6.5.19

use bincode::de::Decoder;
use bincode::enc::Encoder;
use bincode::error::{DecodeError, EncodeError};
use bincode::{Decode, Encode};
use bitfield::bitfield;

use crate::pdo::{MA50_UNIT, MV20_UNIT, MV25_UNIT, MW1000_UNIT, MW500_UNIT};
use crate::{type_c, PowerRole};

/// Command data length
pub const COMMAND_DATA_LEN: usize = 6;

bitfield! {
    /// Raw arguments
    #[derive(Copy, Clone, Default, PartialEq, Eq)]
    pub(super) struct ArgsRaw([u8]);
    impl Debug;

    /// Connector number
    pub u8, connector_number, set_connector_number: 6, 0;
    /// Power role, 1 = source
    pub bool, power_role, set_power_role: 7;
    /// Max PD power, in 0.5W/1W units depending on [`lsb_control`]
    pub u8, max_power, set_max_power: 15, 8;
    /// Type-C current
    pub u8, type_c_current, set_type_c_current: 18, 16;
    /// Units for [`max_power`] and [`output_voltage`]
    pub bool, lsb_control, set_lsb_control: 19;
    /// Operating current in 50mA units
    pub u8, operating_current, set_operating_current: 27, 20;
    /// Output voltage in 20mV/25mV units depending on [`lsb_control`]
    pub u16, output_voltage, set_output_voltage: 41, 30;
}

#[cfg(feature = "defmt")]
impl defmt::Format for ArgsRaw<[u8; COMMAND_DATA_LEN]> {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "ArgsRaw {{ .0: {}, \
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

/// Type-C decode error, contains the invalid value
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
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Args(ArgsRaw<[u8; COMMAND_DATA_LEN]>);

impl Args {
    pub fn connector_number(&self) -> u8 {
        self.0.connector_number()
    }

    pub fn set_connector_number(&mut self, connector_number: u8) -> &mut Self {
        self.0.set_connector_number(connector_number);
        self
    }

    pub fn power_role(&self) -> PowerRole {
        match self.0.power_role() {
            true => PowerRole::Source,
            false => PowerRole::Sink,
        }
    }

    pub fn set_power_role(&mut self, power_role: PowerRole) -> &mut Self {
        self.0.set_power_role(power_role == PowerRole::Source);
        self
    }

    /// Max power in mW
    pub fn max_power(&self) -> u16 {
        match self.0.lsb_control() {
            true => self.0.max_power() as u16 * MW1000_UNIT as u16,
            false => self.0.max_power() as u16 * MW500_UNIT as u16,
        }
    }

    pub fn type_c_current(&self) -> Current {
        let current: Result<Current, _> = Current::try_from(self.0.type_c_current());
        current.unwrap() // Won't panic, validated in try_from
    }

    pub fn set_type_c_current(&mut self, type_c_current: Current) -> &mut Self {
        self.0.set_type_c_current(type_c_current.into());
        self
    }

    /// Operating current in mA
    pub fn operating_current(&self) -> u16 {
        self.0.operating_current() as u16 * MA50_UNIT
    }

    /// Set operating current in mA
    pub fn set_operating_current(&mut self, operating_current: u16) -> &mut Self {
        self.0.set_operating_current((operating_current / MA50_UNIT) as u8);
        self
    }

    /// Output voltage in mV
    pub fn output_voltage(&self) -> u16 {
        match self.0.lsb_control() {
            true => self.0.output_voltage() * MV25_UNIT,
            false => self.0.output_voltage() * MV20_UNIT,
        }
    }

    /// Sets LSB-control, output voltage, and max power
    pub fn set_power_args(&mut self, lsb_control: bool, output_voltage: u16, max_power: u16) -> &mut Self {
        self.0.set_lsb_control(lsb_control);

        match lsb_control {
            true => {
                self.0.set_output_voltage(output_voltage / MV25_UNIT);
                self.0.set_max_power((max_power / MW1000_UNIT as u16) as u8)
            }
            false => {
                self.0.set_output_voltage(output_voltage / MV20_UNIT);
                self.0.set_max_power((max_power / MW500_UNIT as u16) as u8);
            }
        }
        self
    }
}

impl TryFrom<[u8; COMMAND_DATA_LEN]> for Args {
    type Error = InvalidCurrent;

    fn try_from(value: [u8; COMMAND_DATA_LEN]) -> Result<Self, Self::Error> {
        let raw = ArgsRaw(value);
        let _current = Current::try_from(raw.type_c_current())?;
        Ok(Self(raw))
    }
}

impl From<Args> for [u8; COMMAND_DATA_LEN] {
    fn from(args: Args) -> Self {
        args.0 .0
    }
}

impl Encode for Args {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        Encode::encode(&self.0 .0, encoder)
    }
}

impl<Context> Decode<Context> for Args {
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, DecodeError> {
        let raw = <[u8; COMMAND_DATA_LEN]>::decode(decoder)?;
        Args::try_from(raw).map_err(|invalid_current| DecodeError::UnexpectedVariant {
            type_name: "Current",
            allowed: &bincode::error::AllowedEnumVariants::Range { min: 0, max: 3 },
            found: invalid_current.0 as u32,
        })
    }
}

#[cfg(test)]
mod test {
    use bincode::config::standard;
    use bincode::decode_from_slice;

    use super::*;

    #[test]
    fn test_decode_args() {
        // Source on connector 3
        // 1W max power
        // USB default current
        // 100 mA operating current
        // 60 mV output voltage
        let encoded: [u8; COMMAND_DATA_LEN] = [0x83, 0x02, 0x22, 0xC0, 0x00, 0x00];
        let (decoded, size): (Args, usize) = decode_from_slice(&encoded, standard().with_fixed_int_encoding()).unwrap();
        assert_eq!(size, COMMAND_DATA_LEN);

        let expected = *Args::default()
            .set_connector_number(3)
            .set_operating_current(100)
            .set_power_role(PowerRole::Source)
            .set_type_c_current(Current::Current(type_c::Current::Current1A5))
            .set_power_args(false, 60, 1000);
        assert_eq!(decoded, expected);
    }
}

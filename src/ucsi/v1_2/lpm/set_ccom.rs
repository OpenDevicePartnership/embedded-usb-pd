//! Types for SET_CCOM command, see UCSI spec 6.5.8

use bitfield::bitfield;
use bytemuck::{Pod, Zeroable};
use pack1::U16LE;

use crate::ucsi::v1_2::{CommandHeaderRaw, COMMAND_LEN};
use crate::PortId;

/// Command padding
pub const COMMAND_PADDING: usize = COMMAND_LEN - size_of::<CommandHeaderRaw>() - size_of::<ArgBitsRaw>();

bitfield! {
    /// Raw argument bits
    #[derive(Copy, Clone, Default, PartialEq, Eq)]
    pub struct ArgBitsRaw(u16);
    impl Debug;

    /// Connector number
    pub u8, connector_number, set_connector_number: 6, 0;
    /// Rp
    pub bool, rp, set_rp: 7;
    /// Rd
    pub bool, rd, set_rd: 8;
    /// Drp
    pub bool, drp, set_drp: 9;
    /// Disabled
    pub bool, disabled, set_disabled: 10;
}

#[cfg(feature = "defmt")]
impl defmt::Format for ArgBitsRaw {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "ArgBitsRaw {{ .0: {}, connector_number: {}, rp: {}, rd: {}, drp: {}, disabled: {} }}",
            self.0,
            self.connector_number(),
            self.rp(),
            self.rd(),
            self.drp(),
            self.disabled()
        )
    }
}

/// Command arguments
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Args<T: PortId> {
    /// Connector number
    pub connector_number: T,
    /// Rp
    pub rp: bool,
    /// Rd
    pub rd: bool,
    /// Drp
    pub drp: bool,
    /// Disabled
    pub disabled: bool,
}

impl<T: PortId> Default for Args<T> {
    fn default() -> Self {
        Self {
            connector_number: T::from(0),
            rp: false,
            rd: false,
            drp: false,
            disabled: false,
        }
    }
}

impl<T: PortId> From<ArgBitsRaw> for Args<T> {
    fn from(raw: ArgBitsRaw) -> Self {
        Self {
            connector_number: raw.connector_number().into(),
            rp: raw.rp(),
            rd: raw.rd(),
            drp: raw.drp(),
            disabled: raw.disabled(),
        }
    }
}

impl<T: PortId> From<Args<T>> for ArgBitsRaw {
    fn from(args: Args<T>) -> Self {
        let mut raw = ArgBitsRaw(0);
        raw.set_connector_number(args.connector_number.into());
        raw.set_rp(args.rp);
        raw.set_rd(args.rd);
        raw.set_drp(args.drp);
        raw.set_disabled(args.disabled);
        raw
    }
}

impl<T: PortId> From<u16> for Args<T> {
    fn from(value: u16) -> Self {
        ArgBitsRaw(value).into()
    }
}

impl<T: PortId> From<Args<T>> for u16 {
    fn from(args: Args<T>) -> Self {
        ArgBitsRaw::from(args).0
    }
}

/// Raw wire format of [`Args`]
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Zeroable, Pod)]
pub struct ArgsRaw {
    /// Argument bits, see [`ArgBitsRaw`]
    pub bits: U16LE,
    /// Reserved bytes, filling out the remainder of the command
    _reserved: [u8; COMMAND_PADDING],
}

impl ArgsRaw {
    /// Length of the raw arguments in bytes
    pub const LEN: usize = size_of::<Self>();
}

#[cfg(feature = "defmt")]
impl defmt::Format for ArgsRaw {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "ArgsRaw {{ bits: {} }}", ArgBitsRaw(self.bits.get()))
    }
}

impl<T: PortId> From<Args<T>> for ArgsRaw {
    fn from(args: Args<T>) -> Self {
        Self {
            bits: U16LE::new(args.into()),
            ..Default::default()
        }
    }
}

impl<T: PortId> From<ArgsRaw> for Args<T> {
    fn from(raw: ArgsRaw) -> Self {
        raw.bits.get().into()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::GlobalPortId;

    /// Mask of all bits defined by [`ArgBitsRaw`]
    const DEFINED_BITS: u16 = 0x07FF;

    #[test]
    fn test_raw_len() {
        assert_eq!(ArgsRaw::LEN, COMMAND_LEN - size_of::<CommandHeaderRaw>());
    }

    #[test]
    fn test_arg_bits_roundtrip() {
        for bit in 0..u16::BITS {
            let raw = 1u16 << bit;
            // Undefined bits are dropped by the roundtrip
            assert_eq!(u16::from(Args::<GlobalPortId>::from(raw)), raw & DEFINED_BITS);
        }
    }

    #[test]
    fn test_args_raw_roundtrip() {
        // Rp/DRP on connector 3, the Rp bit shares a byte with the connector number
        let encoded: [u8; ArgsRaw::LEN] = [0x83, 0x02, 0x00, 0x00, 0x00, 0x00];
        let expected: Args<GlobalPortId> = Args {
            connector_number: GlobalPortId(3),
            rp: true,
            drp: true,
            ..Default::default()
        };

        assert_eq!(Args::from(bytemuck::must_cast::<_, ArgsRaw>(encoded)), expected);
        let bytes: [u8; ArgsRaw::LEN] = bytemuck::must_cast(ArgsRaw::from(expected));
        assert_eq!(bytes, encoded);
    }

    #[test]
    fn test_args_raw_ignores_reserved() {
        let encoded: [u8; ArgsRaw::LEN] = [0x01, 0xF8, 0xFF, 0xFF, 0xFF, 0xFF];
        let expected: Args<GlobalPortId> = Args {
            connector_number: GlobalPortId(1),
            ..Default::default()
        };
        assert_eq!(Args::from(bytemuck::must_cast::<_, ArgsRaw>(encoded)), expected);
    }
}

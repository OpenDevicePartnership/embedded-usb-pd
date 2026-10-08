//! Types for CONNECTOR_RESET command, see UCSI spec 6.5.3
use bytemuck::{Pod, Zeroable};

use crate::ucsi::v1_2::lpm::ConnectorNumberRaw;
use crate::ucsi::v1_2::{CommandHeaderRaw, COMMAND_LEN};
use crate::PortId;

/// Command padding
pub const COMMAND_PADDING: usize = COMMAND_LEN - size_of::<CommandHeaderRaw>() - size_of::<ConnectorNumberRaw>();

/// Command arguments
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Args<T: PortId> {
    /// Connector number
    pub connector_number: T,
    /// Perform a Hard Reset instead of a Data Reset
    pub hard_reset: bool,
}

impl<T: PortId> Default for Args<T> {
    fn default() -> Self {
        Self {
            connector_number: T::from(0),
            hard_reset: false,
        }
    }
}

/// Raw wire format of [`Args`]
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Zeroable, Pod)]
pub struct ArgsRaw {
    /// Connector number in bits 6:0, hard reset in bit 7
    pub connector: u8,
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
        let connector = ConnectorNumberRaw(self.connector);
        defmt::write!(
            fmt,
            "ArgsRaw {{ connector_number: {}, hard_reset: {} }}",
            connector.connector_number(),
            connector.high_bit()
        )
    }
}

impl<T: PortId> From<Args<T>> for ArgsRaw {
    fn from(args: Args<T>) -> Self {
        let mut connector = ConnectorNumberRaw::default();
        connector.set_connector_number(args.connector_number.into());
        connector.set_high_bit(args.hard_reset);
        Self {
            connector: connector.0,
            ..Default::default()
        }
    }
}

impl<T: PortId> From<ArgsRaw> for Args<T> {
    fn from(raw: ArgsRaw) -> Self {
        let connector = ConnectorNumberRaw(raw.connector);
        Self {
            connector_number: connector.connector_number().into(),
            hard_reset: connector.high_bit(),
        }
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
    fn test_args_raw_roundtrip() {
        // Hard reset on connector 1
        let encoded: [u8; ArgsRaw::LEN] = [0x81, 0x00, 0x00, 0x00, 0x00, 0x00];
        let expected: Args<GlobalPortId> = Args {
            connector_number: GlobalPortId(1),
            hard_reset: true,
        };

        assert_eq!(Args::from(bytemuck::must_cast::<_, ArgsRaw>(encoded)), expected);
        let bytes: [u8; ArgsRaw::LEN] = bytemuck::must_cast(ArgsRaw::from(expected));
        assert_eq!(bytes, encoded);
    }

    #[test]
    fn test_args_raw_ignores_reserved() {
        let encoded: [u8; ArgsRaw::LEN] = [0x03, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
        let expected: Args<GlobalPortId> = Args {
            connector_number: GlobalPortId(3),
            hard_reset: false,
        };

        assert_eq!(Args::from(bytemuck::must_cast::<_, ArgsRaw>(encoded)), expected);
    }
}

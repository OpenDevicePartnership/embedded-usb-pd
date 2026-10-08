//! Types for GET_CURRENT_CAM command, see UCSI spec 6.5.13

use bytemuck::{Pod, Zeroable};

use crate::ucsi::v1_2::lpm::ConnectorNumberRaw;
use crate::ucsi::v1_2::{CommandHeaderRaw, COMMAND_LEN};
use crate::PortId;

/// Data length for the GET_CAM_SUPPORTED command response
/// This matches the mailbox size
pub const RESPONSE_DATA_LEN: usize = 16;
/// Command padding
pub const COMMAND_PADDING: usize = COMMAND_LEN - size_of::<CommandHeaderRaw>() - size_of::<ConnectorNumberRaw>();

/// Command arguments
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Args<T: PortId> {
    /// Connector number
    pub connector_number: T,
}

impl<T: PortId> Default for Args<T> {
    fn default() -> Self {
        Self {
            connector_number: T::from(0),
        }
    }
}

/// Raw wire format of [`Args`]
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Zeroable, Pod)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ArgsRaw {
    /// Connector number in bits 6:0
    pub connector: u8,
    /// Reserved bytes, filling out the remainder of the command
    _reserved: [u8; COMMAND_PADDING],
}

impl ArgsRaw {
    /// Length of the raw arguments in bytes
    pub const LEN: usize = size_of::<Self>();
}

impl<T: PortId> From<Args<T>> for ArgsRaw {
    /// Converts the arguments into their raw wire format
    fn from(args: Args<T>) -> Self {
        let mut connector = ConnectorNumberRaw::default();
        connector.set_connector_number(args.connector_number.into());
        Self {
            connector: connector.0,
            ..Default::default()
        }
    }
}

impl<T: PortId> From<ArgsRaw> for Args<T> {
    /// Reconstructs the arguments from their raw wire format
    fn from(raw: ArgsRaw) -> Self {
        Self {
            connector_number: ConnectorNumberRaw(raw.connector).connector_number().into(),
        }
    }
}

/// GET_CURRENT_CAM response data, supports up to [`RESPONSE_DATA_LEN`] alternate modes
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ResponseData {
    pub alt_modes: [u8; RESPONSE_DATA_LEN],
}

/// Raw wire format of [`ResponseData`]
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Zeroable, Pod)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ResponseDataRaw {
    /// Current alternate mode for each supported alternate mode
    pub alt_modes: [u8; RESPONSE_DATA_LEN],
}

impl ResponseDataRaw {
    /// Length of the raw response data in bytes
    pub const LEN: usize = size_of::<Self>();
}

impl From<ResponseDataRaw> for ResponseData {
    fn from(raw: ResponseDataRaw) -> Self {
        Self {
            alt_modes: raw.alt_modes,
        }
    }
}

impl From<ResponseData> for ResponseDataRaw {
    fn from(data: ResponseData) -> Self {
        Self {
            alt_modes: data.alt_modes,
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
        assert_eq!(ResponseDataRaw::LEN, RESPONSE_DATA_LEN);
    }

    #[test]
    fn test_args_raw_roundtrip() {
        let encoded: [u8; ArgsRaw::LEN] = [0x03, 0x00, 0x00, 0x00, 0x00, 0x00];
        let args: Args<GlobalPortId> = Args {
            connector_number: GlobalPortId(3),
        };

        assert_eq!(
            bytemuck::must_cast::<_, [u8; ArgsRaw::LEN]>(ArgsRaw::from(args)),
            encoded
        );
        assert_eq!(Args::from(bytemuck::must_cast::<_, ArgsRaw>(encoded)), args);
    }

    #[test]
    fn test_response_data_roundtrip() {
        let bytes = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
        ];
        let expected = ResponseData { alt_modes: bytes };

        assert_eq!(
            ResponseData::from(bytemuck::must_cast::<_, ResponseDataRaw>(bytes)),
            expected
        );
        let encoded: [u8; RESPONSE_DATA_LEN] = bytemuck::must_cast(ResponseDataRaw::from(expected));
        assert_eq!(encoded, bytes);
    }
}

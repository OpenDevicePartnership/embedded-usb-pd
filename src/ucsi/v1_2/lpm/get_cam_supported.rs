//! Types for GET_CAM_SUPPORTED command, see UCSI spec 6.5.12
use bytemuck::{Pod, Zeroable};

use crate::ucsi::v1_2::lpm::ConnectorNumberRaw;
use crate::ucsi::v1_2::{CommandHeaderRaw, COMMAND_LEN};
use crate::PortId;

/// Data length for the GET_CAM_SUPPORTED command response
pub const RESPONSE_DATA_LEN: usize = 1;
/// Command padding
pub const COMMAND_PADDING: usize = COMMAND_LEN - size_of::<CommandHeaderRaw>() - size_of::<ConnectorNumberRaw>();
/// Maximum number of alternate modes supported
pub const MAX_ALT_MODES: usize = 8;

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

/// GET_CAM_SUPPORTED response data, supports up to [`MAX_ALT_MODES`] alternate modes
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ResponseData {
    /// Support flag for each alternate mode
    pub alt_modes: [bool; MAX_ALT_MODES],
}

/// Raw wire format of [`ResponseData`]
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Zeroable, Pod)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ResponseDataRaw {
    /// Bitmap of supported alternate modes
    pub alt_modes: u8,
}

impl ResponseDataRaw {
    /// Length of the raw response data in bytes
    pub const LEN: usize = size_of::<Self>();
}

impl From<ResponseDataRaw> for ResponseData {
    fn from(raw: ResponseDataRaw) -> Self {
        let mut data = ResponseData::default();
        for (index, alt_mode) in data.alt_modes.iter_mut().enumerate() {
            *alt_mode = (raw.alt_modes & (1 << index)) != 0;
        }
        data
    }
}

impl From<ResponseData> for ResponseDataRaw {
    fn from(data: ResponseData) -> Self {
        let mut alt_modes = 0u8;
        for (index, supported) in data.alt_modes.iter().enumerate() {
            if *supported {
                alt_modes |= 1 << index;
            }
        }
        Self { alt_modes }
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
        let bytes = [0x12u8; RESPONSE_DATA_LEN];
        let expected = ResponseData {
            alt_modes: [false, true, false, false, true, false, false, false],
        };

        let data = ResponseData::from(bytemuck::must_cast::<_, ResponseDataRaw>(bytes));
        assert_eq!(data, expected);

        let encoded: [u8; RESPONSE_DATA_LEN] = bytemuck::must_cast(ResponseDataRaw::from(expected));
        assert_eq!(encoded, bytes);
    }
}

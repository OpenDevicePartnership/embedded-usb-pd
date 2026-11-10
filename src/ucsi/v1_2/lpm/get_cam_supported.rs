//! Types for GET_CAM_SUPPORTED command, see UCSI spec 6.5.12
use bytemuck::{Pod, Zeroable};

use crate::ucsi::v1_2::lpm::ConnectorNumberRaw;
use crate::ucsi::v1_2::{CommandHeaderRaw, COMMAND_LEN};
use crate::PortId;

/// Data length for the GET_CAM_SUPPORTED command response
pub const RESPONSE_DATA_LEN: usize = 16;
/// Command padding
pub const COMMAND_PADDING: usize = COMMAND_LEN - size_of::<CommandHeaderRaw>() - size_of::<ConnectorNumberRaw>();
/// Maximum number of alternate modes supported
pub const MAX_ALT_MODES: usize = 128;

/// Command arguments
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Args<T: PortId> {
    /// Connector number
    pub connector_number: T,
}

impl<T: PortId, Out: PortId> crate::ucsi::MapPort<T, Out> for Args<T> {
    type Output = Args<Out>;

    fn map_port<F>(self, f: F) -> Self::Output
    where
        F: Fn(T) -> Out,
    {
        Args {
            connector_number: f(self.connector_number),
        }
    }
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
///
/// This is one of the few UCSI types that implements setters/getters instead of pure public fields.
/// A `[bool; 128]` is 128-bytes, nothing crazy. But UCSI v3.0 supports up to 2040 alternate modes.
/// 2KB for a struct is excessive, particularly when it's highly unlikely that a connector would support
/// more than a handful of alternate modes. Setters and getters allow for forward compatibility with a
/// future UCSI v3.0 implementation.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ResponseData {
    /// Bitmap of supported alternate modes
    pub alt_modes_raw: [u8; RESPONSE_DATA_LEN],
}

impl ResponseData {
    /// Tries to get whether the alternate mode at the given index is supported
    ///
    /// Returns `None` if the index is out of bounds, otherwise returns `Some(true)` if supported or `Some(false)` if not supported.
    pub fn try_get_supported(&self, index: usize) -> Option<bool> {
        if index >= MAX_ALT_MODES {
            return None;
        }
        let byte_index = index / 8;
        let bit_index = index % 8;
        Some((self.alt_modes_raw.get(byte_index).copied().unwrap_or(0) & (1 << bit_index)) != 0)
    }

    /// Tries to set whether the alternate mode at the given index is supported
    ///
    /// Returns `None` if the index is out of bounds, otherwise returns `Some(())`.
    pub fn try_set_supported(&mut self, index: usize, supported: bool) -> Option<()> {
        if index >= MAX_ALT_MODES {
            return None;
        }
        let byte_index = index / 8;
        let bit_index = index % 8;
        if supported {
            if let Some(byte) = self.alt_modes_raw.get_mut(byte_index) {
                *byte |= 1 << bit_index;
            }
        } else {
            if let Some(byte) = self.alt_modes_raw.get_mut(byte_index) {
                *byte &= !(1 << bit_index);
            }
        }
        Some(())
    }

    /// Gets whether the alternate mode at the given index is supported
    ///
    /// Returns `false` if the index is out of bounds.
    pub fn get_supported(&self, index: usize) -> bool {
        self.try_get_supported(index).unwrap_or(false)
    }

    /// Sets whether the alternate mode at the given index is supported
    ///
    /// Does nothing if the index is out of bounds.
    pub fn set_supported(&mut self, index: usize, supported: bool) {
        let _ = self.try_set_supported(index, supported);
    }
}

/// Raw wire format of [`ResponseData`]
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Zeroable, Pod)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ResponseDataRaw {
    /// Bitmap of supported alternate modes
    pub alt_modes: [u8; RESPONSE_DATA_LEN],
}

impl ResponseDataRaw {
    /// Length of the raw response data in bytes
    pub const LEN: usize = size_of::<Self>();
}

impl From<ResponseDataRaw> for ResponseData {
    fn from(raw: ResponseDataRaw) -> Self {
        let mut data = ResponseData::default();
        data.alt_modes_raw.copy_from_slice(&raw.alt_modes);
        data
    }
}

impl From<ResponseData> for ResponseDataRaw {
    fn from(data: ResponseData) -> Self {
        Self {
            alt_modes: data.alt_modes_raw,
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
        let mut bytes = [0u8; RESPONSE_DATA_LEN];
        bytes[0] = 0x42;
        bytes[1] = 0x85;
        let raw = ResponseDataRaw { alt_modes: bytes };

        let data = ResponseData::from(raw);
        assert_eq!(data.alt_modes_raw, bytes);

        // Test accessors
        assert!(!data.get_supported(0));
        assert!(data.get_supported(1));
        assert!(!data.get_supported(2));
        assert!(!data.get_supported(3));
        assert!(!data.get_supported(4));
        assert!(!data.get_supported(5));
        assert!(data.get_supported(6));
        assert!(!data.get_supported(7));

        assert!(data.get_supported(8));
        assert!(!data.get_supported(9));
        assert!(data.get_supported(10));
        assert!(!data.get_supported(11));
        assert!(!data.get_supported(12));
        assert!(!data.get_supported(13));
        assert!(!data.get_supported(14));
        assert!(data.get_supported(15));

        for i in 16..128 {
            assert!(!data.get_supported(i));
        }

        // Check past end
        assert!(data.try_get_supported(128).is_none());

        let encoded: [u8; RESPONSE_DATA_LEN] = bytemuck::must_cast(ResponseDataRaw::from(data));
        assert_eq!(encoded, bytes);
    }
}

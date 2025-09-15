//! Types for GET_CAM_SUPPORTED command, see UCSI spec 6.5.12
use bincode::de::Decoder;
use bincode::enc::Encoder;
use bincode::error::{DecodeError, EncodeError};
use bincode::{Decode, Encode};

use crate::ucsi::{CommandHeaderRaw, COMMAND_LEN};

/// Data length for the GET_CAM_SUPPORTED command response
pub const RESPONSE_DATA_LEN: usize = 1;
/// Command padding
// -1 for the connector number byte
pub const COMMAND_PADDING: usize = COMMAND_LEN - size_of::<CommandHeaderRaw>() - 1;
/// Maximum number of alternate modes supported
pub const MAX_ALT_MODES: usize = 8;

/// Command arguments
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Args;

impl Encode for Args {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        // Padding to fill the command length
        [0u8; COMMAND_PADDING].encode(encoder)
    }
}

impl<Context> Decode<Context> for Args {
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, DecodeError> {
        // Read padding
        let _padding: [u8; COMMAND_PADDING] = Decode::decode(decoder)?;
        Ok(Self)
    }
}

/// GET_CAM_SUPPORTED response data, supports up to [`MAX_ALT_MODES`] alternate modes
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ResponseData {
    alt_modes: u8,
}

impl ResponseData {
    pub fn alt_mode_supported(&self, index: usize) -> bool {
        if index < MAX_ALT_MODES {
            (self.alt_modes & (1 << index)) != 0
        } else {
            false
        }
    }

    pub fn set_alt_mode_supported(&mut self, index: usize, supported: bool) {
        if index < MAX_ALT_MODES {
            if supported {
                self.alt_modes |= 1 << index;
            } else {
                self.alt_modes &= !(1 << index);
            }
        }
    }
}

impl Encode for ResponseData {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.alt_modes.encode(encoder)
    }
}

impl<Context> Decode<Context> for ResponseData {
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, DecodeError> {
        u8::decode(decoder).map(|v| ResponseData { alt_modes: v })
    }
}

#[cfg(test)]
mod test {
    use bincode::config::standard;
    use bincode::decode_from_slice;

    use super::*;

    #[test]
    fn test_encode_response_data() {
        let bytes = [0x12; RESPONSE_DATA_LEN];
        let expected = ResponseData { alt_modes: 0x12 };
        let (data, len): (ResponseData, _) =
            decode_from_slice(&bytes, standard().with_fixed_int_encoding()).expect("Decoding failed");
        assert_eq!(data, expected);
        assert_eq!(len, RESPONSE_DATA_LEN);
    }
}

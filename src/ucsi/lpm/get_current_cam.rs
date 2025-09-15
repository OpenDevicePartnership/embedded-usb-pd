//! Types for GET_CURRENT_CAM command, see UCSI spec 6.5.13

use bincode::de::Decoder;
use bincode::enc::Encoder;
use bincode::error::{DecodeError, EncodeError};
use bincode::{Decode, Encode};

use crate::ucsi::{CommandHeaderRaw, COMMAND_LEN};

/// Data length for the GET_CAM_SUPPORTED command response
/// This matches the mailbox size
pub const RESPONSE_DATA_LEN: usize = 16;
/// Command padding
// -1 for the connector number byte
pub const COMMAND_PADDING: usize = COMMAND_LEN - size_of::<CommandHeaderRaw>() - 1;

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

/// GET_CURRENT_CAM response data, supports up to [`RESPONSE_DATA_LEN`] alternate modes
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ResponseData {
    pub alt_modes: [u8; RESPONSE_DATA_LEN],
}

impl Encode for ResponseData {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.alt_modes.encode(encoder)
    }
}

impl<Context> Decode<Context> for ResponseData {
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, DecodeError> {
        <[u8; RESPONSE_DATA_LEN]>::decode(decoder).map(|v| ResponseData { alt_modes: v })
    }
}

#[cfg(test)]
mod test {
    use bincode::config::standard;
    use bincode::decode_from_slice;

    use super::*;

    #[test]
    fn test_encode_response_data() {
        let bytes = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
        ];
        let expected = ResponseData { alt_modes: bytes };
        let (data, len): (ResponseData, _) =
            decode_from_slice(&bytes, standard().with_fixed_int_encoding()).expect("Decoding failed");
        assert_eq!(data, expected);
        assert_eq!(len, RESPONSE_DATA_LEN);
    }
}

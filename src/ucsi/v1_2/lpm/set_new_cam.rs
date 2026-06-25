//! Types for SET_NEW_CAM command, see UCSI spec 6.5.14

use bincode::de::Decoder;
use bincode::enc::Encoder;
use bincode::error::{DecodeError, EncodeError};
use bincode::{Decode, Encode};

use crate::ucsi::v1_2::lpm::ConnectorNumberRaw;

/// Command data length
pub const COMMAND_DATA_LEN: usize = 6;

/// Command arguments
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Args {
    /// Connector number
    pub connector_number: u8,
    /// Enter or exit mode
    pub enter: bool,
    /// Alternate mode offset
    pub am_offset: u8,
    /// Alternate mode specific
    pub am_specific: u32,
}

impl Encode for Args {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        let mut connector_number = ConnectorNumberRaw::default();
        connector_number.set_connector_number(self.connector_number);
        connector_number.set_high_bit(self.enter);
        connector_number.encode(encoder)?;
        self.am_offset.encode(encoder)?;
        self.am_specific.encode(encoder)?;
        Ok(())
    }
}

impl<Context> Decode<Context> for Args {
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, DecodeError> {
        let connector_number = ConnectorNumberRaw(u8::decode(decoder)?);
        let am_offset = u8::decode(decoder)?;
        let am_specific = u32::decode(decoder)?;
        Ok(Args {
            connector_number: connector_number.connector_number(),
            enter: connector_number.high_bit(),
            am_offset,
            am_specific,
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
        // Enter alt mode at offset 1, on connector3, with AM-specific 0x12345678
        let encoded: [u8; 6] = [0x83, 0x01, 0x78, 0x56, 0x34, 0x12];
        let (decoded, size): (Args, usize) = decode_from_slice(&encoded, standard().with_fixed_int_encoding()).unwrap();
        assert_eq!(size, 6);

        let expected = Args {
            connector_number: 3,
            enter: true,
            am_offset: 1,
            am_specific: 0x12345678,
        };
        assert_eq!(decoded, expected);
    }
}

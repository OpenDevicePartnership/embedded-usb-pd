//! Types for SET_UOR command, see UCSI spec 6.5.9

use bincode::de::Decoder;
use bincode::enc::Encoder;
use bincode::error::{DecodeError, EncodeError};
use bincode::{Decode, Encode};
use bitfield::bitfield;

use crate::ucsi::{CommandHeaderRaw, COMMAND_LEN};

/// Command padding
pub const COMMAND_PADDING: usize = COMMAND_LEN - size_of::<CommandHeaderRaw>() - size_of::<ArgsRaw>();

bitfield! {
    /// Raw arguments
    #[derive(Copy, Clone, Default, PartialEq, Eq)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub(super) struct ArgsRaw(u16);
    impl Debug;

    /// Connector number
    pub u8, connector_number, set_connector_number: 6, 0;
    /// DFP
    pub bool, dfp, set_dfp: 7;
    /// UFP
    pub bool, ufp, set_ufp: 8;
    /// Accept data-role swap
    pub bool, accept_swap, set_accept_swap: 9;
}

/// Command arguments
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Args(ArgsRaw);

impl Args {
    pub fn connector_number(&self) -> u8 {
        self.0.connector_number()
    }

    pub fn set_connector_number(&mut self, connector_number: u8) -> &mut Self {
        self.0.set_connector_number(connector_number);
        self
    }

    pub fn dfp(&self) -> bool {
        self.0.dfp()
    }

    pub fn set_dfp(&mut self, dfp: bool) -> &mut Self {
        self.0.set_dfp(dfp);
        self
    }

    pub fn ufp(&self) -> bool {
        self.0.ufp()
    }

    pub fn set_ufp(&mut self, ufp: bool) -> &mut Self {
        self.0.set_ufp(ufp);
        self
    }

    pub fn accept_swap(&self) -> bool {
        self.0.accept_swap()
    }

    pub fn set_accept_swap(&mut self, accept_swap: bool) -> &mut Self {
        self.0.set_accept_swap(accept_swap);
        self
    }
}

impl From<u16> for Args {
    fn from(value: u16) -> Self {
        Self(ArgsRaw(value))
    }
}

impl From<Args> for u16 {
    fn from(args: Args) -> Self {
        args.0 .0
    }
}

impl Encode for Args {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        Encode::encode(&self.0 .0, encoder)?;
        // Padding to fill the command length
        [0u8; COMMAND_PADDING].encode(encoder)
    }
}

impl<Context> Decode<Context> for Args {
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, DecodeError> {
        let raw = u16::decode(decoder)?;
        // Read padding
        let _padding: [u8; COMMAND_PADDING] = Decode::decode(decoder)?;
        Ok(Args::from(raw))
    }
}

#[cfg(test)]
mod test {
    use bincode::config::standard;
    use bincode::decode_from_slice;

    use super::*;

    #[test]
    fn test_decode_args() {
        // DFP/accept swap on connector 3
        let encoded: [u8; 6] = [0x83, 0x02, 0x00, 0x00, 0x00, 0x00];
        let (decoded, size): (Args, usize) = decode_from_slice(&encoded, standard().with_fixed_int_encoding()).unwrap();
        assert_eq!(size, 6);

        let expected = *Args::default()
            .set_connector_number(3)
            .set_accept_swap(true)
            .set_dfp(true);
        assert_eq!(decoded, expected);
    }
}

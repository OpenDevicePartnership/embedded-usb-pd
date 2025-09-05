//! Types for SET_PDR command, see UCSI spec 6.5.10

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
    /// Swap to source
    pub bool, swap_source, set_swap_source: 7;
    /// Swap to sink
    pub bool, swap_sink, set_swap_sink: 8;
    /// Accept power-role swap
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

    pub fn swap_source(&self) -> bool {
        self.0.swap_source()
    }

    pub fn set_swap_source(&mut self, swap_source: bool) -> &mut Self {
        self.0.set_swap_source(swap_source);
        self
    }

    pub fn swap_sink(&self) -> bool {
        self.0.swap_sink()
    }

    pub fn set_swap_sink(&mut self, swap_sink: bool) -> &mut Self {
        self.0.set_swap_sink(swap_sink);
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
        // Source swap/accept swap on connector 3
        let encoded: [u8; 6] = [0x83, 0x02, 0x00, 0x00, 0x00, 0x00];
        let (decoded, size): (Args, usize) = decode_from_slice(&encoded, standard().with_fixed_int_encoding()).unwrap();
        assert_eq!(size, 6);

        let expected = *Args::default()
            .set_connector_number(3)
            .set_accept_swap(true)
            .set_swap_source(true);
        assert_eq!(decoded, expected);
    }
}

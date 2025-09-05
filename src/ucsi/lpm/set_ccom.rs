//! Types for SET_CCOM command, see UCSI spec 6.5.8

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
    /// Rp
    pub bool, rp, set_rp: 7;
    /// Rd
    pub bool, rd, set_rd: 8;
    /// Drp
    pub bool, drp, set_drp: 9;
    /// Disabled
    pub bool, disabled, set_disabled: 10;
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

    pub fn rp(&self) -> bool {
        self.0.rp()
    }

    pub fn set_rp(&mut self, rp: bool) -> &mut Self {
        self.0.set_rp(rp);
        self
    }

    pub fn rd(&self) -> bool {
        self.0.rd()
    }

    pub fn set_rd(&mut self, rd: bool) -> &mut Self {
        self.0.set_rd(rd);
        self
    }

    pub fn drp(&self) -> bool {
        self.0.drp()
    }

    pub fn set_drp(&mut self, drp: bool) -> &mut Self {
        self.0.set_drp(drp);
        self
    }

    pub fn disabled(&self) -> bool {
        self.0.disabled()
    }

    pub fn set_disabled(&mut self, disabled: bool) -> &mut Self {
        self.0.set_disabled(disabled);
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
        // Rp/DRP on connector 3
        let encoded: [u8; 6] = [0x83, 0x02, 0x00, 0x00, 0x00, 0x00];
        let (decoded, size): (Args, usize) = decode_from_slice(&encoded, standard().with_fixed_int_encoding()).unwrap();
        assert_eq!(size, 6);

        let expected = *Args::default().set_connector_number(3).set_drp(true).set_rp(true);
        assert_eq!(decoded, expected);
    }
}

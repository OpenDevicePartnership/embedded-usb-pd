//! Types for GET_ALTERNATE_MODES command, see UCSI spec 6.5.11

use bincode::de::Decoder;
use bincode::enc::Encoder;
use bincode::error::{DecodeError, EncodeError};
use bincode::{Decode, Encode};
use bitfield::bitfield;

use super::Recipient;
use crate::ucsi::lpm::InvalidRecipient;
use crate::ucsi::{CommandHeaderRaw, COMMAND_LEN};
use crate::vdm::{AltModeId, Svid};

/// Data length for the GET_ALTERNATE_MODES command response
pub const RESPONSE_DATA_LEN: usize = 12;
/// Command padding
pub const COMMAND_PADDING: usize = COMMAND_LEN - size_of::<CommandHeaderRaw>() - size_of::<ArgsRaw>();

bitfield! {
    /// Raw arguments
    #[derive(Copy, Clone, PartialEq, Eq)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub(super) struct ArgsRaw(u32);
    impl Debug;

    /// Recipient
    pub u8, recipient, set_recipient: 2, 0;
    /// Connector number
    pub u8, connector_number, set_connector_number: 14, 8;
    /// Alternate mode offset
    pub u8, mode_offset, set_mode_offset: 23, 16;
    /// Number of alternate modes
    pub u8, num_modes, set_num_modes: 25, 24;
}

/// Command arguments
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Args(ArgsRaw);

impl Args {
    pub fn recipient(&self) -> Recipient {
        let recipient: Result<Recipient, _> = self.0.recipient().try_into();
        // Won't panic, validated in try_from
        recipient.unwrap()
    }

    pub fn set_recipient(&mut self, recipient: Recipient) -> &mut Self {
        self.0.set_recipient(recipient.into());
        self
    }

    pub fn connector_number(&self) -> u8 {
        self.0.connector_number()
    }

    pub fn set_connector_number(&mut self, number: u8) -> &mut Self {
        self.0.set_connector_number(number);
        self
    }

    pub fn mode_offset(&self) -> u8 {
        self.0.mode_offset()
    }

    pub fn set_mode_offset(&mut self, offset: u8) -> &mut Self {
        self.0.set_mode_offset(offset);
        self
    }

    pub fn num_modes(&self) -> u8 {
        self.0.num_modes()
    }

    pub fn set_num_modes(&mut self, num: u8) -> &mut Self {
        self.0.set_num_modes(num);
        self
    }
}

impl TryFrom<u32> for Args {
    type Error = InvalidRecipient;

    fn try_from(raw: u32) -> Result<Self, Self::Error> {
        let raw = ArgsRaw(raw);
        let _recipient: Recipient = (raw.recipient()).try_into()?;

        Ok(Args(raw))
    }
}

impl From<Args> for u32 {
    fn from(args: Args) -> Self {
        args.0 .0
    }
}

impl Default for Args {
    fn default() -> Self {
        Args(ArgsRaw(0))
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
        let raw = u32::decode(decoder)?;
        // Read padding
        let _padding: [u8; COMMAND_PADDING] = Decode::decode(decoder)?;
        Args::try_from(raw).map_err(|invalid_recipient| DecodeError::UnexpectedVariant {
            type_name: "Recipient",
            allowed: &bincode::error::AllowedEnumVariants::Allowed(&[
                Recipient::Connector as u32,
                Recipient::Sop as u32,
                Recipient::SopP as u32,
                Recipient::SopPp as u32,
            ]),
            found: invalid_recipient.0 as u32,
        })
    }
}

/// Representation of a single alt-mode
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct AltMode {
    pub svid: Svid,
    pub mid: AltModeId,
}

/// Length of the alternate modes array
pub const ALT_MODES_LEN: usize = 2;

/// GET_ALTERNATE_MODES response data
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ResponseData {
    pub alt_modes: [AltMode; ALT_MODES_LEN],
}

impl Encode for ResponseData {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        for alt_mode in &self.alt_modes {
            alt_mode.svid.0.encode(encoder)?;
            alt_mode.mid.0.encode(encoder)?;
        }
        Ok(())
    }
}

impl<Context> Decode<Context> for ResponseData {
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, DecodeError> {
        let mut alt_modes = [AltMode::default(); ALT_MODES_LEN];
        for alt_mode in &mut alt_modes {
            alt_mode.svid = Svid(u16::decode(decoder)?);
            alt_mode.mid = AltModeId(u32::decode(decoder)?);
        }
        Ok(ResponseData { alt_modes })
    }
}

#[cfg(test)]
mod test {
    use bincode::config::standard;
    use bincode::decode_from_slice;

    use super::*;

    #[test]
    fn test_decode_args() {
        // SOP on connector 3, 2 requested alt modes
        let encoded: [u8; 6] = [0x00, 0x01, 0x0, 0x1, 0x0, 0x0];
        let (decoded, size): (Args, usize) = decode_from_slice(&encoded, standard().with_fixed_int_encoding()).unwrap();
        assert_eq!(size, 6);

        let mut expected = Args::default();
        expected.set_connector_number(1);
        expected.set_num_modes(1);
        expected.set_recipient(Recipient::Connector);
        expected.set_mode_offset(0);
        assert_eq!(decoded, expected);
    }

    #[test]
    fn test_decode_response_data() {
        // No particular meaning to these values
        let encoded: [u8; RESPONSE_DATA_LEN] = [0x34, 0x12, 0x78, 0x56, 0x34, 0x12, 0x12, 0x34, 0x12, 0x34, 0x56, 0x78];
        let (decoded, size): (ResponseData, usize) =
            decode_from_slice(&encoded, standard().with_fixed_int_encoding()).unwrap();
        assert_eq!(size, RESPONSE_DATA_LEN);

        let mut expected = ResponseData::default();
        expected.alt_modes[0].svid = Svid(0x1234);
        expected.alt_modes[0].mid = AltModeId(0x12345678);
        expected.alt_modes[1].svid = Svid(0x3412);
        expected.alt_modes[1].mid = AltModeId(0x78563412);
        assert_eq!(decoded, expected);
    }
}

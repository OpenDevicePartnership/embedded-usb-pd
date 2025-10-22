//! Types for the ACK_CC_CI command
use bincode::de::{Decode, Decoder};
use bincode::enc::{Encode, Encoder};
use bincode::error::{DecodeError, EncodeError};
use bitfield::bitfield;

use crate::ucsi::{CommandHeaderRaw, COMMAND_LEN};

bitfield! {
    /// Raw ack flags, see UCSI spec 6.5.4 for details
    #[derive(Copy, Clone, PartialEq, Eq)]
    struct AckRaw(u8);
    impl Debug;

    /// Ack connector change
    pub bool, connector_change, set_connector_change: 0;
    /// Ack command complete
    pub bool, command_complete, set_command_complete: 1;
}

#[cfg(feature = "defmt")]
impl defmt::Format for AckRaw {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "AckRaw {{ .0: {}, connector_change: {}, command_complete: {} }}",
            self.0,
            self.connector_change(),
            self.command_complete()
        )
    }
}

/// Higher-level wrapper around [`AckRaw`]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Ack(AckRaw);

impl Ack {
    /// Returns connector change ack status
    pub fn connector_change(&self) -> bool {
        self.0.connector_change()
    }

    /// Set connector change ack status
    pub fn set_connector_change(&mut self, ack: bool) -> &mut Self {
        self.0.set_connector_change(ack);
        self
    }

    /// Returns command complete ack status
    pub fn command_complete(&self) -> bool {
        self.0.command_complete()
    }

    /// Set command complete ack status
    pub fn set_command_complete(&mut self, ack: bool) -> &mut Self {
        self.0.set_command_complete(ack);
        self
    }
}

impl From<u8> for Ack {
    fn from(raw: u8) -> Self {
        Self(AckRaw(raw))
    }
}

impl Default for Ack {
    fn default() -> Self {
        Self(AckRaw(0))
    }
}

impl Encode for Ack {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        Encode::encode(&self.0 .0, encoder)
    }
}

impl<Context> Decode<Context> for Ack {
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, DecodeError> {
        let raw = u8::decode(decoder)?;
        Ok(Self::from(raw))
    }
}

/// ACK_CC_CI command structure
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Args {
    /// Ack flags
    pub ack: Ack,
}

/// Data length for the ACK_CC_CI command response
pub const RESPONSE_DATA_LEN: u8 = 0;
/// Command padding
pub const COMMAND_PADDING: usize = COMMAND_LEN - size_of::<CommandHeaderRaw>() - size_of::<Ack>();

impl Encode for Args {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.ack.encode(encoder)?;
        // Padding to fill the command length
        [0u8; COMMAND_PADDING].encode(encoder)
    }
}

impl<Context> Decode<Context> for Args {
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, DecodeError> {
        let ack = Ack::decode(decoder)?;
        // Read padding
        let _padding: [u8; COMMAND_PADDING] = Decode::decode(decoder)?;
        Ok(Self { ack })
    }
}

//! Types for GET_PD_MESSAGE command, see UCSI spec 4.5.20

use bincode::de::Decoder;
use bincode::enc::Encoder;
use bincode::error::{DecodeError, EncodeError};
use bincode::{Decode, Encode};
use bitfield::bitfield;

use crate::ucsi::v1_2::lpm::{InvalidRecipient, Recipient};
use crate::ucsi::v1_2::{CommandHeaderRaw, COMMAND_LEN};

/// Data length for the GET_PD_MESSAGE command response
pub const RESPONSE_DATA_LEN: usize = 16;
/// Command padding
pub const COMMAND_PADDING: usize = COMMAND_LEN - size_of::<CommandHeaderRaw>() - size_of::<ArgsRaw>();

bitfield! {
    /// Raw arguments
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub(super) struct ArgsRaw(u32);
    impl Debug;

    /// Connector number
    pub u8, connector_number, set_connector_number: 6, 0;
    /// Recipient
    pub u8, recipient, set_recipient: 9, 7;
    /// Message offset
    pub u8, message_offset, set_message_offset: 15, 10;
    /// Number of bytes
    pub u8, num_bytes, set_num_bytes: 23, 16;
    /// Message type
    pub u8, message_type, set_message_type: 29, 24;
}

#[cfg(feature = "defmt")]
impl defmt::Format for ArgsRaw {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "ArgsRaw {{ .0: {}, recipient: {}, connector_number: {}, message_offset: {}, num_bytes: {}, message_type: {} }}",
            self.0,
            self.recipient(),
            self.connector_number(),
            self.message_offset(),
            self.num_bytes(),
            self.message_type()
        )
    }
}

/// Response Message Type enum
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MessageType {
    /// Extended sink capabilities
    SinkCapExtended,
    /// Extended source capabilities
    SourceCapExtended,
    /// Battery capabilities
    BatteryCap,
    /// Battery Status
    BatteryStatus,
    /// Discover identity
    DiscoverIdentity,
}

/// Invalid response message type error, contains the invalid response message type value
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct InvalidMessageType(pub u8);

impl TryFrom<u8> for MessageType {
    type Error = InvalidMessageType;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(MessageType::SinkCapExtended),
            0x01 => Ok(MessageType::SourceCapExtended),
            0x02 => Ok(MessageType::BatteryCap),
            0x03 => Ok(MessageType::BatteryStatus),
            0x04 => Ok(MessageType::DiscoverIdentity),
            _ => Err(InvalidMessageType(value)),
        }
    }
}

impl From<MessageType> for u8 {
    fn from(msg_type: MessageType) -> Self {
        match msg_type {
            MessageType::SinkCapExtended => 0x00,
            MessageType::SourceCapExtended => 0x01,
            MessageType::BatteryCap => 0x02,
            MessageType::BatteryStatus => 0x03,
            MessageType::DiscoverIdentity => 0x04,
        }
    }
}

/// Command arguments
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Args(ArgsRaw);

impl Args {
    pub fn recipient(&self) -> Recipient {
        // Panic Safety: ArgsRaw::recipient is guaranteed to be a valid and defined value of Recipient:
        // 1. Args::set_recipient only accepts Recipient values
        // 2. ArgsRaw::set_recipient is only set with values from u8::from(Recipient)
        // 3. Recipient::try_from(u8) only fails for undefined values and is unit tested with all defined values to roundtrip correctly
        // 4. The only way to construct an Args is through Args::try_from(u32), which validates Recipient::try_from(u8)
        #[allow(clippy::unwrap_used)]
        self.0.recipient().try_into().unwrap()
    }

    // NOTE: Self::recipient has a SAFETY requirement on argument being `Recipient` and only setting with values
    // returned from `impl From<Recipient> for u8`
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

    pub fn message_offset(&self) -> u8 {
        self.0.message_offset()
    }

    pub fn set_message_offset(&mut self, offset: u8) -> &mut Self {
        self.0.set_message_offset(offset);
        self
    }

    pub fn num_bytes(&self) -> u8 {
        self.0.num_bytes()
    }

    pub fn set_num_bytes(&mut self, num: u8) -> &mut Self {
        self.0.set_num_bytes(num);
        self
    }

    pub fn message_type(&self) -> MessageType {
        // Panic Safety: ArgsRaw::message_type is guaranteed to be a valid and defined value of MessageType:
        // 1. Args::set_message_type only accepts MessageType values
        // 2. ArgsRaw::set_message_type is only set with values from u8::from(MessageType)
        // 3. MessageType::try_from(u8) only fails for undefined values and is unit tested with all defined values to roundtrip correctly
        // 4. The only way to construct an Args is through Args::try_from(u32), which validates MessageType::try_from(u8)
        #[allow(clippy::unwrap_used)]
        self.0.message_type().try_into().unwrap()
    }

    pub fn set_message_type(&mut self, message_type: MessageType) -> &mut Self {
        self.0.set_message_type(message_type.into());
        self
    }
}

/// Invalid args error
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum InvalidArgs {
    /// Invalid recipient value
    InvalidRecipient(InvalidRecipient),
    /// Invalid message type value
    InvalidMessageType(InvalidMessageType),
}

impl TryFrom<u32> for Args {
    type Error = InvalidArgs;

    fn try_from(raw: u32) -> Result<Self, Self::Error> {
        // note: safety requirements must be upheld by validating enum fields are valid defined values
        let raw = ArgsRaw(raw);
        let _recipient: Recipient = raw.recipient().try_into().map_err(InvalidArgs::InvalidRecipient)?;
        let _message_type: MessageType = raw.message_type().try_into().map_err(InvalidArgs::InvalidMessageType)?;

        // all fields are valid
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
        Args::try_from(raw).map_err(|err| match err {
            InvalidArgs::InvalidRecipient(invalid_recipient) => DecodeError::UnexpectedVariant {
                type_name: "Recipient",
                allowed: &bincode::error::AllowedEnumVariants::Allowed(&[
                    Recipient::Connector as u32,
                    Recipient::Sop as u32,
                    Recipient::SopP as u32,
                    Recipient::SopPp as u32,
                ]),
                found: invalid_recipient.0 as u32,
            },
            InvalidArgs::InvalidMessageType(invalid_message_type) => DecodeError::UnexpectedVariant {
                type_name: "MessageType",
                allowed: &bincode::error::AllowedEnumVariants::Allowed(&[
                    MessageType::SinkCapExtended as u32,
                    MessageType::SourceCapExtended as u32,
                    MessageType::BatteryCap as u32,
                    MessageType::BatteryStatus as u32,
                    MessageType::DiscoverIdentity as u32,
                ]),
                found: invalid_message_type.0 as u32,
            },
        })
    }
}

/// GET_PD_MESSAGE response data
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ResponseData {
    /// Returned bytes
    pub bytes: [u8; RESPONSE_DATA_LEN],
}

impl Encode for ResponseData {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.bytes.encode(encoder)
    }
}

impl<Context> Decode<Context> for ResponseData {
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, DecodeError> {
        Ok(ResponseData {
            bytes: Decode::decode(decoder)?,
        })
    }
}

#[cfg(test)]
mod test {
    use bincode::config::standard;
    use bincode::decode_from_slice;

    use super::*;

    #[test]
    fn test_message_type_try_from() {
        assert_eq!(MessageType::try_from(0x00), Ok(MessageType::SinkCapExtended));
        assert_eq!(MessageType::try_from(0x01), Ok(MessageType::SourceCapExtended));
        assert_eq!(MessageType::try_from(0x02), Ok(MessageType::BatteryCap));
        assert_eq!(MessageType::try_from(0x03), Ok(MessageType::BatteryStatus));
        assert_eq!(MessageType::try_from(0x04), Ok(MessageType::DiscoverIdentity));
        for i in 0x05..=0xFF {
            assert_eq!(MessageType::try_from(i), Err(InvalidMessageType(i)));
        }
    }

    #[test]
    fn test_message_type_into_u8() {
        let as_u8: u8 = MessageType::SinkCapExtended.into();
        assert_eq!(as_u8, 0x00);
        let as_u8: u8 = MessageType::SourceCapExtended.into();
        assert_eq!(as_u8, 0x01);
        let as_u8: u8 = MessageType::BatteryCap.into();
        assert_eq!(as_u8, 0x02);
        let as_u8: u8 = MessageType::BatteryStatus.into();
        assert_eq!(as_u8, 0x03);
        let as_u8: u8 = MessageType::DiscoverIdentity.into();
        assert_eq!(as_u8, 0x04);
    }

    #[test]
    fn test_decode_args() {
        // SOP on connector 3, message offset 2, 1 byte, battery cap message type
        let encoded: [u8; 6] = [0x83, 0x08, 0x01, 0x02, 0x00, 0x00];
        let (decoded, size): (Args, usize) = decode_from_slice(&encoded, standard().with_fixed_int_encoding()).unwrap();
        assert_eq!(size, 6);

        let mut expected = Args::default();
        expected.set_connector_number(3);
        expected.set_recipient(Recipient::Sop);
        expected.set_message_offset(2);
        expected.set_num_bytes(1);
        expected.set_message_type(MessageType::BatteryCap);
        assert_eq!(decoded, expected);
    }

    #[test]
    fn test_decode_args_invalid_recipient() {
        // Invalid recipient on connector 3, message offset 2, 1 byte, battery cap message type
        let encoded: [u8; 6] = [0x83, 0x0B, 0x01, 0x02, 0x00, 0x00];
        let Err(bincode::error::DecodeError::UnexpectedVariant {
            type_name,
            allowed,
            found,
        }): Result<(Args, usize), _> = decode_from_slice(&encoded, standard().with_fixed_int_encoding())
        else {
            panic!("Expected UnexpectedVariant error");
        };
        assert_eq!(type_name, "Recipient");
        assert_eq!(
            *allowed,
            bincode::error::AllowedEnumVariants::Allowed(&[
                Recipient::Connector as u32,
                Recipient::Sop as u32,
                Recipient::SopP as u32,
                Recipient::SopPp as u32,
            ])
        );
        assert_eq!(found, 0x07);
    }

    #[test]
    fn test_decode_args_invalid_message_type() {
        // Invalid message type on connector 3, message offset 2, 1 byte, battery cap message type
        let encoded: [u8; 6] = [0x83, 0x38, 0x01, 0x0f, 0x00, 0x00];
        let Err(bincode::error::DecodeError::UnexpectedVariant {
            type_name,
            allowed,
            found,
        }): Result<(Args, usize), _> = decode_from_slice(&encoded, standard().with_fixed_int_encoding())
        else {
            panic!("Expected UnexpectedVariant error");
        };
        assert_eq!(type_name, "MessageType");
        assert_eq!(
            *allowed,
            bincode::error::AllowedEnumVariants::Allowed(&[
                MessageType::SinkCapExtended as u32,
                MessageType::SourceCapExtended as u32,
                MessageType::BatteryCap as u32,
                MessageType::BatteryStatus as u32,
                MessageType::DiscoverIdentity as u32,
            ])
        );
        assert_eq!(found, 0x0f);
    }

    #[test]
    fn test_decode_response_data() {
        // No particular meaning to these values
        let encoded: [u8; RESPONSE_DATA_LEN] = [
            0x34, 0x12, 0x78, 0x56, 0x34, 0x12, 0x12, 0x34, 0x12, 0x34, 0x56, 0x78, 0xAB, 0xCD, 0xEF, 0x12,
        ];
        let (decoded, size): (ResponseData, usize) =
            decode_from_slice(&encoded, standard().with_fixed_int_encoding()).unwrap();
        assert_eq!(size, RESPONSE_DATA_LEN);

        let mut expected = ResponseData::default();
        expected.bytes[0] = 0x34;
        expected.bytes[1] = 0x12;
        expected.bytes[2] = 0x78;
        expected.bytes[3] = 0x56;
        expected.bytes[4] = 0x34;
        expected.bytes[5] = 0x12;
        expected.bytes[6] = 0x12;
        expected.bytes[7] = 0x34;
        expected.bytes[8] = 0x12;
        expected.bytes[9] = 0x34;
        expected.bytes[10] = 0x56;
        expected.bytes[11] = 0x78;
        expected.bytes[12] = 0xAB;
        expected.bytes[13] = 0xCD;
        expected.bytes[14] = 0xEF;
        expected.bytes[15] = 0x12;
        assert_eq!(decoded, expected);
    }
}

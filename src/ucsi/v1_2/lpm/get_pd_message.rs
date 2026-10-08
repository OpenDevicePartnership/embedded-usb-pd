//! Types for GET_PD_MESSAGE command, see UCSI spec 4.5.20

use bitfield::bitfield;
use bytemuck::{Pod, Zeroable};
use pack1::U32LE;

use crate::ucsi::v1_2::lpm::{InvalidRecipient, Recipient};
use crate::ucsi::v1_2::{CommandHeaderRaw, COMMAND_LEN};
use crate::PortId;

/// Data length for the GET_PD_MESSAGE command response
pub const RESPONSE_DATA_LEN: usize = 16;
/// Command padding
pub const COMMAND_PADDING: usize = COMMAND_LEN - size_of::<CommandHeaderRaw>() - size_of::<ArgBitsRaw>();

bitfield! {
    /// Raw argument bits
    #[derive(Copy, Clone, Default, PartialEq, Eq)]
    pub struct ArgBitsRaw(u32);
    impl Debug;

    /// Connector number
    pub u8, connector_number, set_connector_number: 6, 0;
    /// Recipient, crosses the boundary between the first and second bytes
    pub u8, recipient, set_recipient: 9, 7;
    /// Message offset
    pub u8, message_offset, set_message_offset: 15, 10;
    /// Number of bytes
    pub u8, num_bytes, set_num_bytes: 23, 16;
    /// Message type
    pub u8, message_type, set_message_type: 29, 24;
}

#[cfg(feature = "defmt")]
impl defmt::Format for ArgBitsRaw {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "ArgBitsRaw {{ .0: {}, recipient: {}, connector_number: {}, message_offset: {}, num_bytes: {}, message_type: {} }}",
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
pub struct Args<T: PortId> {
    /// Connector number
    pub connector_number: T,
    /// Recipient
    pub recipient: Recipient,
    /// Message offset
    pub message_offset: u8,
    /// Number of bytes
    pub num_bytes: u8,
    /// Message type
    pub message_type: MessageType,
}

impl<T: PortId> Default for Args<T> {
    fn default() -> Self {
        Self {
            connector_number: T::from(0),
            recipient: Recipient::Connector,
            message_offset: 0,
            num_bytes: 0,
            message_type: MessageType::SinkCapExtended,
        }
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

impl<T: PortId> TryFrom<ArgBitsRaw> for Args<T> {
    type Error = InvalidArgs;

    fn try_from(raw: ArgBitsRaw) -> Result<Self, Self::Error> {
        Ok(Self {
            connector_number: raw.connector_number().into(),
            recipient: raw.recipient().try_into().map_err(InvalidArgs::InvalidRecipient)?,
            message_offset: raw.message_offset(),
            num_bytes: raw.num_bytes(),
            message_type: raw.message_type().try_into().map_err(InvalidArgs::InvalidMessageType)?,
        })
    }
}

impl<T: PortId> From<Args<T>> for ArgBitsRaw {
    fn from(args: Args<T>) -> Self {
        let mut raw = ArgBitsRaw(0);
        raw.set_connector_number(args.connector_number.into());
        raw.set_recipient(args.recipient.into());
        raw.set_message_offset(args.message_offset);
        raw.set_num_bytes(args.num_bytes);
        raw.set_message_type(args.message_type.into());
        raw
    }
}

impl<T: PortId> TryFrom<u32> for Args<T> {
    type Error = InvalidArgs;

    fn try_from(raw: u32) -> Result<Self, Self::Error> {
        ArgBitsRaw(raw).try_into()
    }
}

impl<T: PortId> From<Args<T>> for u32 {
    fn from(args: Args<T>) -> Self {
        ArgBitsRaw::from(args).0
    }
}

/// Raw wire format of [`Args`]
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Zeroable, Pod)]
pub struct ArgsRaw {
    /// Argument bits, see [`ArgBitsRaw`]
    pub bits: U32LE,
    /// Reserved bytes, filling out the remainder of the command
    _reserved: [u8; COMMAND_PADDING],
}

impl ArgsRaw {
    /// Length of the raw arguments in bytes
    pub const LEN: usize = size_of::<Self>();
}

#[cfg(feature = "defmt")]
impl defmt::Format for ArgsRaw {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "ArgsRaw {{ bits: {} }}", ArgBitsRaw(self.bits.get()))
    }
}

impl<T: PortId> From<Args<T>> for ArgsRaw {
    fn from(args: Args<T>) -> Self {
        Self {
            bits: U32LE::new(args.into()),
            ..Default::default()
        }
    }
}

impl<T: PortId> TryFrom<ArgsRaw> for Args<T> {
    type Error = InvalidArgs;

    fn try_from(raw: ArgsRaw) -> Result<Self, Self::Error> {
        raw.bits.get().try_into()
    }
}

/// GET_PD_MESSAGE response data
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ResponseData {
    /// Returned bytes
    pub bytes: [u8; RESPONSE_DATA_LEN],
}

/// Raw wire format of [`ResponseData`]
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Zeroable, Pod)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ResponseDataRaw {
    /// Returned bytes
    pub bytes: [u8; RESPONSE_DATA_LEN],
}

impl ResponseDataRaw {
    /// Length of the raw response data in bytes
    pub const LEN: usize = size_of::<Self>();
}

impl From<ResponseData> for ResponseDataRaw {
    fn from(data: ResponseData) -> Self {
        Self { bytes: data.bytes }
    }
}

impl From<ResponseDataRaw> for ResponseData {
    fn from(raw: ResponseDataRaw) -> Self {
        Self { bytes: raw.bytes }
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
    fn test_args_raw_roundtrip() {
        // SOP on connector 3, message offset 2, 1 byte, battery cap message type
        let encoded: [u8; ArgsRaw::LEN] = [0x83, 0x08, 0x01, 0x02, 0x00, 0x00];
        let expected: Args<GlobalPortId> = Args {
            connector_number: GlobalPortId(3),
            recipient: Recipient::Sop,
            message_offset: 2,
            num_bytes: 1,
            message_type: MessageType::BatteryCap,
        };

        assert_eq!(Args::try_from(bytemuck::must_cast::<_, ArgsRaw>(encoded)), Ok(expected));
        let bytes: [u8; ArgsRaw::LEN] = bytemuck::must_cast(ArgsRaw::from(expected));
        assert_eq!(bytes, encoded);
    }

    #[test]
    fn test_args_raw_roundtrip_recipient_cross_byte() {
        // SOP'' (0b011) on connector 0x7f, recipient bit 0 is in byte 0 and bit 1 in byte 1
        let encoded: [u8; ArgsRaw::LEN] = [0xff, 0x01, 0x00, 0x04, 0x00, 0x00];
        let expected: Args<GlobalPortId> = Args {
            connector_number: GlobalPortId(0x7f),
            recipient: Recipient::SopPp,
            message_offset: 0,
            num_bytes: 0,
            message_type: MessageType::DiscoverIdentity,
        };

        assert_eq!(Args::try_from(bytemuck::must_cast::<_, ArgsRaw>(encoded)), Ok(expected));
        let bytes: [u8; ArgsRaw::LEN] = bytemuck::must_cast(ArgsRaw::from(expected));
        assert_eq!(bytes, encoded);
    }

    #[test]
    fn test_args_raw_invalid_recipient() {
        // Invalid recipient on connector 3, message offset 2, 1 byte, battery cap message type
        let encoded: [u8; ArgsRaw::LEN] = [0x83, 0x0B, 0x01, 0x02, 0x00, 0x00];
        assert_eq!(
            Args::<GlobalPortId>::try_from(bytemuck::must_cast::<_, ArgsRaw>(encoded)),
            Err(InvalidArgs::InvalidRecipient(InvalidRecipient(0x07)))
        );
    }

    #[test]
    fn test_args_raw_invalid_message_type() {
        // Invalid message type on connector 3, message offset 14, 1 byte
        let encoded: [u8; ArgsRaw::LEN] = [0x83, 0x38, 0x01, 0x0f, 0x00, 0x00];
        assert_eq!(
            Args::<GlobalPortId>::try_from(bytemuck::must_cast::<_, ArgsRaw>(encoded)),
            Err(InvalidArgs::InvalidMessageType(InvalidMessageType(0x0f)))
        );
    }

    #[test]
    fn test_response_data_raw_roundtrip() {
        // No particular meaning to these values
        let encoded: [u8; ResponseDataRaw::LEN] = [
            0x34, 0x12, 0x78, 0x56, 0x34, 0x12, 0x12, 0x34, 0x12, 0x34, 0x56, 0x78, 0xAB, 0xCD, 0xEF, 0x12,
        ];
        let expected = ResponseData { bytes: encoded };

        assert_eq!(
            ResponseData::from(bytemuck::must_cast::<_, ResponseDataRaw>(encoded)),
            expected
        );
        let bytes: [u8; ResponseDataRaw::LEN] = bytemuck::must_cast(ResponseDataRaw::from(expected));
        assert_eq!(bytes, encoded);
    }
}

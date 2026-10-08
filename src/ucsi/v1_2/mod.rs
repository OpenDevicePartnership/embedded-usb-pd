//! UCSI v1.2 implementation, see spec at https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/usb-type-c-ucsi-spec.pdf
#![allow(missing_docs)]

use bytemuck::{Pod, Zeroable};
use pack1::U32LE;

use crate::{GlobalPortId, LocalPortId, PdError, PortId};

pub mod cci;
pub mod lpm;
pub mod ppm;

/// Standard command length of 64 bits
pub const COMMAND_LEN: usize = 8;

/// Ucsi opcodes, see spec for more detail
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CommandType {
    PpmReset = 0x01,
    Cancel,
    ConnectorReset,
    AckCcCi,
    SetNotificationEnable,
    GetCapability,
    GetConnectorCapability,
    SetCcom,
    SetUor,
    SetPdm,
    SetPdr,
    GetAlternateModes,
    GetCamSupported,
    GetCurrentCam,
    SetNewCam,
    GetPdos,
    GetCableProperty,
    GetConnectorStatus,
    GetErrorStatus,
    SetPowerLevel,
    GetPdMessage,
}

impl CommandType {
    /// Returns true if this command has response data
    pub fn has_response(&self) -> bool {
        // Written as a negative so this function returns true for command not in this list.
        // One of the major uses for this function is to determine if there's a response to serialize.
        // If this function returns true by default then that makes it that a subsequent serialization
        // attempt will fail due to the lack of corresponding response data types. Otherwise the
        // serialization will not be attempted and no error will occur.
        !matches!(
            self,
            CommandType::PpmReset
                | CommandType::Cancel
                | CommandType::ConnectorReset
                | CommandType::AckCcCi
                | CommandType::SetNotificationEnable
                | CommandType::SetCcom
                | CommandType::SetUor
                | CommandType::SetPdr
                | CommandType::SetNewCam
        )
    }
}

/// Invalid command type error
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct InvalidCommandType(pub u8);

impl From<InvalidCommandType> for PdError {
    fn from(_: InvalidCommandType) -> Self {
        PdError::InvalidParams
    }
}

impl TryFrom<u8> for CommandType {
    type Error = InvalidCommandType;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(CommandType::PpmReset),
            0x02 => Ok(CommandType::Cancel),
            0x03 => Ok(CommandType::ConnectorReset),
            0x04 => Ok(CommandType::AckCcCi),
            0x05 => Ok(CommandType::SetNotificationEnable),
            0x06 => Ok(CommandType::GetCapability),
            0x07 => Ok(CommandType::GetConnectorCapability),
            0x08 => Ok(CommandType::SetCcom),
            0x09 => Ok(CommandType::SetUor),
            0x0A => Ok(CommandType::SetPdm),
            0x0B => Ok(CommandType::SetPdr),
            0x0C => Ok(CommandType::GetAlternateModes),
            0x0D => Ok(CommandType::GetCamSupported),
            0x0E => Ok(CommandType::GetCurrentCam),
            0x0F => Ok(CommandType::SetNewCam),
            0x10 => Ok(CommandType::GetPdos),
            0x11 => Ok(CommandType::GetCableProperty),
            0x12 => Ok(CommandType::GetConnectorStatus),
            0x13 => Ok(CommandType::GetErrorStatus),
            0x14 => Ok(CommandType::SetPowerLevel),
            0x15 => Ok(CommandType::GetPdMessage),
            _ => Err(InvalidCommandType(value)),
        }
    }
}

impl From<CommandType> for u8 {
    fn from(command: CommandType) -> Self {
        command as u8
    }
}

/// Length of a UCSI command payload, the command minus its header
pub const COMMAND_PAYLOAD_LEN: usize = COMMAND_LEN - CommandHeaderRaw::LEN;

/// Error returned when a command cannot be converted to or from its raw bytes
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum InvalidCommand {
    /// Not a valid command type
    InvalidCommandType(InvalidCommandType),
    /// Invalid LPM command
    Lpm(lpm::InvalidCommand),
}

impl From<InvalidCommandType> for InvalidCommand {
    fn from(value: InvalidCommandType) -> Self {
        InvalidCommand::InvalidCommandType(value)
    }
}

impl From<lpm::InvalidCommand> for InvalidCommand {
    fn from(value: lpm::InvalidCommand) -> Self {
        InvalidCommand::Lpm(value)
    }
}

impl From<InvalidCommand> for PdError {
    fn from(_: InvalidCommand) -> Self {
        PdError::InvalidParams
    }
}

/// Raw wire format of [`Command`]
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Zeroable, Pod)]
pub struct CommandRaw {
    /// Command type
    pub command: u8,
    /// Data length
    pub data_len: u8,
    /// Command payload, interpreted according to the command type
    pub payload: [u8; COMMAND_PAYLOAD_LEN],
}

impl CommandRaw {
    /// Length of a raw command in bytes
    pub const LEN: usize = size_of::<Self>();
}

#[cfg(feature = "defmt")]
impl defmt::Format for CommandRaw {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "CommandRaw {{ command: {}, data_len: {}, payload: {} }}",
            self.command,
            self.data_len,
            self.payload
        )
    }
}

/// UCSI commands
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Command<T: PortId> {
    PpmCommand(ppm::Command),
    LpmCommand(lpm::Command<T>),
}

pub type GlobalCommand = Command<GlobalPortId>;
pub type LocalCommand = Command<LocalPortId>;

impl<T: PortId> Command<T> {
    /// Length of a command in bytes
    pub const LEN: usize = COMMAND_LEN;
    /// Length of a command payload, the command minus its header
    pub const PAYLOAD_LEN: usize = COMMAND_PAYLOAD_LEN;

    /// Returns the command type for this command
    pub const fn command_type(&self) -> CommandType {
        match self {
            Command::PpmCommand(cmd) => cmd.command_type(),
            Command::LpmCommand(cmd) => cmd.command_type(),
        }
    }

    /// Returns the connector this command targets
    ///
    /// PPM commands are not connector specific and return `None`.
    pub fn connector_number(&self) -> Option<T> {
        match self {
            Command::PpmCommand(_) => None,
            Command::LpmCommand(cmd) => Some(cmd.connector_number()),
        }
    }

    /// Sets the connector this command targets
    ///
    /// Does nothing for PPM commands, which are not connector specific.
    pub fn set_connector_number(&mut self, connector: T) -> &mut Self {
        match self {
            Command::PpmCommand(_) => (),
            Command::LpmCommand(cmd) => {
                cmd.set_connector_number(connector);
            }
        }

        self
    }
}

impl<T: PortId> TryFrom<Command<T>> for CommandRaw {
    type Error = InvalidCommand;

    /// Converts a command into its raw wire format
    ///
    /// Returns an error if the arguments cannot be represented on the wire.
    fn try_from(command: Command<T>) -> Result<Self, Self::Error> {
        match command {
            Command::PpmCommand(cmd) => Ok(CommandRaw::from(cmd)),
            Command::LpmCommand(cmd) => Ok(CommandRaw::try_from(cmd)?),
        }
    }
}

impl<T: PortId> TryFrom<CommandRaw> for Command<T> {
    type Error = InvalidCommand;

    /// Reconstructs a command from its raw wire format
    ///
    /// The data length in the header is ignored, every modelled command has a fixed-size payload.
    fn try_from(raw: CommandRaw) -> Result<Self, Self::Error> {
        match CommandType::try_from(raw.command)? {
            // PPM commands
            CommandType::PpmReset
            | CommandType::Cancel
            | CommandType::GetCapability
            | CommandType::AckCcCi
            | CommandType::SetNotificationEnable => Ok(Command::PpmCommand(ppm::Command::try_from(raw)?)),
            // All other commands are LPM commands
            _ => Ok(Command::LpmCommand(lpm::Command::try_from(raw)?)),
        }
    }
}

/// Error returned when response data cannot be reconstructed from its raw bytes
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum InvalidResponseData {
    /// Not a valid command type
    InvalidCommandType(InvalidCommandType),
    /// Invalid LPM response data
    Lpm(lpm::InvalidResponseData),
}

impl From<InvalidCommandType> for InvalidResponseData {
    fn from(value: InvalidCommandType) -> Self {
        InvalidResponseData::InvalidCommandType(value)
    }
}

impl From<lpm::InvalidResponseData> for InvalidResponseData {
    fn from(value: lpm::InvalidResponseData) -> Self {
        InvalidResponseData::Lpm(value)
    }
}

impl From<InvalidResponseData> for PdError {
    fn from(_: InvalidResponseData) -> Self {
        PdError::InvalidParams
    }
}

/// UCSI command response data
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ResponseData {
    Ppm(ppm::ResponseData),
    Lpm(lpm::ResponseData),
}

impl ResponseData {
    /// Returns the command type that produces this response data
    pub const fn command_type(&self) -> CommandType {
        match self {
            ResponseData::Ppm(data) => data.command_type(),
            ResponseData::Lpm(data) => data.command_type(),
        }
    }

    /// Number of valid bytes this response data occupies on the wire
    pub fn data_len(&self) -> usize {
        match self {
            ResponseData::Ppm(data) => data.data_len(),
            ResponseData::Lpm(data) => data.data_len(),
        }
    }
}

impl From<ResponseData> for ResponseDataRaw {
    /// Converts response data from a [`ResponseDataRaw::MAX_LEN`] sized buffer
    ///
    /// Only the first [`ResponseData::data_len`] bytes are valid.
    fn from(data: ResponseData) -> Self {
        match data {
            ResponseData::Ppm(data) => data.into(),
            ResponseData::Lpm(data) => data.into(),
        }
    }
}

impl TryFrom<(CommandType, ResponseDataRaw)> for ResponseData {
    type Error = InvalidResponseData;

    /// Reconstructs response data from its command type and raw bytes
    fn try_from((command_type, bytes): (CommandType, ResponseDataRaw)) -> Result<Self, Self::Error> {
        match command_type {
            // PPM commands
            CommandType::PpmReset
            | CommandType::Cancel
            | CommandType::GetCapability
            | CommandType::AckCcCi
            | CommandType::SetNotificationEnable => Ok(ResponseData::Ppm(ppm::ResponseData::try_from((
                command_type,
                ResponseDataRaw { payload: bytes.payload },
            ))?)),
            // All other commands are LPM commands
            _ => Ok(ResponseData::Lpm(lpm::ResponseData::try_from((
                command_type,
                ResponseDataRaw { payload: bytes.payload },
            ))?)),
        }
    }
}

/// Raw wire format of [`ResponseData`]
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Zeroable, Pod)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ResponseDataRaw {
    /// Raw PDOs
    pub payload: [u8; ppm::RESPONSE_DATA_MAX_LEN],
}

impl ResponseDataRaw {
    /// Maximum length in bytes of any response data
    pub const MAX_LEN: usize = ppm::RESPONSE_DATA_MAX_LEN;
    /// The PPM and LPM response buffers are forwarded without resizing, so they must be the same size
    const _MAX_LEN_CHECK: () = assert!(ppm::RESPONSE_DATA_MAX_LEN == lpm::RESPONSE_DATA_MAX_LEN);
}

/// Raw wire format of [`Response`]
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Zeroable, Pod)]
pub struct ResponseRaw {
    /// Command status and connect change indicator
    pub cci: U32LE,
    /// Response data, interpreted according to the command type
    pub data: ResponseDataRaw,
}

impl ResponseRaw {
    /// Length of a raw response in bytes
    pub const LEN: usize = size_of::<Self>();
}

/// Length of a response in bytes, the CCI plus the maximum response data
pub const RESPONSE_LEN: usize = size_of::<ResponseRaw>();

#[cfg(feature = "defmt")]
impl defmt::Format for ResponseRaw {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "ResponseRaw {{ cci: {}, data: {} }}",
            cci::CciRaw::from(self.cci.get()),
            self.data
        )
    }
}

/// UCSI command response
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Response<T: PortId> {
    /// CCI is produced by every command
    pub cci: cci::CciNoDataLen<T>,
    /// Response data for the command
    pub data: Option<ResponseData>,
}

impl<T: PortId> Response<T> {
    /// Length of a response in bytes
    pub const LEN: usize = ResponseRaw::LEN;

    /// Number of valid bytes this response occupies on the wire
    ///
    /// Covers the CCI plus however much response data the command produced.
    pub fn valid_len(&self) -> usize {
        size_of::<U32LE>() + self.data.map_or(0, |data| data.data_len())
    }
}

impl<T: PortId> From<Response<T>> for ResponseRaw {
    /// Converts a response into its raw wire format
    ///
    /// Only the first [`Response::valid_len`] bytes are valid.
    fn from(response: Response<T>) -> Self {
        let data_len = response.data.map_or(0, |data| data.data_len());
        ResponseRaw {
            cci: U32LE::new(response.cci.into_cci(data_len as u8).into()),
            data: response.data.map_or(
                ResponseDataRaw {
                    payload: [0u8; ResponseDataRaw::MAX_LEN],
                },
                Into::into,
            ),
        }
    }
}

impl<T: PortId> TryFrom<(CommandType, ResponseRaw)> for Response<T> {
    type Error = InvalidResponseData;

    /// Reconstructs a response from its command type and raw wire format
    ///
    /// The command type is needed because a response carries no indication of which command
    /// produced it. Response data is only decoded for commands that produce it.
    fn try_from((command_type, raw): (CommandType, ResponseRaw)) -> Result<Self, Self::Error> {
        let cci = cci::Cci::<T>::from(raw.cci.get());
        let data = if command_type.has_response() && cci.data_len != 0 {
            Some(ResponseData::try_from((command_type, raw.data))?)
        } else {
            None
        };

        Ok(Self { cci: cci.into(), data })
    }
}

impl<T: PortId> From<cci::Cci<T>> for Response<T> {
    fn from(cci: cci::Cci<T>) -> Self {
        Self {
            cci: cci.into(),
            data: None,
        }
    }
}

impl<T: PortId> From<ppm::Response<T>> for Response<T> {
    fn from(response: ppm::Response<T>) -> Self {
        Self {
            cci: response.cci.into(),
            data: response.data.map(ResponseData::Ppm),
        }
    }
}

impl<T: PortId> From<lpm::Response<T>> for Response<T> {
    fn from(response: lpm::Response<T>) -> Self {
        Self {
            cci: response.cci.into(),
            data: response.data.map(ResponseData::Lpm),
        }
    }
}

pub type GlobalResponse = Response<GlobalPortId>;
pub type LocalResponse = Response<LocalPortId>;

/// Raw wire format of [`CommandHeader`], the common header shared by all UCSI commands
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Zeroable, Pod)]
pub struct CommandHeaderRaw {
    /// Command
    pub command: u8,
    /// Data length
    pub data_len: u8,
}

impl CommandHeaderRaw {
    /// Length of the raw command header in bytes
    pub const LEN: usize = size_of::<Self>();
}

#[cfg(feature = "defmt")]
impl defmt::Format for CommandHeaderRaw {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "CommandHeaderRaw {{ command: {}, data_len: {} }}",
            self.command,
            self.data_len
        )
    }
}

/// Higher-level wrapper around [`CommandHeaderRaw`]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct CommandHeader {
    /// Command type
    pub command: CommandType,
    /// Data length
    pub data_len: u8,
}

impl CommandHeader {
    /// Create a new command header
    pub const fn new(command: CommandType, data_len: u8) -> Self {
        Self { command, data_len }
    }
}

impl TryFrom<CommandHeaderRaw> for CommandHeader {
    type Error = InvalidCommandType;

    fn try_from(raw: CommandHeaderRaw) -> Result<Self, Self::Error> {
        Ok(Self {
            command: raw.command.try_into()?,
            data_len: raw.data_len,
        })
    }
}

impl From<CommandHeader> for CommandHeaderRaw {
    fn from(header: CommandHeader) -> Self {
        Self {
            command: header.command.into(),
            data_len: header.data_len,
        }
    }
}

impl TryFrom<u16> for CommandHeader {
    type Error = InvalidCommandType;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        CommandHeader::try_from(bytemuck::must_cast::<_, CommandHeaderRaw>(value.to_le_bytes()))
    }
}

impl From<CommandHeader> for u16 {
    fn from(header: CommandHeader) -> Self {
        u16::from_le_bytes(bytemuck::must_cast(CommandHeaderRaw::from(header)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ucsi::v1_2::lpm::get_pdos;

    /// Test PPM command round-tripping
    ///
    /// Only test one command just to make sure the overall flow works
    #[test]
    fn test_command_bytes_ppm() {
        let mut bytes = [0u8; COMMAND_LEN];
        bytes[0] = CommandType::AckCcCi as u8;
        bytes[2] = 0x2; // Set connector change ack

        let expected = GlobalCommand::PpmCommand(ppm::Command::AckCcCi(ppm::ack_cc_ci::Args {
            ack: ppm::ack_cc_ci::Ack::from(0x2),
        }));

        let raw = bytemuck::must_cast::<_, CommandRaw>(bytes);

        assert_eq!(Command::try_from(raw), Ok(expected));
        assert_eq!(CommandRaw::try_from(expected), Ok(raw));
    }

    /// Test LPM command round-tripping
    ///
    /// Only test one command just to make sure the overall flow works
    #[test]
    fn test_command_bytes_lpm() {
        let mut bytes = [0u8; COMMAND_LEN];
        bytes[0] = CommandType::GetConnectorStatus as u8;
        bytes[2] = 0x1;

        let expected = Command::LpmCommand(lpm::Command::new(lpm::CommandData::GetConnectorStatus(
            lpm::get_connector_status::Args {
                connector_number: GlobalPortId(1),
            },
        )));

        let raw = bytemuck::must_cast::<_, CommandRaw>(bytes);

        assert_eq!(Command::try_from(raw), Ok(expected));
        assert_eq!(CommandRaw::try_from(expected), Ok(raw));
    }

    /// A connector number is only meaningful for LPM commands
    #[test]
    fn test_command_connector() {
        let ppm = GlobalCommand::PpmCommand(ppm::Command::PpmReset);
        assert_eq!(ppm.connector_number(), None);

        let lpm = GlobalCommand::LpmCommand(lpm::Command::new(lpm::CommandData::GetConnectorStatus(
            lpm::get_connector_status::Args {
                connector_number: GlobalPortId(1),
            },
        )));
        assert_eq!(lpm.connector_number(), Some(GlobalPortId(1)));
    }

    /// Setting the connector updates the arguments that carry it, and is a no-op for PPM commands
    #[test]
    fn test_command_set_connector() {
        let mut ppm = GlobalCommand::PpmCommand(ppm::Command::PpmReset);
        let before = ppm;
        ppm.set_connector_number(GlobalPortId(2));
        assert_eq!(ppm, before);

        let mut lpm = GlobalCommand::LpmCommand(lpm::Command::new(lpm::CommandData::ConnectorReset(
            lpm::connector_reset::Args {
                connector_number: GlobalPortId(1),
                hard_reset: true,
            },
        )));
        lpm.set_connector_number(GlobalPortId(2));

        assert_eq!(lpm.connector_number(), Some(GlobalPortId(2)));
        assert_eq!(
            lpm,
            GlobalCommand::LpmCommand(lpm::Command::new(lpm::CommandData::ConnectorReset(
                lpm::connector_reset::Args {
                    connector_number: GlobalPortId(2),
                    hard_reset: true,
                }
            )))
        );
    }

    #[test]
    fn test_command_from_raw_invalid_command_type() {
        assert_eq!(
            GlobalCommand::try_from(CommandRaw::default()),
            Err(InvalidCommand::InvalidCommandType(InvalidCommandType(0)))
        );
    }

    /// Test PPM response encoding
    ///
    /// Only test one response type just to make sure the overall flow works
    #[test]
    fn test_ppm_response_encoding() {
        let (response_data, bytes) = ppm::get_capability::test::create_response_data();
        let expected = GlobalResponse {
            cci: cci::Cci::new_cmd_complete().into(),
            data: Some(ResponseData::Ppm(ppm::ResponseData::GetCapability(response_data))),
        };

        let raw = ResponseRaw::from(expected);
        let encoded_bytes = bytemuck::must_cast::<_, [u8; RESPONSE_LEN]>(raw);

        assert_eq!(
            expected.valid_len(),
            size_of::<U32LE>() + ppm::get_capability::RESPONSE_DATA_LEN
        );
        assert_eq!(
            encoded_bytes.get(..size_of::<U32LE>()).unwrap(),
            u32::from(expected.cci.into_cci(ppm::get_capability::RESPONSE_DATA_LEN as u8)).to_le_bytes()
        );
        assert_eq!(encoded_bytes.get(size_of::<U32LE>()..).unwrap(), bytes);
        assert_eq!(Response::try_from((CommandType::GetCapability, raw)), Ok(expected));
    }

    /// Test LPM response encoding
    ///
    /// Only test one response type just to make sure the overall flow works
    #[test]
    fn test_lpm_response_encoding() {
        let (response_data, bytes) = lpm::get_connector_status::test::create_response_data();
        let expected = GlobalResponse {
            cci: cci::Cci::new_cmd_complete().into(),
            data: Some(ResponseData::Lpm(lpm::ResponseData::GetConnectorStatus(response_data))),
        };

        let raw = ResponseRaw::from(expected);
        let encoded_bytes = bytemuck::must_cast::<_, [u8; RESPONSE_LEN]>(raw);

        assert_eq!(
            expected.valid_len(),
            size_of::<U32LE>() + lpm::get_connector_status::RESPONSE_DATA_LEN
        );
        assert_eq!(
            encoded_bytes
                .get(size_of::<U32LE>()..size_of::<U32LE>() + lpm::get_connector_status::RESPONSE_DATA_LEN)
                .unwrap(),
            bytes
        );
        assert_eq!(Response::try_from((CommandType::GetConnectorStatus, raw)), Ok(expected));
    }

    /// Commands without response data produce a CCI-only response
    #[test]
    fn test_response_without_data() {
        let expected = GlobalResponse {
            cci: cci::Cci::new_cmd_complete().into(),
            data: None,
        };

        let raw = ResponseRaw::from(expected);

        assert_eq!(expected.valid_len(), size_of::<U32LE>());
        assert_eq!(Response::try_from((CommandType::AckCcCi, raw)), Ok(expected));
    }

    /// Every defined command type round-trips through the raw header, preserving the data length
    #[test]
    fn test_command_header_roundtrip() {
        const COMMAND_TYPES: [CommandType; 21] = [
            CommandType::PpmReset,
            CommandType::Cancel,
            CommandType::ConnectorReset,
            CommandType::AckCcCi,
            CommandType::SetNotificationEnable,
            CommandType::GetCapability,
            CommandType::GetConnectorCapability,
            CommandType::SetCcom,
            CommandType::SetUor,
            CommandType::SetPdm,
            CommandType::SetPdr,
            CommandType::GetAlternateModes,
            CommandType::GetCamSupported,
            CommandType::GetCurrentCam,
            CommandType::SetNewCam,
            CommandType::GetPdos,
            CommandType::GetCableProperty,
            CommandType::GetConnectorStatus,
            CommandType::GetErrorStatus,
            CommandType::SetPowerLevel,
            CommandType::GetPdMessage,
        ];

        for command in COMMAND_TYPES {
            let expected = CommandHeader::new(command, 0x06);
            let raw = CommandHeaderRaw::from(expected);

            assert_eq!(raw.command, command as u8);
            assert_eq!(raw.data_len, 0x06);
            assert_eq!(CommandHeader::try_from(raw), Ok(expected));
            assert_eq!(CommandHeader::try_from(u16::from(expected)), Ok(expected));
        }
    }

    #[test]
    fn test_command_header_invalid_command() {
        assert_eq!(
            CommandHeader::try_from(CommandHeaderRaw {
                command: 0x00,
                data_len: 0x06
            }),
            Err(InvalidCommandType(0x00))
        );
        assert_eq!(
            CommandHeader::try_from(CommandHeaderRaw {
                command: 0x16,
                data_len: 0x06
            }),
            Err(InvalidCommandType(0x16))
        );
    }

    /// Verify that the data len is correctly propagated into the raw CCI
    #[test]
    fn test_data_len() {
        let response_data = Response::<GlobalPortId> {
            cci: cci::Cci::new_cmd_complete().into(),
            data: Some(ResponseData::Lpm(lpm::ResponseData::GetPdos(get_pdos::ResponseData {
                pdos: [0x12, 0x34, 0x56, 0x78],
            }))),
        };

        let raw = ResponseRaw::from(response_data);
        let cci = cci::Cci::<GlobalPortId>::from(raw.cci.get());
        assert_eq!(cci.data_len, 16);
    }
}

//! UCSI v1.2 implementation, see spec at https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/usb-type-c-ucsi-spec.pdf
#![allow(missing_docs)]

use bytemuck::{Pod, Zeroable};
use pack1::U32LE;

use crate::ucsi::{ConnectorNumber, MapPort};
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
    SetPdr = 0x0B,
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
                | CommandType::SetPowerLevel
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

    /// PPM commands are not connector specific and return `None`.
    pub fn connector_number(&self) -> Option<T> {
        match self {
            Command::PpmCommand(_) => None,
            Command::LpmCommand(cmd) => Some(cmd.connector_number()),
        }
    }

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

impl<T: PortId, Out: PortId> MapPort<T, Out> for Command<T> {
    type Output = Command<Out>;

    fn map_port<F>(self, f: F) -> Self::Output
    where
        F: Fn(T) -> Out,
    {
        match self {
            Command::PpmCommand(cmd) => Command::PpmCommand(cmd),
            Command::LpmCommand(cmd) => Command::LpmCommand(cmd.map_port(f)),
        }
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
            | CommandType::GetErrorStatus
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
    /// Invalid CCI field combination
    Cci(cci::InvalidFieldCombo),
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
            | CommandType::GetErrorStatus
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

impl<T: PortId, Out: PortId> MapPort<T, Out> for Response<T> {
    type Output = Response<Out>;

    fn map_port<F>(self, f: F) -> Self::Output
    where
        F: Fn(T) -> Out,
    {
        Response {
            cci: self.cci.map_port(f),
            data: self.data,
        }
    }
}

impl<T: PortId> TryFrom<Response<T>> for ResponseRaw {
    type Error = InvalidResponseData;
    /// Converts a response into its raw wire format
    ///
    /// Only the first [`Response::valid_len`] bytes are valid.
    fn try_from(response: Response<T>) -> Result<Self, Self::Error> {
        let data_len = response.data.map_or(0, |data| data.data_len());
        Ok(ResponseRaw {
            cci: U32LE::new(
                response
                    .cci
                    .into_cci(data_len as u8)
                    .try_into()
                    .map_err(InvalidResponseData::Cci)?,
            ),
            data: response.data.map_or(
                ResponseDataRaw {
                    payload: [0u8; ResponseDataRaw::MAX_LEN],
                },
                Into::into,
            ),
        })
    }
}

impl<T: PortId> TryFrom<(CommandType, ResponseRaw)> for Response<T> {
    type Error = InvalidResponseData;

    /// Reconstructs a response from its command type and raw wire format
    ///
    /// The command type is needed because a response carries no indication of which command
    /// produced it. Response data is only decoded for commands that produce it.
    fn try_from((command_type, raw): (CommandType, ResponseRaw)) -> Result<Self, Self::Error> {
        let cci = cci::Cci::<T>::try_from(raw.cci.get()).map_err(InvalidResponseData::Cci)?;
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
    use crate::ucsi::v1_2::cci::InvalidFieldCombo;
    use crate::ucsi::v1_2::lpm::get_pdos;

    /// Checks encoding and decoding independently against the wire fixture.
    /// Commands have an opcode, zero data length, then arguments and zero reserved bytes.
    fn assert_command_encoding(expected: GlobalCommand, bytes: [u8; COMMAND_LEN]) {
        let raw = bytemuck::must_cast::<_, CommandRaw>(bytes);

        assert_eq!(u8::from(expected.command_type()), raw.command);
        assert_eq!(Command::try_from(raw), Ok(expected));
        assert_eq!(CommandRaw::try_from(expected), Ok(raw));
    }

    #[test]
    fn test_ppm_reset_command_roundtrip() {
        // PPM_RESET (0x01), no arguments.
        assert_command_encoding(
            GlobalCommand::PpmCommand(ppm::Command::PpmReset),
            [0x01, 0, 0, 0, 0, 0, 0, 0],
        );
    }

    #[test]
    fn test_cancel_command_roundtrip() {
        // CANCEL (0x02), no arguments.
        assert_command_encoding(
            GlobalCommand::PpmCommand(ppm::Command::Cancel),
            [0x02, 0, 0, 0, 0, 0, 0, 0],
        );
    }

    #[test]
    fn test_ack_cc_ci_command_roundtrip() {
        // ACK_CC_CI (0x04), command-complete acknowledgement true (bit 1).
        assert_command_encoding(
            GlobalCommand::PpmCommand(ppm::Command::AckCcCi(ppm::ack_cc_ci::Args {
                ack: ppm::ack_cc_ci::Ack {
                    connector_change: false,
                    command_complete: true,
                },
            })),
            [0x04, 0, 0x02, 0, 0, 0, 0, 0],
        );
    }

    #[test]
    fn test_set_notification_enable_command_roundtrip() {
        // SET_NOTIFICATION_ENABLE (0x05), command-complete notification true (bit 0).
        assert_command_encoding(
            GlobalCommand::PpmCommand(ppm::Command::SetNotificationEnable(
                ppm::set_notification_enable::Args {
                    notification_enable: ppm::set_notification_enable::NotificationEnable {
                        cmd_complete: true,
                        ..Default::default()
                    },
                },
            )),
            [0x05, 0, 0x01, 0, 0, 0, 0, 0],
        );
    }

    #[test]
    fn test_get_capability_command_roundtrip() {
        // GET_CAPABILITY (0x06), no arguments.
        assert_command_encoding(
            GlobalCommand::PpmCommand(ppm::Command::GetCapability),
            [0x06, 0, 0, 0, 0, 0, 0, 0],
        );
    }

    #[test]
    fn test_get_error_status_command_roundtrip() {
        // GET_ERROR_STATUS (0x13), no arguments; routed to the PPM, not an LPM.
        assert_command_encoding(
            GlobalCommand::PpmCommand(ppm::Command::GetErrorStatus),
            [0x13, 0, 0, 0, 0, 0, 0, 0],
        );
    }

    #[test]
    fn test_connector_reset_command_roundtrip() {
        // CONNECTOR_RESET (0x03), connector 1.
        assert_command_encoding(
            GlobalCommand::LpmCommand(lpm::Command::new(lpm::CommandData::ConnectorReset(
                lpm::connector_reset::Args {
                    connector_number: GlobalPortId(1),
                },
            ))),
            [0x03, 0, 0x01, 0, 0, 0, 0, 0],
        );
    }

    #[test]
    fn test_get_connector_capability_command_roundtrip() {
        // GET_CONNECTOR_CAPABILITY (0x07), connector 1.
        assert_command_encoding(
            GlobalCommand::LpmCommand(lpm::Command::new(lpm::CommandData::GetConnectorCapability(
                lpm::get_connector_capability::Args {
                    connector_number: GlobalPortId(1),
                },
            ))),
            [0x07, 0, 0x01, 0, 0, 0, 0, 0],
        );
    }

    #[test]
    fn test_set_ccom_command_roundtrip() {
        // SET_CCOM (0x08), connector 1, Rp true (bit 7).
        assert_command_encoding(
            GlobalCommand::LpmCommand(lpm::Command::new(lpm::CommandData::SetCcom(lpm::set_ccom::Args {
                connector_number: GlobalPortId(1),
                rp: true,
                ..Default::default()
            }))),
            [0x08, 0, 0x81, 0, 0, 0, 0, 0],
        );
    }

    #[test]
    fn test_set_uor_command_roundtrip() {
        // SET_UOR (0x09), connector 1, DFP true (bit 7).
        assert_command_encoding(
            GlobalCommand::LpmCommand(lpm::Command::new(lpm::CommandData::SetUor(lpm::set_uor::Args {
                connector_number: GlobalPortId(1),
                dfp: true,
                ..Default::default()
            }))),
            [0x09, 0, 0x81, 0, 0, 0, 0, 0],
        );
    }

    #[test]
    fn test_set_pdr_command_roundtrip() {
        // SET_PDR (0x0B), connector 1, swap to source true (bit 7).
        assert_command_encoding(
            GlobalCommand::LpmCommand(lpm::Command::new(lpm::CommandData::SetPdr(lpm::set_pdr::Args {
                connector_number: GlobalPortId(1),
                swap_source: true,
                ..Default::default()
            }))),
            [0x0B, 0, 0x81, 0, 0, 0, 0, 0],
        );
    }

    #[test]
    fn test_get_alternate_modes_command_roundtrip() {
        // GET_ALTERNATE_MODES (0x0C), SOP (1), connector 3, offset 1, two modes.
        assert_command_encoding(
            GlobalCommand::LpmCommand(lpm::Command::new(lpm::CommandData::GetAlternateModes(
                lpm::get_alternate_modes::Args {
                    recipient: lpm::Recipient::Sop,
                    connector_number: GlobalPortId(3),
                    mode_offset: 1,
                    num_modes: 2,
                },
            ))),
            [0x0C, 0, 0x01, 0x03, 0x01, 0x02, 0, 0],
        );
    }

    #[test]
    fn test_get_cam_supported_command_roundtrip() {
        // GET_CAM_SUPPORTED (0x0D), connector 1.
        assert_command_encoding(
            GlobalCommand::LpmCommand(lpm::Command::new(lpm::CommandData::GetCamSupported(
                lpm::get_cam_supported::Args {
                    connector_number: GlobalPortId(1),
                },
            ))),
            [0x0D, 0, 0x01, 0, 0, 0, 0, 0],
        );
    }

    #[test]
    fn test_get_current_cam_command_roundtrip() {
        // GET_CURRENT_CAM (0x0E), connector 1.
        assert_command_encoding(
            GlobalCommand::LpmCommand(lpm::Command::new(lpm::CommandData::GetCurrentCam(
                lpm::get_current_cam::Args {
                    connector_number: GlobalPortId(1),
                },
            ))),
            [0x0E, 0, 0x01, 0, 0, 0, 0, 0],
        );
    }

    #[test]
    fn test_set_new_cam_command_roundtrip() {
        // SET_NEW_CAM (0x0F), connector 3, enter true (bit 7), offset 1.
        // AM-specific value 0x12345678 is little-endian.
        assert_command_encoding(
            GlobalCommand::LpmCommand(lpm::Command::new(lpm::CommandData::SetNewCam(lpm::set_new_cam::Args {
                connector_number: GlobalPortId(3),
                enter: true,
                am_offset: 1,
                am_specific: 0x12345678,
            }))),
            [0x0F, 0, 0x83, 0x01, 0x78, 0x56, 0x34, 0x12],
        );
    }

    #[test]
    fn test_get_pdos_command_roundtrip() {
        // GET_PDOS (0x10), connector 3, partner true (bit 7), PDO offset 4.
        // 0x14: one PDO (count minus one = 0), source (bit 2), maximum caps (bits 4:3 = 2).
        assert_command_encoding(
            GlobalCommand::LpmCommand(lpm::Command::new(lpm::CommandData::GetPdos(get_pdos::Args {
                connector_number: GlobalPortId(3),
                partner: true,
                pdo_offset: 4,
                num_pdos: 1,
                role: crate::PowerRole::Source,
                source_capability_type: get_pdos::SourceCapabilityType::Maximum,
            }))),
            [0x10, 0, 0x83, 0x04, 0x14, 0, 0, 0],
        );
    }

    #[test]
    fn test_get_cable_property_command_roundtrip() {
        // GET_CABLE_PROPERTY (0x11), connector 1.
        assert_command_encoding(
            GlobalCommand::LpmCommand(lpm::Command::new(lpm::CommandData::GetCableProperty(
                lpm::get_cable_property::Args {
                    connector_number: GlobalPortId(1),
                },
            ))),
            [0x11, 0, 0x01, 0, 0, 0, 0, 0],
        );
    }

    #[test]
    fn test_get_connector_status_command_roundtrip() {
        // GET_CONNECTOR_STATUS (0x12), connector 1.
        assert_command_encoding(
            GlobalCommand::LpmCommand(lpm::Command::new(lpm::CommandData::GetConnectorStatus(
                lpm::get_connector_status::Args {
                    connector_number: GlobalPortId(1),
                },
            ))),
            [0x12, 0, 0x01, 0, 0, 0, 0, 0],
        );
    }

    #[test]
    fn test_set_power_level_command_roundtrip() {
        // SET_POWER_LEVEL (0x14), connector 3, source (bit 7).
        // Max power 1000 mW (two 500 mW units), Type-C current 1.5 A (2).
        assert_command_encoding(
            GlobalCommand::LpmCommand(lpm::Command::new(lpm::CommandData::SetPowerLevel(
                lpm::set_power_level::Args {
                    connector_number: GlobalPortId(3),
                    power_role: crate::PowerRole::Source,
                    max_power: 1000,
                    type_c_current: lpm::set_power_level::Current::Current(crate::type_c::Current::Current1A5),
                },
            ))),
            [0x14, 0, 0x83, 0x02, 0x02, 0, 0, 0],
        );
    }

    #[test]
    fn test_get_pd_message_command_roundtrip() {
        // GET_PD_MESSAGE (0x15), connector 3, SOP (1, starting at payload bit 7).
        // 0x08: message offset 2; one byte; BatteryCap message type (2).
        assert_command_encoding(
            GlobalCommand::LpmCommand(lpm::Command::new(lpm::CommandData::GetPdMessage(
                lpm::get_pd_message::Args {
                    connector_number: GlobalPortId(3),
                    recipient: lpm::Recipient::Sop,
                    message_offset: 2,
                    num_bytes: 1,
                    message_type: lpm::get_pd_message::MessageType::BatteryCap,
                },
            ))),
            [0x15, 0, 0x83, 0x08, 0x01, 0x02, 0, 0],
        );
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
            },
        )));
        lpm.set_connector_number(GlobalPortId(2));

        assert_eq!(lpm.connector_number(), Some(GlobalPortId(2)));
        assert_eq!(
            lpm,
            GlobalCommand::LpmCommand(lpm::Command::new(lpm::CommandData::ConnectorReset(
                lpm::connector_reset::Args {
                    connector_number: GlobalPortId(2),
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

    /// Checks encoding and decoding independently against the wire fixture.
    /// Every fixture has command complete true, its payload length, and zero reserved bytes.
    fn assert_response_encoding(command_type: CommandType, data: Option<ResponseData>, bytes: &[u8]) {
        let expected = GlobalResponse {
            cci: cci::Cci::new_cmd_complete().into(),
            data,
        };
        let mut payload = [0u8; ResponseDataRaw::MAX_LEN];
        payload.get_mut(..bytes.len()).unwrap().copy_from_slice(bytes);
        // CCI bit 31 is command complete; bits 15:8 hold the payload length.
        let raw = ResponseRaw {
            cci: U32LE::new(0x8000_0000 | ((bytes.len() as u32) << 8)),
            data: ResponseDataRaw { payload },
        };

        assert_eq!(expected.valid_len(), size_of::<U32LE>() + bytes.len());
        assert_eq!(ResponseRaw::try_from(expected), Ok(raw));
        assert_eq!(Response::try_from((command_type, raw)), Ok(expected));
        if let Some(data) = data {
            assert_eq!(data.command_type(), command_type);
            assert_eq!(data.data_len(), bytes.len());
            assert_eq!(ResponseDataRaw::from(data), raw.data);
            assert_eq!(ResponseData::try_from((command_type, raw.data)), Ok(data));
        }
    }

    #[test]
    fn test_get_capability_response_roundtrip() {
        // GET_CAPABILITY: shared fixture documents attributes 0x43 and features 0xFF,
        // two connectors, three alternate modes, BC 1.20, PD 3.00, Type-C 2.00.
        let (data, bytes) = ppm::get_capability::test::create_response_data();
        assert_response_encoding(
            CommandType::GetCapability,
            Some(ResponseData::Ppm(ppm::ResponseData::GetCapability(data))),
            &bytes,
        );
    }

    #[test]
    fn test_get_error_status_response_roundtrip() {
        // GET_ERROR_STATUS: 0x00F0 sets CC communication, dead battery, contract
        // failure, and overcurrent errors; remaining bytes are arbitrary vendor data.
        let data = ppm::get_error_status::ResponseData {
            information: ppm::get_error_status::Information {
                cc_comm: true,
                dead_battery: true,
                contract_failure: true,
                overcurrent: true,
                ..Default::default()
            },
            vendor: [
                0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE,
            ],
        };
        assert_response_encoding(
            CommandType::GetErrorStatus,
            Some(ResponseData::Ppm(ppm::ResponseData::GetErrorStatus(data))),
            &[
                0xF0, 0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE,
            ],
        );
    }

    #[test]
    fn test_get_connector_capability_response_roundtrip() {
        // GET_CONNECTOR_CAPABILITY: 0xA1 sets Rp-only, USB2, and alternate mode.
        // 0x3F sets provider, consumer, and all four role-swap capabilities.
        let data = lpm::get_connector_capability::ResponseData {
            operation_mode: lpm::get_connector_capability::OperationModeFlags {
                rp_only: true,
                usb2: true,
                alternate_mode: true,
                ..Default::default()
            },
            provider: true,
            consumer: true,
            swap_to_dfp: true,
            swap_to_ufp: true,
            swap_to_src: true,
            swap_to_snk: true,
        };
        assert_response_encoding(
            CommandType::GetConnectorCapability,
            Some(ResponseData::Lpm(lpm::ResponseData::GetConnectorCapability(data))),
            &[0xA1, 0x3F],
        );
    }

    #[test]
    fn test_get_alternate_modes_response_roundtrip() {
        // GET_ALTERNATE_MODES: arbitrary SVID/MID pairs, all little-endian:
        // (0x1234, 0x12345678) and (0x3412, 0x78563412).
        let data = lpm::get_alternate_modes::ResponseData {
            alt_modes: [
                lpm::get_alternate_modes::AltMode {
                    svid: crate::vdm::structured::Svid(0x1234),
                    mid: crate::vdm::AltModeId(0x12345678),
                },
                lpm::get_alternate_modes::AltMode {
                    svid: crate::vdm::structured::Svid(0x3412),
                    mid: crate::vdm::AltModeId(0x78563412),
                },
            ],
        };
        assert_response_encoding(
            CommandType::GetAlternateModes,
            Some(ResponseData::Lpm(lpm::ResponseData::GetAlternateModes(data))),
            &[0x34, 0x12, 0x78, 0x56, 0x34, 0x12, 0x12, 0x34, 0x12, 0x34, 0x56, 0x78],
        );
    }

    #[test]
    fn test_get_cam_supported_response_roundtrip() {
        // GET_CAM_SUPPORTED: alternate modes at indices 1 and 4 supported (0x12).
        let data = lpm::get_cam_supported::ResponseData {
            alt_modes: [false, true, false, false, true, false, false, false],
        };
        assert_response_encoding(
            CommandType::GetCamSupported,
            Some(ResponseData::Lpm(lpm::ResponseData::GetCamSupported(data))),
            &[0x12],
        );
    }

    #[test]
    fn test_get_current_cam_response_roundtrip() {
        // GET_CURRENT_CAM: arbitrary alternate-mode indices 0 through 15.
        let bytes = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
        ];
        assert_response_encoding(
            CommandType::GetCurrentCam,
            Some(ResponseData::Lpm(lpm::ResponseData::GetCurrentCam(
                lpm::get_current_cam::ResponseData { alt_modes: bytes },
            ))),
            &bytes,
        );
    }

    #[test]
    fn test_get_pdos_response_roundtrip() {
        // GET_PDOS: four arbitrary raw PDOs (0x12, 0x34, 0x56, 0x78), little-endian.
        assert_response_encoding(
            CommandType::GetPdos,
            Some(ResponseData::Lpm(lpm::ResponseData::GetPdos(get_pdos::ResponseData {
                pdos: [0x12, 0x34, 0x56, 0x78],
            }))),
            &[0x12, 0, 0, 0, 0x34, 0, 0, 0, 0x56, 0, 0, 0, 0x78, 0, 0, 0],
        );
    }

    #[test]
    fn test_get_pdos_response_roundtrip_variable_length() {
        // GET_PDOS: two arbitrary raw PDOs (0x11223344, 0x55667788), little-endian.
        // Trailing zero slots are not included in the eight-byte wire payload.
        assert_response_encoding(
            CommandType::GetPdos,
            Some(ResponseData::Lpm(lpm::ResponseData::GetPdos(get_pdos::ResponseData {
                pdos: [0x11223344, 0x55667788, 0, 0],
            }))),
            &[0x44, 0x33, 0x22, 0x11, 0x88, 0x77, 0x66, 0x55],
        );
    }

    #[test]
    fn test_get_cable_property_response_roundtrip() {
        // GET_CABLE_PROPERTY: 0x0005 = 1 Kbps; current 100 mA (two 50 mA units).
        // 0xF7: VBUS, active, configurable direction, Type-C plug, alt mode, PD major 3.
        // 0x0A: latency 10.
        let data = lpm::get_cable_property::ResponseData {
            speed_supported: lpm::get_cable_property::SpeedSupported::Kbps(1),
            current_capability: 100,
            vbus_in_cable: true,
            active_cable: true,
            directionality_configurable: true,
            plug_end_type: lpm::get_cable_property::PlugEndType::TypeC,
            alt_mode_supported: true,
            cable_pd_major: 3,
            latency: 10,
        };
        assert_response_encoding(
            CommandType::GetCableProperty,
            Some(ResponseData::Lpm(lpm::ResponseData::GetCableProperty(data))),
            &[0x05, 0x00, 0x02, 0xF7, 0x0A],
        );
    }

    #[test]
    fn test_get_connector_status_response_roundtrip() {
        // GET_CONNECTOR_STATUS: shared fixture documents external-supply/error changes,
        // connected PD sink, USB/alt-mode DFP partner, raw RDO 0x78563412,
        // nominal charging, lowered provider power, and PD version 3.00.
        let (data, bytes) = lpm::get_connector_status::test::create_response_data();
        assert_response_encoding(
            CommandType::GetConnectorStatus,
            Some(ResponseData::Lpm(lpm::ResponseData::GetConnectorStatus(data))),
            &bytes,
        );
    }

    #[test]
    fn test_get_pd_message_response_roundtrip() {
        // GET_PD_MESSAGE: arbitrary 16-byte message fragment, passed through unchanged.
        let bytes = [
            0x34, 0x12, 0x78, 0x56, 0x34, 0x12, 0x12, 0x34, 0x12, 0x34, 0x56, 0x78, 0xAB, 0xCD, 0xEF, 0x12,
        ];
        assert_response_encoding(
            CommandType::GetPdMessage,
            Some(ResponseData::Lpm(lpm::ResponseData::GetPdMessage(
                lpm::get_pd_message::ResponseData { bytes },
            ))),
            &bytes,
        );
    }

    #[test]
    fn test_ppm_reset_response_roundtrip() {
        // PPM_RESET: CCI only, no response data.
        assert_response_encoding(CommandType::PpmReset, None, &[]);
    }

    #[test]
    fn test_cancel_response_roundtrip() {
        // CANCEL: CCI only, no response data.
        assert_response_encoding(CommandType::Cancel, None, &[]);
    }

    #[test]
    fn test_ack_cc_ci_response_roundtrip() {
        // ACK_CC_CI: CCI only, no response data.
        assert_response_encoding(CommandType::AckCcCi, None, &[]);
    }

    #[test]
    fn test_set_notification_enable_response_roundtrip() {
        // SET_NOTIFICATION_ENABLE: CCI only, no response data.
        assert_response_encoding(CommandType::SetNotificationEnable, None, &[]);
    }

    #[test]
    fn test_connector_reset_response_roundtrip() {
        // CONNECTOR_RESET: CCI only, no response data.
        assert_response_encoding(CommandType::ConnectorReset, None, &[]);
    }

    #[test]
    fn test_set_ccom_response_roundtrip() {
        // SET_CCOM: CCI only, no response data.
        assert_response_encoding(CommandType::SetCcom, None, &[]);
    }

    #[test]
    fn test_set_uor_response_roundtrip() {
        // SET_UOR: CCI only, no response data.
        assert_response_encoding(CommandType::SetUor, None, &[]);
    }

    #[test]
    fn test_set_pdr_response_roundtrip() {
        // SET_PDR: CCI only, no response data.
        assert_response_encoding(CommandType::SetPdr, None, &[]);
    }

    #[test]
    fn test_set_new_cam_response_roundtrip() {
        // SET_NEW_CAM: CCI only, no response data.
        assert_response_encoding(CommandType::SetNewCam, None, &[]);
    }

    #[test]
    fn test_set_power_level_response_roundtrip() {
        // SET_POWER_LEVEL: CCI only, no response data.
        assert_response_encoding(CommandType::SetPowerLevel, None, &[]);
    }

    /// Every defined command type round-trips through the raw header, preserving the data length
    #[test]
    fn test_command_header_roundtrip() {
        const COMMAND_TYPES: [CommandType; 20] = [
            CommandType::PpmReset,
            CommandType::Cancel,
            CommandType::ConnectorReset,
            CommandType::AckCcCi,
            CommandType::SetNotificationEnable,
            CommandType::GetCapability,
            CommandType::GetConnectorCapability,
            CommandType::SetCcom,
            CommandType::SetUor,
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

        let raw = ResponseRaw::try_from(response_data).unwrap();
        let cci = cci::Cci::<GlobalPortId>::try_from(raw.cci.get()).unwrap();
        assert_eq!(cci.data_len, 16);
    }

    /// Test PPM response encoding with an invalid CCI
    ///
    /// Only test this with a single command just to check the overall flow
    #[test]
    fn test_ppm_response_invalid_cci() {
        let (response_data, _bytes) = ppm::get_capability::test::create_response_data();
        let expected = GlobalResponse {
            cci: cci::CciNoDataLen {
                // Invalid combination: cmd_complete and reset_complete cannot both be set
                cmd_complete: true,
                reset_complete: true,
                ..Default::default()
            },
            data: Some(ResponseData::Ppm(ppm::ResponseData::GetCapability(response_data))),
        };

        let result = ResponseRaw::try_from(expected);
        assert_eq!(
            result,
            Err(InvalidResponseData::Cci(InvalidFieldCombo {
                cmd_complete: true,
                reset_complete: true,
                data_len: true,
                ..Default::default()
            }))
        );
    }

    /// Test LPM response encoding with an invalid CCI
    ///
    /// Only test this with a single command just to check the overall flow
    #[test]
    fn test_lpm_response_invalid_cci() {
        let (response_data, _bytes) = lpm::get_connector_status::test::create_response_data();
        let expected = GlobalResponse {
            cci: cci::CciNoDataLen {
                // Invalid combination: cmd_complete and reset_complete cannot both be set
                cmd_complete: true,
                reset_complete: true,
                ..Default::default()
            },
            data: Some(ResponseData::Lpm(lpm::ResponseData::GetConnectorStatus(response_data))),
        };

        let result = ResponseRaw::try_from(expected);
        assert_eq!(
            result,
            Err(InvalidResponseData::Cci(InvalidFieldCombo {
                cmd_complete: true,
                reset_complete: true,
                data_len: true,
                ..Default::default()
            }))
        );
    }

    /// Test all values of [`CommandType::has_response`]
    #[test]
    fn test_command_type_has_response() {
        assert_eq!(CommandType::PpmReset.has_response(), false);
        assert_eq!(CommandType::Cancel.has_response(), false);
        assert_eq!(CommandType::ConnectorReset.has_response(), false);
        assert_eq!(CommandType::AckCcCi.has_response(), false);
        assert_eq!(CommandType::SetNotificationEnable.has_response(), false);
        assert_eq!(CommandType::GetCapability.has_response(), true);
        assert_eq!(CommandType::GetConnectorCapability.has_response(), true);
        assert_eq!(CommandType::SetCcom.has_response(), false);
        assert_eq!(CommandType::SetUor.has_response(), false);
        assert_eq!(CommandType::SetPdr.has_response(), false);
        assert_eq!(CommandType::GetAlternateModes.has_response(), true);
        assert_eq!(CommandType::GetCamSupported.has_response(), true);
        assert_eq!(CommandType::GetCurrentCam.has_response(), true);
        assert_eq!(CommandType::SetNewCam.has_response(), false);
        assert_eq!(CommandType::GetPdos.has_response(), true);
        assert_eq!(CommandType::GetCableProperty.has_response(), true);
        assert_eq!(CommandType::GetConnectorStatus.has_response(), true);
        assert_eq!(CommandType::GetErrorStatus.has_response(), true);
        assert_eq!(CommandType::SetPowerLevel.has_response(), false);
        assert_eq!(CommandType::GetPdMessage.has_response(), true);
    }
}

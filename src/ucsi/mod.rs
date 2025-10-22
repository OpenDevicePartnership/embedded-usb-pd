//! Ucsi types, see spec at https://www.usb.org/document-library/usb-type-cr-connector-system-software-interface-ucsi-specification
#![allow(missing_docs)]

use bincode::de::{Decode, Decoder};
use bincode::enc::{Encode, Encoder};
use bincode::error::{AllowedEnumVariants, DecodeError, EncodeError};
use bincode::{decode_from_slice, encode_into_slice};
use bitfield::bitfield;

use crate::{GlobalPortId, LocalPortId, PdError, PortId};

pub mod cci;
pub mod lpm;
pub mod ppm;

/// Standard command length of 64 bits
pub const COMMAND_LEN: usize = 8;

/// Ucsi opcodes, see spec for more detail
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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
    GetAttentionVdo,
    GetCamCs = 0x18,
    LpmFwUpdateRequest,
    SecurityRequest,
    SetRetimerMode,
    SetSinkPath,
    SetPdos,
    ReadPowerLevel,
    ChunkingSupport,
    SetUsb = 0x21,
    GetLpmPpmInfo,
}

impl CommandType {
    /// Returns true if this command has response data
    pub fn has_response(&self) -> bool {
        // Written as a negative so this function returns true for command not in this list.
        // One of the major uses for this function is to determine if there's a response to serialize.
        // If this function returns true by default then that makes it that a subsequent sealization
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
            0x16 => Ok(CommandType::GetAttentionVdo),
            0x18 => Ok(CommandType::GetCamCs),
            0x19 => Ok(CommandType::LpmFwUpdateRequest),
            0x1A => Ok(CommandType::SecurityRequest),
            0x1B => Ok(CommandType::SetRetimerMode),
            0x1C => Ok(CommandType::SetSinkPath),
            0x1D => Ok(CommandType::SetPdos),
            0x1E => Ok(CommandType::ReadPowerLevel),
            0x1F => Ok(CommandType::ChunkingSupport),
            0x21 => Ok(CommandType::SetUsb),
            0x22 => Ok(CommandType::GetLpmPpmInfo),
            _ => Err(InvalidCommandType(value)),
        }
    }
}

impl From<CommandType> for u8 {
    fn from(command: CommandType) -> Self {
        command as u8
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
    /// Returns the command type for this command
    pub const fn command_type(&self) -> CommandType {
        match self {
            Command::PpmCommand(cmd) => cmd.command_type(),
            Command::LpmCommand(cmd) => cmd.command_type(),
        }
    }

    /// Deserialize the a command from a slice
    pub fn decode_from_slice(bytes: &[u8]) -> Result<(Self, usize), DecodeError> {
        decode_from_slice(bytes, bincode::config::standard().with_fixed_int_encoding())
    }
}

impl<Context, T: PortId> Decode<Context> for Command<T> {
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, DecodeError> {
        let header = CommandHeader::decode(decoder)?;
        let mut decoder = decoder.with_context(header);
        match header.command() {
            // PPM commands
            CommandType::PpmReset
            | CommandType::Cancel
            | CommandType::GetCapability
            | CommandType::AckCcCi
            | CommandType::SetNotificationEnable => {
                let command = ppm::Command::decode(&mut decoder)?;
                Ok(Command::PpmCommand(command))
            }
            // All other commands are LPM commands
            _ => {
                let command = lpm::Command::decode(&mut decoder)?;
                Ok(Command::LpmCommand(command))
            }
        }
    }
}

/// UCSI command response data
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ResponseData {
    Ppm(ppm::ResponseData),
    Lpm(lpm::ResponseData),
}

impl ResponseData {
    /// Encodes the response into a slice
    pub fn encode_into_slice(&self, bytes: &mut [u8]) -> Result<usize, EncodeError> {
        encode_into_slice(self, bytes, bincode::config::standard().with_fixed_int_encoding())
    }
}

impl Encode for ResponseData {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        match self {
            ResponseData::Ppm(resp) => resp.encode(encoder),
            ResponseData::Lpm(resp) => resp.encode(encoder),
        }
    }
}

/// UCSI command response
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Response<T: PortId> {
    /// CCI is produced by every command
    pub cci: cci::Cci<T>,
    /// Response data for the command
    pub data: Option<ResponseData>,
}

impl<T: PortId> From<cci::Cci<T>> for Response<T> {
    fn from(cci: cci::Cci<T>) -> Self {
        Self { cci, data: None }
    }
}

impl<T: PortId> From<ppm::Response<T>> for Response<T> {
    fn from(response: ppm::Response<T>) -> Self {
        Self {
            cci: response.cci,
            data: response.data.map(ResponseData::Ppm),
        }
    }
}

impl<T: PortId> From<lpm::Response<T>> for Response<T> {
    fn from(response: lpm::Response<T>) -> Self {
        Self {
            cci: response.cci,
            data: response.data.map(ResponseData::Lpm),
        }
    }
}

pub type GlobalResponse = Response<GlobalPortId>;
pub type LocalResponse = Response<LocalPortId>;

bitfield! {
    /// Common header shared by all UCSI commands
    #[derive(Copy, Clone)]
    pub(self) struct CommandHeaderRaw(u16);
    impl Debug;

    /// Command
    pub u8, command, set_command: 7, 0;
    /// Data length
    pub u8, data_len, set_data_len: 15, 8;
}

#[cfg(feature = "defmt")]
impl defmt::Format for CommandHeaderRaw {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "CommandHeaderRaw {{ command: {}, data_len: {} }}",
            self.command(),
            self.data_len()
        )
    }
}

/// Higher-level wrapper around [`CommandHeaderRaw`]
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct CommandHeader(CommandHeaderRaw);

impl CommandHeader {
    /// Create a new command header
    pub fn new(command: CommandType, data_len: u8) -> Self {
        let mut raw = CommandHeaderRaw(0);
        raw.set_command(command.into());
        raw.set_data_len(data_len);
        Self(raw)
    }

    /// Returns command type
    pub fn command(&self) -> CommandType {
        // Unwrap is safe here because we validate the command in `try_from`
        self.0.command().try_into().unwrap()
    }

    /// Sets command type
    pub fn set_command(&mut self, command: CommandType) -> &mut Self {
        self.0.set_command(command as u8);
        self
    }

    /// Returns data length
    pub fn data_len(&self) -> u8 {
        self.0.data_len()
    }

    /// Sets data length
    pub fn set_data_len(&mut self, len: u8) -> &mut Self {
        self.0.set_data_len(len);
        self
    }
}

impl TryFrom<u16> for CommandHeader {
    type Error = InvalidCommandType;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let raw = CommandHeaderRaw(value);

        // Validate command
        let _: CommandType = raw.command().try_into()?;
        Ok(Self(raw))
    }
}

impl Encode for CommandHeader {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.0 .0.encode(encoder)
    }
}

impl<Context> Decode<Context> for CommandHeader {
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, DecodeError> {
        let raw = u16::decode(decoder)?;
        CommandHeader::try_from(raw).map_err(|_| DecodeError::UnexpectedVariant {
            type_name: "CommandHeader",
            allowed: &AllowedEnumVariants::Allowed(&[
                CommandType::PpmReset as u32,
                CommandType::Cancel as u32,
                CommandType::ConnectorReset as u32,
                CommandType::AckCcCi as u32,
                CommandType::SetNotificationEnable as u32,
                CommandType::GetCapability as u32,
                CommandType::GetConnectorCapability as u32,
                CommandType::SetCcom as u32,
                CommandType::SetUor as u32,
                CommandType::SetPdm as u32,
                CommandType::SetPdr as u32,
                CommandType::GetAlternateModes as u32,
                CommandType::GetCamSupported as u32,
                CommandType::GetCurrentCam as u32,
                CommandType::SetNewCam as u32,
                CommandType::GetPdos as u32,
                CommandType::GetCableProperty as u32,
                CommandType::GetConnectorStatus as u32,
                CommandType::GetErrorStatus as u32,
                CommandType::SetPowerLevel as u32,
                CommandType::GetPdMessage as u32,
                CommandType::GetAttentionVdo as u32,
                CommandType::GetCamCs as u32,
                CommandType::LpmFwUpdateRequest as u32,
                CommandType::SecurityRequest as u32,
                CommandType::SetRetimerMode as u32,
                CommandType::SetSinkPath as u32,
                CommandType::SetPdos as u32,
                CommandType::ReadPowerLevel as u32,
                CommandType::ChunkingSupport as u32,
                CommandType::SetUsb as u32,
            ]),
            found: raw as u32,
        })
    }
}

#[cfg(test)]
mod tests {
    use bincode::config::standard;
    use bincode::decode_from_slice;

    use super::*;

    /// Test PPM command decoding
    ///
    /// Only test one command just to make sure the overall flow works
    #[test]
    fn test_command_decoding_ppm() {
        let mut bytes = [0u8; COMMAND_LEN];
        bytes[0] = CommandType::AckCcCi as u8;
        bytes[2] = 0x2; // Set connector change ack

        let (ack_cc_ci, consumed) = Command::decode_from_slice(&bytes).unwrap();
        assert_eq!(consumed, bytes.len());
        assert_eq!(
            ack_cc_ci,
            GlobalCommand::PpmCommand(ppm::Command::AckCcCi(ppm::ack_cc_ci::Args {
                ack: ppm::ack_cc_ci::Ack::from(0x2)
            }))
        );
    }

    /// Test LPM command decoding
    ///
    /// Only test one command just to make sure the overall flow works
    #[test]
    fn test_command_decoding_lpm() {
        let mut bytes = [0u8; COMMAND_LEN];
        bytes[0] = CommandType::GetConnectorStatus as u8;
        bytes[2] = 0x1;

        let (get_connector_status, consumed) = Command::decode_from_slice(&bytes).unwrap();
        assert_eq!(consumed, bytes.len());
        assert_eq!(
            get_connector_status,
            Command::LpmCommand(lpm::Command::new(GlobalPortId(1), lpm::CommandData::GetConnectorStatus))
        );
    }

    /// Test PPM response encoding
    ///
    /// Only test one response type just to make sure the overall flow works
    #[test]
    fn test_ppm_response_encoding() {
        let (response_data, bytes) = ppm::get_capability::test::create_response_data();
        let expected = ResponseData::Ppm(ppm::ResponseData::GetCapability(response_data));

        let mut encoded_bytes = [0u8; ppm::get_capability::RESPONSE_DATA_LEN];
        let len = expected.encode_into_slice(&mut encoded_bytes).unwrap();

        assert_eq!(len, ppm::get_capability::RESPONSE_DATA_LEN);
        assert_eq!(encoded_bytes, bytes);
    }

    /// Test LPM response encoding
    ///
    /// Only test one response type just to make sure the overall flow works
    #[test]
    fn test_lpm_response_encoding() {
        let (response_data, bytes) = lpm::get_connector_status::test::create_response_data();
        let expected = ResponseData::Lpm(lpm::ResponseData::GetConnectorStatus(response_data));

        let mut encoded_bytes = [0u8; lpm::get_connector_status::RESPONSE_DATA_LEN];
        let len = expected.encode_into_slice(&mut encoded_bytes).unwrap();

        assert_eq!(len, lpm::get_connector_status::RESPONSE_DATA_LEN);
        assert_eq!(encoded_bytes, bytes);
    }

    #[test]
    fn test_command_header_decoding_ppm_reset() {
        let bytes = [CommandType::PpmReset as u8, 0x00];
        let (header, consumed): (CommandHeader, usize) =
            decode_from_slice(&bytes, standard().with_fixed_int_encoding()).unwrap();
        assert_eq!(consumed, 2);
        assert_eq!(header.command(), CommandType::PpmReset);
        assert_eq!(header.data_len(), 0);
    }

    #[test]
    fn test_command_header_decoding_cancel() {
        let bytes = [CommandType::Cancel as u8, 0x00];
        let (header, consumed): (CommandHeader, usize) =
            decode_from_slice(&bytes, standard().with_fixed_int_encoding()).unwrap();
        assert_eq!(consumed, 2);
        assert_eq!(header.command(), CommandType::Cancel);
        assert_eq!(header.data_len(), 0);
    }

    #[test]
    fn test_command_header_decoding_connector_reset() {
        let bytes = [CommandType::ConnectorReset as u8, 0x00];
        let (header, consumed): (CommandHeader, usize) =
            decode_from_slice(&bytes, standard().with_fixed_int_encoding()).unwrap();
        assert_eq!(consumed, 2);
        assert_eq!(header.command(), CommandType::ConnectorReset);
        assert_eq!(header.data_len(), 0);
    }

    #[test]
    fn test_command_header_decoding_ack_cc_ci() {
        let bytes = [CommandType::AckCcCi as u8, 0x00];
        let (header, consumed): (CommandHeader, usize) =
            decode_from_slice(&bytes, standard().with_fixed_int_encoding()).unwrap();
        assert_eq!(consumed, 2);
        assert_eq!(header.command(), CommandType::AckCcCi);
        assert_eq!(header.data_len(), 0);
    }

    #[test]
    fn test_command_header_decoding_set_notification_enable() {
        let bytes = [CommandType::SetNotificationEnable as u8, 0x00];
        let (header, consumed): (CommandHeader, usize) =
            decode_from_slice(&bytes, standard().with_fixed_int_encoding()).unwrap();
        assert_eq!(consumed, 2);
        assert_eq!(header.command(), CommandType::SetNotificationEnable);
        assert_eq!(header.data_len(), 0);
    }

    #[test]
    fn test_command_header_decoding_get_capability() {
        let bytes = [CommandType::GetCapability as u8, 0x00];
        let (header, consumed): (CommandHeader, usize) =
            decode_from_slice(&bytes, standard().with_fixed_int_encoding()).unwrap();
        assert_eq!(consumed, 2);
        assert_eq!(header.command(), CommandType::GetCapability);
        assert_eq!(header.data_len(), 0);
    }

    #[test]
    fn test_command_header_decoding_get_connector_capability() {
        let bytes = [CommandType::GetConnectorCapability as u8, 0x00];
        let (header, consumed): (CommandHeader, usize) =
            decode_from_slice(&bytes, standard().with_fixed_int_encoding()).unwrap();
        assert_eq!(consumed, 2);
        assert_eq!(header.command(), CommandType::GetConnectorCapability);
        assert_eq!(header.data_len(), 0);
    }

    #[test]
    fn test_command_header_decoding_set_ccom() {
        let bytes = [CommandType::SetCcom as u8, 0x00];
        let (header, consumed): (CommandHeader, usize) =
            decode_from_slice(&bytes, standard().with_fixed_int_encoding()).unwrap();
        assert_eq!(consumed, 2);
        assert_eq!(header.command(), CommandType::SetCcom);
        assert_eq!(header.data_len(), 0);
    }

    #[test]
    fn test_command_header_decoding_set_uor() {
        let bytes = [CommandType::SetUor as u8, 0x00];
        let (header, consumed): (CommandHeader, usize) =
            decode_from_slice(&bytes, standard().with_fixed_int_encoding()).unwrap();
        assert_eq!(consumed, 2);
        assert_eq!(header.command(), CommandType::SetUor);
        assert_eq!(header.data_len(), 0);
    }

    #[test]
    fn test_command_header_decoding_set_pdm() {
        let bytes = [CommandType::SetPdm as u8, 0x00];
        let (header, consumed): (CommandHeader, usize) =
            decode_from_slice(&bytes, standard().with_fixed_int_encoding()).unwrap();
        assert_eq!(consumed, 2);
        assert_eq!(header.command(), CommandType::SetPdm);
        assert_eq!(header.data_len(), 0);
    }

    #[test]
    fn test_command_header_decoding_set_pdr() {
        let bytes = [CommandType::SetPdr as u8, 0x00];
        let (header, consumed): (CommandHeader, usize) =
            decode_from_slice(&bytes, standard().with_fixed_int_encoding()).unwrap();
        assert_eq!(consumed, 2);
        assert_eq!(header.command(), CommandType::SetPdr);
        assert_eq!(header.data_len(), 0);
    }

    #[test]
    fn test_command_header_decoding_get_alternate_modes() {
        let bytes = [CommandType::GetAlternateModes as u8, 0x00];
        let (header, consumed): (CommandHeader, usize) =
            decode_from_slice(&bytes, standard().with_fixed_int_encoding()).unwrap();
        assert_eq!(consumed, 2);
        assert_eq!(header.command(), CommandType::GetAlternateModes);
        assert_eq!(header.data_len(), 0);
    }

    #[test]
    fn test_command_header_decoding_get_cam_supported() {
        let bytes = [CommandType::GetCamSupported as u8, 0x00];
        let (header, consumed): (CommandHeader, usize) =
            decode_from_slice(&bytes, standard().with_fixed_int_encoding()).unwrap();
        assert_eq!(consumed, 2);
        assert_eq!(header.command(), CommandType::GetCamSupported);
        assert_eq!(header.data_len(), 0);
    }

    #[test]
    fn test_command_header_decoding_get_current_cam() {
        let bytes = [CommandType::GetCurrentCam as u8, 0x00];
        let (header, consumed): (CommandHeader, usize) =
            decode_from_slice(&bytes, standard().with_fixed_int_encoding()).unwrap();
        assert_eq!(consumed, 2);
        assert_eq!(header.command(), CommandType::GetCurrentCam);
        assert_eq!(header.data_len(), 0);
    }

    #[test]
    fn test_command_header_decoding_set_new_cam() {
        let bytes = [CommandType::SetNewCam as u8, 0x00];
        let (header, consumed): (CommandHeader, usize) =
            decode_from_slice(&bytes, standard().with_fixed_int_encoding()).unwrap();
        assert_eq!(consumed, 2);
        assert_eq!(header.command(), CommandType::SetNewCam);
        assert_eq!(header.data_len(), 0);
    }

    #[test]
    fn test_command_header_decoding_get_pdos() {
        let bytes = [CommandType::GetPdos as u8, 0x00];
        let (header, consumed): (CommandHeader, usize) =
            decode_from_slice(&bytes, standard().with_fixed_int_encoding()).unwrap();
        assert_eq!(consumed, 2);
        assert_eq!(header.command(), CommandType::GetPdos);
        assert_eq!(header.data_len(), 0);
    }

    #[test]
    fn test_command_header_decoding_get_cable_property() {
        let bytes = [CommandType::GetCableProperty as u8, 0x00];
        let (header, consumed): (CommandHeader, usize) =
            decode_from_slice(&bytes, standard().with_fixed_int_encoding()).unwrap();
        assert_eq!(consumed, 2);
        assert_eq!(header.command(), CommandType::GetCableProperty);
        assert_eq!(header.data_len(), 0);
    }

    #[test]
    fn test_command_header_decoding_get_connector_status() {
        let bytes = [CommandType::GetConnectorStatus as u8, 0x00];
        let (header, consumed): (CommandHeader, usize) =
            decode_from_slice(&bytes, standard().with_fixed_int_encoding()).unwrap();
        assert_eq!(consumed, 2);
        assert_eq!(header.command(), CommandType::GetConnectorStatus);
        assert_eq!(header.data_len(), 0);
    }

    #[test]
    fn test_command_header_decoding_get_error_status() {
        let bytes = [CommandType::GetErrorStatus as u8, 0x00];
        let (header, consumed): (CommandHeader, usize) =
            decode_from_slice(&bytes, standard().with_fixed_int_encoding()).unwrap();
        assert_eq!(consumed, 2);
        assert_eq!(header.command(), CommandType::GetErrorStatus);
        assert_eq!(header.data_len(), 0);
    }

    #[test]
    fn test_command_header_decoding_set_power_level() {
        let bytes = [CommandType::SetPowerLevel as u8, 0x00];
        let (header, consumed): (CommandHeader, usize) =
            decode_from_slice(&bytes, standard().with_fixed_int_encoding()).unwrap();
        assert_eq!(consumed, 2);
        assert_eq!(header.command(), CommandType::SetPowerLevel);
        assert_eq!(header.data_len(), 0);
    }

    #[test]
    fn test_command_header_decoding_get_pd_message() {
        let bytes = [CommandType::GetPdMessage as u8, 0x00];
        let (header, consumed): (CommandHeader, usize) =
            decode_from_slice(&bytes, standard().with_fixed_int_encoding()).unwrap();
        assert_eq!(consumed, 2);
        assert_eq!(header.command(), CommandType::GetPdMessage);
        assert_eq!(header.data_len(), 0);
    }

    #[test]
    fn test_command_header_decoding_get_attention_vdo() {
        let bytes = [CommandType::GetAttentionVdo as u8, 0x00];
        let (header, consumed): (CommandHeader, usize) =
            decode_from_slice(&bytes, standard().with_fixed_int_encoding()).unwrap();
        assert_eq!(consumed, 2);
        assert_eq!(header.command(), CommandType::GetAttentionVdo);
        assert_eq!(header.data_len(), 0);
    }

    #[test]
    fn test_command_header_decoding_get_cam_cs() {
        let bytes = [CommandType::GetCamCs as u8, 0x00];
        let (header, consumed): (CommandHeader, usize) =
            decode_from_slice(&bytes, standard().with_fixed_int_encoding()).unwrap();
        assert_eq!(consumed, 2);
        assert_eq!(header.command(), CommandType::GetCamCs);
        assert_eq!(header.data_len(), 0);
    }

    #[test]
    fn test_command_header_decoding_lpm_fw_update_request() {
        let bytes = [CommandType::LpmFwUpdateRequest as u8, 0x00];
        let (header, consumed): (CommandHeader, usize) =
            decode_from_slice(&bytes, standard().with_fixed_int_encoding()).unwrap();
        assert_eq!(consumed, 2);
        assert_eq!(header.command(), CommandType::LpmFwUpdateRequest);
        assert_eq!(header.data_len(), 0);
    }

    #[test]
    fn test_command_header_decoding_security_request() {
        let bytes = [CommandType::SecurityRequest as u8, 0x00];
        let (header, consumed): (CommandHeader, usize) =
            decode_from_slice(&bytes, standard().with_fixed_int_encoding()).unwrap();
        assert_eq!(consumed, 2);
        assert_eq!(header.command(), CommandType::SecurityRequest);
        assert_eq!(header.data_len(), 0);
    }

    #[test]
    fn test_command_header_decoding_set_retimer_mode() {
        let bytes = [CommandType::SetRetimerMode as u8, 0x00];
        let (header, consumed): (CommandHeader, usize) =
            decode_from_slice(&bytes, standard().with_fixed_int_encoding()).unwrap();
        assert_eq!(consumed, 2);
        assert_eq!(header.command(), CommandType::SetRetimerMode);
        assert_eq!(header.data_len(), 0);
    }

    #[test]
    fn test_command_header_decoding_set_sink_path() {
        let bytes = [CommandType::SetSinkPath as u8, 0x00];
        let (header, consumed): (CommandHeader, usize) =
            decode_from_slice(&bytes, standard().with_fixed_int_encoding()).unwrap();
        assert_eq!(consumed, 2);
        assert_eq!(header.command(), CommandType::SetSinkPath);
        assert_eq!(header.data_len(), 0);
    }

    #[test]
    fn test_command_header_decoding_set_pdos() {
        let bytes = [CommandType::SetPdos as u8, 0x00];
        let (header, consumed): (CommandHeader, usize) =
            decode_from_slice(&bytes, standard().with_fixed_int_encoding()).unwrap();
        assert_eq!(consumed, 2);
        assert_eq!(header.command(), CommandType::SetPdos);
        assert_eq!(header.data_len(), 0);
    }

    #[test]
    fn test_command_header_decoding_read_power_level() {
        let bytes = [CommandType::ReadPowerLevel as u8, 0x00];
        let (header, consumed): (CommandHeader, usize) =
            decode_from_slice(&bytes, standard().with_fixed_int_encoding()).unwrap();
        assert_eq!(consumed, 2);
        assert_eq!(header.command(), CommandType::ReadPowerLevel);
        assert_eq!(header.data_len(), 0);
    }

    #[test]
    fn test_command_header_decoding_chunking_support() {
        let bytes = [CommandType::ChunkingSupport as u8, 0x00];
        let (header, consumed): (CommandHeader, usize) =
            decode_from_slice(&bytes, standard().with_fixed_int_encoding()).unwrap();
        assert_eq!(consumed, 2);
        assert_eq!(header.command(), CommandType::ChunkingSupport);
        assert_eq!(header.data_len(), 0);
    }

    #[test]
    fn test_command_header_decoding_set_usb() {
        let bytes = [CommandType::SetUsb as u8, 0x00];
        let (header, consumed): (CommandHeader, usize) =
            decode_from_slice(&bytes, standard().with_fixed_int_encoding()).unwrap();
        assert_eq!(consumed, 2);
        assert_eq!(header.command(), CommandType::SetUsb);
        assert_eq!(header.data_len(), 0);
    }

    #[test]
    fn test_command_header_decoding_get_lpm_ppm_info() {
        let bytes = [CommandType::GetLpmPpmInfo as u8, 0x00];
        let (header, consumed): (CommandHeader, usize) =
            decode_from_slice(&bytes, standard().with_fixed_int_encoding()).unwrap();
        assert_eq!(consumed, 2);
        assert_eq!(header.command(), CommandType::GetLpmPpmInfo);
        assert_eq!(header.data_len(), 0);
    }
}

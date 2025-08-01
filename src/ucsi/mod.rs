//! Ucsi types, see spec at https://www.usb.org/document-library/usb-type-cr-connector-system-software-interface-ucsi-specification
#![allow(missing_docs)]

use bincode::de::{Decode, Decoder};
use bincode::enc::{Encode, Encoder};
use bincode::error::{AllowedEnumVariants, DecodeError, EncodeError};
use bitfield::bitfield;

use crate::PdError;

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
            0x17 => Ok(CommandType::GetCamCs),
            0x18 => Ok(CommandType::LpmFwUpdateRequest),
            0x19 => Ok(CommandType::SecurityRequest),
            0x1A => Ok(CommandType::SetRetimerMode),
            0x1B => Ok(CommandType::SetSinkPath),
            0x1C => Ok(CommandType::SetPdos),
            0x1D => Ok(CommandType::ReadPowerLevel),
            0x1E => Ok(CommandType::ChunkingSupport),
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
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Command {
    PpmCommand(ppm::Command),
    LpmCommand(lpm::Command),
}

impl Command {
    /// Returns the command type for this command
    pub const fn command_type(&self) -> CommandType {
        match self {
            Command::PpmCommand(cmd) => cmd.command_type(),
            Command::LpmCommand(cmd) => cmd.command_type(),
        }
    }
}

/// UCSI command response data
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ResponseData {
    PpmResponse(ppm::Response),
    LpmResponse(lpm::Response),
}

/// UCSI command response
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Response {
    /// CCI is produced by every command
    pub cci: cci::Cci,
    /// Response data for the command
    pub data: Option<ResponseData>,
}

impl From<cci::Cci> for Response {
    fn from(cci: cci::Cci) -> Self {
        Self { cci, data: None }
    }
}

bitfield! {
    /// Common header shared by all UCSI commands
    #[derive(Copy, Clone)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub(self) struct CommandHeaderRaw(u16);
    impl Debug;

    /// Command
    pub u8, command, set_command: 0, 7;
    /// Data length
    pub u8, data_len, set_data_len: 8, 15;
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

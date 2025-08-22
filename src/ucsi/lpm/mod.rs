use bincode::de::Decoder;
use bincode::enc::Encoder;
use bincode::error::{AllowedEnumVariants, DecodeError, EncodeError};
use bincode::{Decode, Encode};
use bitfield::bitfield;

use crate::ucsi::{cci, CommandHeader, CommandType};
use crate::{GlobalPortId, LocalPortId, PortId};

pub mod get_connector_capability;
pub mod get_connector_status;

/// Connector reset types
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ResetType {
    Hard,
    Data,
}

/// LPM command data
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CommandData {
    ConnectorReset(ResetType),
    GetConnectorStatus,
    GetConnectorCapability,
}

impl CommandData {
    /// Returns the command type for this command
    pub const fn command_type(&self) -> CommandType {
        match self {
            CommandData::ConnectorReset(_) => CommandType::ConnectorReset,
            CommandData::GetConnectorStatus => CommandType::GetConnectorStatus,
            CommandData::GetConnectorCapability => CommandType::GetConnectorCapability,
        }
    }
}

/// LPM commands
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Command<T: PortId> {
    pub port: T,
    pub operation: CommandData,
}

impl<T: PortId> Command<T> {
    /// Returns the command type for this command
    pub const fn command_type(&self) -> CommandType {
        self.operation.command_type()
    }
}

impl<T: PortId> Encode for Command<T> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        CommandHeader::new(self.command_type(), 0).encode(encoder)?;
        match self.operation {
            CommandData::GetConnectorStatus => {
                let raw_port: u8 = self.port.into();
                raw_port.encode(encoder)?;
                get_connector_status::Args.encode(encoder)
            }
            CommandData::GetConnectorCapability => {
                let raw_port: u8 = self.port.into();
                raw_port.encode(encoder)?;
                get_connector_capability::Args.encode(encoder)
            }
            _ => Err(EncodeError::Other("Unsupported command")),
        }
    }
}

impl<T: PortId> Decode<CommandHeader> for Command<T> {
    fn decode<D: Decoder<Context = CommandHeader>>(decoder: &mut D) -> Result<Self, DecodeError> {
        match decoder.context().command() {
            CommandType::GetConnectorStatus => {
                let connector_number = ConnectorNumberRaw::decode(decoder)?.connector_number();
                // Don't actually have any args, but need to consume command padding
                let _args = get_connector_status::Args::decode(decoder)?;
                Ok(Command {
                    port: From::from(connector_number),
                    operation: CommandData::GetConnectorStatus,
                })
            }
            CommandType::GetConnectorCapability => {
                let connector_number = ConnectorNumberRaw::decode(decoder)?.connector_number();
                // Don't actually have any args, but need to consume command padding
                let _args = get_connector_capability::Args::decode(decoder)?;
                Ok(Command {
                    port: From::from(connector_number),
                    operation: CommandData::GetConnectorCapability,
                })
            }
            command_type => Err(DecodeError::UnexpectedVariant {
                type_name: "CommandType",
                allowed: &AllowedEnumVariants::Allowed(&[CommandType::GetConnectorStatus as u32]),
                found: command_type as u32,
            }),
        }
    }
}

impl<T: PortId> Decode<()> for Command<T> {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, DecodeError> {
        let header = CommandHeader::decode(decoder)?;
        Command::decode(&mut decoder.with_context(header))
    }
}

pub type GlobalCommand = Command<GlobalPortId>;
pub type LocalCommand = Command<LocalPortId>;

/// LPM response data
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ResponseData {
    GetConnectorStatus(get_connector_status::ResponseData),
    GetConnectorCapability(get_connector_capability::ResponseData),
}

impl Encode for ResponseData {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        match self {
            ResponseData::GetConnectorStatus(data) => data.encode(encoder),
            ResponseData::GetConnectorCapability(data) => data.encode(encoder),
        }
    }
}

impl Decode<CommandType> for ResponseData {
    fn decode<D: Decoder<Context = CommandType>>(decoder: &mut D) -> Result<Self, DecodeError> {
        match decoder.context() {
            CommandType::GetConnectorStatus => Ok(ResponseData::GetConnectorStatus(
                get_connector_status::ResponseData::decode(decoder)?,
            )),
            CommandType::GetConnectorCapability => Ok(ResponseData::GetConnectorCapability(
                get_connector_capability::ResponseData::decode(decoder)?,
            )),
            command_type => Err(DecodeError::UnexpectedVariant {
                type_name: "CommandType",
                allowed: &AllowedEnumVariants::Allowed(&[CommandType::GetConnectorStatus as u32]),
                found: *command_type as u32,
            }),
        }
    }
}

/// LPM command response
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Response<T: PortId> {
    /// CCI is produced by every command
    pub cci: cci::Cci<T>,
    /// Response data for the command
    pub data: Option<ResponseData>,
}

pub type GlobalResponse = Response<GlobalPortId>;
pub type LocalResponse = Response<LocalPortId>;

bitfield! {
    /// Raw connector number
    #[derive(Copy, Clone, Default)]
    pub(self) struct ConnectorNumberRaw(u8);
    impl Debug;

    // Connector number
    pub u8, connector_number, set_connector_number: 6, 0;
    // Only 7-bits used for the connector number, some commands use this bit as part of their arguments
    pub bool, high_bit, set_high_bit: 7;
}

impl Encode for ConnectorNumberRaw {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.0.encode(encoder)
    }
}

impl Decode<CommandHeader> for ConnectorNumberRaw {
    fn decode<D: Decoder<Context = CommandHeader>>(decoder: &mut D) -> Result<Self, DecodeError> {
        let raw = u8::decode(decoder)?;
        Ok(ConnectorNumberRaw(raw))
    }
}

#[cfg(test)]
mod tests {
    use bincode::config::standard;
    use bincode::decode_from_slice;

    use super::*;
    use crate::ucsi::COMMAND_LEN;

    #[test]
    fn test_decode_get_connector_status() {
        let mut bytes = [0u8; COMMAND_LEN];
        bytes[0] = CommandType::GetConnectorStatus as u8;
        bytes[2] = 0x1;

        let (get_connector_status, consumed): (GlobalCommand, usize) =
            decode_from_slice(&bytes, standard().with_fixed_int_encoding()).unwrap();
        assert_eq!(consumed, bytes.len());
        assert_eq!(
            get_connector_status,
            GlobalCommand {
                port: GlobalPortId(1),
                operation: CommandData::GetConnectorStatus,
            }
        );
    }

    #[test]
    fn test_decode_get_connector_capability() {
        let mut bytes = [0u8; COMMAND_LEN];
        bytes[0] = CommandType::GetConnectorCapability as u8;
        bytes[2] = 0x1;

        let (get_connector_capability, consumed): (GlobalCommand, usize) =
            decode_from_slice(&bytes, standard().with_fixed_int_encoding()).unwrap();
        assert_eq!(consumed, bytes.len());
        assert_eq!(
            get_connector_capability,
            GlobalCommand {
                port: GlobalPortId(1),
                operation: CommandData::GetConnectorCapability,
            }
        );
    }
}

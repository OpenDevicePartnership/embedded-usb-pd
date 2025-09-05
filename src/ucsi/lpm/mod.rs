use bincode::de::Decoder;
use bincode::enc::Encoder;
use bincode::error::{AllowedEnumVariants, DecodeError, EncodeError};
use bincode::{Decode, Encode};
use bitfield::bitfield;

use crate::ucsi::{cci, CommandHeader, CommandType};
use crate::{GlobalPortId, LocalPortId, PortId};

pub mod get_connector_capability;
pub mod get_connector_status;
pub mod get_error_status;
pub mod set_ccom;
pub mod set_new_cam;
pub mod set_pdr;
pub mod set_power_level;
pub mod set_uor;

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
    SetPowerLevel(set_power_level::Args),
    SetNewCam(set_new_cam::Args),
    GetErrorStatus,
    SetCcom(set_ccom::Args),
    SetUor(set_uor::Args),
    SetPdr(set_pdr::Args),
}

impl CommandData {
    /// Returns the command type for this command
    pub const fn command_type(&self) -> CommandType {
        match self {
            CommandData::ConnectorReset(_) => CommandType::ConnectorReset,
            CommandData::GetConnectorStatus => CommandType::GetConnectorStatus,
            CommandData::GetConnectorCapability => CommandType::GetConnectorCapability,
            CommandData::SetPowerLevel(_) => CommandType::SetPowerLevel,
            CommandData::SetNewCam(_) => CommandType::SetNewCam,
            CommandData::GetErrorStatus => CommandType::GetErrorStatus,
            CommandData::SetCcom(_) => CommandType::SetCcom,
            CommandData::SetUor(_) => CommandType::SetUor,
            CommandData::SetPdr(_) => CommandType::SetPdr,
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
            CommandData::SetPowerLevel(args) => {
                // The connector number for this command is combined with its arguments, let it handle everything
                args.encode(encoder)
            }
            CommandData::SetNewCam(args) => {
                // The connector number for this command is combined with its arguments, let it handle everything
                args.encode(encoder)
            }
            CommandData::GetErrorStatus => {
                let raw_port: u8 = self.port.into();
                raw_port.encode(encoder)?;
                get_error_status::Args.encode(encoder)
            }
            CommandData::SetCcom(args) => {
                // The connector number for this command is combined with its arguments, let it handle everything
                args.encode(encoder)
            }
            CommandData::SetUor(args) => {
                // The connector number for this command is combined with its arguments, let it handle everything
                args.encode(encoder)
            }
            CommandData::SetPdr(args) => {
                // The connector number for this command is combined with its arguments, let it handle everything
                args.encode(encoder)
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
            CommandType::SetPowerLevel => {
                // The connector number is combined with arguments, let it handle everything
                let args = set_power_level::Args::decode(decoder)?;
                Ok(Command {
                    port: From::from(args.connector_number()),
                    operation: CommandData::SetPowerLevel(args),
                })
            }
            CommandType::SetNewCam => {
                // The connector number is combined with arguments, let it handle everything
                let args = set_new_cam::Args::decode(decoder)?;
                Ok(Command {
                    port: From::from(args.connector_number),
                    operation: CommandData::SetNewCam(args),
                })
            }
            CommandType::GetErrorStatus => {
                let connector_number = ConnectorNumberRaw::decode(decoder)?.connector_number();
                // Don't actually have any args, but need to consume command padding
                let _args = get_error_status::Args::decode(decoder)?;
                Ok(Command {
                    port: From::from(connector_number),
                    operation: CommandData::GetErrorStatus,
                })
            }
            CommandType::SetCcom => {
                // The connector number is combined with arguments, let it handle everything
                let args = set_ccom::Args::decode(decoder)?;
                Ok(Command {
                    port: From::from(args.connector_number()),
                    operation: CommandData::SetCcom(args),
                })
            }
            CommandType::SetUor => {
                // The connector number is combined with arguments, let it handle everything
                let args = set_uor::Args::decode(decoder)?;
                Ok(Command {
                    port: From::from(args.connector_number()),
                    operation: CommandData::SetUor(args),
                })
            }
            CommandType::SetPdr => {
                // The connector number is combined with arguments, let it handle everything
                let args = set_pdr::Args::decode(decoder)?;
                Ok(Command {
                    port: From::from(args.connector_number()),
                    operation: CommandData::SetPdr(args),
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
    GetErrorStatus(get_error_status::ResponseData),
}

impl Encode for ResponseData {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        match self {
            ResponseData::GetConnectorStatus(data) => data.encode(encoder),
            ResponseData::GetConnectorCapability(data) => data.encode(encoder),
            ResponseData::GetErrorStatus(data) => data.encode(encoder),
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
            CommandType::GetErrorStatus => Ok(ResponseData::GetErrorStatus(get_error_status::ResponseData::decode(
                decoder,
            )?)),
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
    use crate::PowerRole;

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

    #[test]
    fn test_decode_set_power_level() {
        let mut bytes = [0u8; COMMAND_LEN];
        bytes[0] = CommandType::SetPowerLevel as u8;
        bytes[2] = 0x81;

        let (set_power_level, consumed): (GlobalCommand, usize) =
            decode_from_slice(&bytes, standard().with_fixed_int_encoding()).unwrap();
        assert_eq!(consumed, bytes.len());
        assert_eq!(
            set_power_level,
            GlobalCommand {
                port: GlobalPortId(1),
                operation: CommandData::SetPowerLevel(
                    *set_power_level::Args::default()
                        .set_connector_number(1)
                        .set_power_role(PowerRole::Source)
                ),
            }
        );
    }

    fn test_decode_set_ccom() {
        let mut bytes = [0u8; COMMAND_LEN];
        bytes[0] = CommandType::SetCcom as u8;
        bytes[2] = 0x81;

        let (set_ccom, consumed): (GlobalCommand, usize) =
            decode_from_slice(&bytes, standard().with_fixed_int_encoding()).unwrap();
        assert_eq!(consumed, bytes.len());
        assert_eq!(
            set_ccom,
            GlobalCommand {
                port: GlobalPortId(1),
                operation: CommandData::SetCcom(*set_ccom::Args::default().set_connector_number(1).set_rp(true)),
            }
        );
    }

    #[test]
    fn test_decode_set_new_cam() {
        let mut bytes = [0u8; COMMAND_LEN];
        bytes[0] = CommandType::SetNewCam as u8;
        bytes[2] = 0x1;

        let (set_new_cam, consumed): (GlobalCommand, usize) =
            decode_from_slice(&bytes, standard().with_fixed_int_encoding()).unwrap();
        assert_eq!(consumed, bytes.len());
        assert_eq!(
            set_new_cam,
            GlobalCommand {
                port: GlobalPortId(1),
                operation: CommandData::SetNewCam(set_new_cam::Args {
                    connector_number: 1,
                    enter: false,
                    am_offset: 0,
                    am_specific: 0,
                }),
            }
        );
    }

    fn test_decode_set_uor() {
        let mut bytes = [0u8; COMMAND_LEN];
        bytes[0] = CommandType::SetUor as u8;
        bytes[2] = 0x81;

        let (set_uor, consumed): (GlobalCommand, usize) =
            decode_from_slice(&bytes, standard().with_fixed_int_encoding()).unwrap();
        assert_eq!(consumed, bytes.len());
        assert_eq!(
            set_uor,
            GlobalCommand {
                port: GlobalPortId(1),
                operation: CommandData::SetUor(*set_uor::Args::default().set_connector_number(1).set_dfp(true)),
            }
        );
    }

    #[test]
    fn test_decode_get_error_status() {
        let mut bytes = [0u8; COMMAND_LEN];
        bytes[0] = CommandType::GetErrorStatus as u8;
        bytes[2] = 0x1;

        let (get_error_status, consumed): (GlobalCommand, usize) =
            decode_from_slice(&bytes, standard().with_fixed_int_encoding()).unwrap();
        assert_eq!(consumed, bytes.len());
        assert_eq!(
            get_error_status,
            GlobalCommand {
                port: GlobalPortId(1),
                operation: CommandData::GetErrorStatus,
            }
        );
    }

    fn test_decode_set_pdr() {
        let mut bytes = [0u8; COMMAND_LEN];
        bytes[0] = CommandType::SetPdr as u8;
        bytes[2] = 0x81;

        let (set_pdr, consumed): (GlobalCommand, usize) =
            decode_from_slice(&bytes, standard().with_fixed_int_encoding()).unwrap();
        assert_eq!(consumed, bytes.len());
        assert_eq!(
            set_pdr,
            GlobalCommand {
                port: GlobalPortId(1),
                operation: CommandData::SetPdr(*set_pdr::Args::default().set_connector_number(1).set_swap_source(true)),
            }
        );
    }
}

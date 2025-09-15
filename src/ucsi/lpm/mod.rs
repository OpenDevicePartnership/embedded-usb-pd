use bincode::de::Decoder;
use bincode::enc::Encoder;
use bincode::error::{AllowedEnumVariants, DecodeError, EncodeError};
use bincode::{Decode, Encode};
use bitfield::bitfield;

use crate::ucsi::{cci, CommandHeader, CommandType};
use crate::{GlobalPortId, LocalPortId, PortId};

pub mod connector_reset;
pub mod get_alternate_modes;
pub mod get_cable_property;
pub mod get_cam_supported;
pub mod get_connector_capability;
pub mod get_connector_status;
pub mod get_current_cam;
pub mod get_error_status;
pub mod get_pdos;
pub mod set_ccom;
pub mod set_new_cam;
pub mod set_pdr;
pub mod set_power_level;
pub mod set_uor;

/// LPM command data
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CommandData {
    ConnectorReset,
    GetConnectorStatus,
    GetConnectorCapability,
    SetPowerLevel(set_power_level::Args),
    SetNewCam(set_new_cam::Args),
    GetErrorStatus,
    SetCcom(set_ccom::Args),
    SetUor(set_uor::Args),
    SetPdr(set_pdr::Args),
    GetAlternateModes(get_alternate_modes::Args),
    GetCamSupported,
    GetCurrentCam,
    GetPdos(get_pdos::Args),
    GetCableProperty,
}

impl CommandData {
    /// Returns the command type for this command
    pub const fn command_type(&self) -> CommandType {
        match self {
            CommandData::ConnectorReset => CommandType::ConnectorReset,
            CommandData::GetConnectorStatus => CommandType::GetConnectorStatus,
            CommandData::GetConnectorCapability => CommandType::GetConnectorCapability,
            CommandData::SetPowerLevel(_) => CommandType::SetPowerLevel,
            CommandData::SetNewCam(_) => CommandType::SetNewCam,
            CommandData::GetErrorStatus => CommandType::GetErrorStatus,
            CommandData::SetCcom(_) => CommandType::SetCcom,
            CommandData::SetUor(_) => CommandType::SetUor,
            CommandData::SetPdr(_) => CommandType::SetPdr,
            CommandData::GetAlternateModes(_) => CommandType::GetAlternateModes,
            CommandData::GetCamSupported => CommandType::GetCamSupported,
            CommandData::GetCurrentCam => CommandType::GetCurrentCam,
            CommandData::GetPdos(_) => CommandType::GetPdos,
            CommandData::GetCableProperty => CommandType::GetCableProperty,
        }
    }
}

/// LPM commands
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Command<T: PortId> {
    port: T,
    operation: CommandData,
}

impl<T: PortId> Command<T> {
    pub const fn new(port: T, operation: CommandData) -> Self {
        Command { port, operation }
    }

    pub fn port(&self) -> T {
        self.port
    }

    pub fn set_port(&mut self, port: T) -> &mut Self {
        self.port = port;
        // These commands have the connector number as part of their arguments, update them too
        // TODO: Figure out how to remove this
        match self.operation {
            CommandData::SetPowerLevel(ref mut args) => {
                args.set_connector_number(self.port.into());
            }
            CommandData::SetNewCam(ref mut args) => {
                args.connector_number = self.port.into();
            }
            CommandData::SetCcom(ref mut args) => {
                args.set_connector_number(self.port.into());
            }
            CommandData::SetUor(ref mut args) => {
                args.set_connector_number(self.port.into());
            }
            CommandData::SetPdr(ref mut args) => {
                args.set_connector_number(self.port.into());
            }
            CommandData::GetAlternateModes(ref mut args) => {
                args.set_connector_number(self.port.into());
            }
            CommandData::GetPdos(ref mut args) => {
                args.set_connector_number(self.port.into());
            }
            _ => {}
        }

        self
    }

    pub fn operation(&self) -> CommandData {
        self.operation
    }

    pub fn set_operation(&mut self, operation: CommandData) -> &mut Self {
        self.operation = operation;
        self
    }
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
        let raw_port: u8 = self.port.into();
        match self.operation {
            CommandData::ConnectorReset => {
                raw_port.encode(encoder)?;
                connector_reset::Args.encode(encoder)
            }
            CommandData::GetConnectorStatus => {
                raw_port.encode(encoder)?;
                get_connector_status::Args.encode(encoder)
            }
            CommandData::GetConnectorCapability => {
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
            CommandData::GetAlternateModes(args) => {
                // This command has a different format without a leading port number
                // TODO: Figure out if this can stay an exception or if each command is responsible for pulling its port number.
                args.encode(encoder)
            }
            CommandData::GetCamSupported => {
                raw_port.encode(encoder)?;
                get_cam_supported::Args.encode(encoder)
            }
            CommandData::GetCurrentCam => {
                raw_port.encode(encoder)?;
                get_current_cam::Args.encode(encoder)
            }
            CommandData::GetPdos(args) => {
                // The connector number for this command is combined with its arguments, let it handle everything
                args.encode(encoder)
            }
            CommandData::GetCableProperty => {
                raw_port.encode(encoder)?;
                get_cable_property::Args.encode(encoder)
            }
        }
    }
}

impl<T: PortId> Decode<CommandHeader> for Command<T> {
    fn decode<D: Decoder<Context = CommandHeader>>(decoder: &mut D) -> Result<Self, DecodeError> {
        match decoder.context().command() {
            CommandType::ConnectorReset => {
                let connector_number = ConnectorNumberRaw::decode(decoder)?.connector_number();
                // Don't actually have any args, but need to consume command padding
                let _args = connector_reset::Args::decode(decoder)?;
                Ok(Command {
                    port: From::from(connector_number),
                    operation: CommandData::ConnectorReset,
                })
            }
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
            CommandType::GetAlternateModes => {
                // This command has a different format without a leading port number
                let args = get_alternate_modes::Args::decode(decoder)?;
                Ok(Command {
                    port: From::from(args.connector_number()),
                    operation: CommandData::GetAlternateModes(args),
                })
            }
            CommandType::GetCamSupported => {
                let connector_number = ConnectorNumberRaw::decode(decoder)?.connector_number();
                // Don't actually have any args, but need to consume command padding
                let _args = get_cam_supported::Args::decode(decoder)?;
                Ok(Command {
                    port: From::from(connector_number),
                    operation: CommandData::GetCamSupported,
                })
            }
            CommandType::GetCurrentCam => {
                let connector_number = ConnectorNumberRaw::decode(decoder)?.connector_number();
                // Don't actually have any args, but need to consume command padding
                let _args = get_current_cam::Args::decode(decoder)?;
                Ok(Command {
                    port: From::from(connector_number),
                    operation: CommandData::GetCurrentCam,
                })
            }
            CommandType::GetPdos => {
                // The connector number is combined with arguments, let it handle everything
                let args = get_pdos::Args::decode(decoder)?;
                Ok(Command {
                    port: From::from(args.connector_number()),
                    operation: CommandData::GetPdos(args),
                })
            }
            CommandType::GetCableProperty => {
                let connector_number = ConnectorNumberRaw::decode(decoder)?.connector_number();
                // Don't actually have any args, but need to consume command padding
                let _args = get_cable_property::Args::decode(decoder)?;
                Ok(Command {
                    port: From::from(connector_number),
                    operation: CommandData::GetCableProperty,
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
    ConnectorReset,
    GetConnectorStatus(get_connector_status::ResponseData),
    GetConnectorCapability(get_connector_capability::ResponseData),
    GetErrorStatus(get_error_status::ResponseData),
    GetAlternateModes(get_alternate_modes::ResponseData),
    GetCamSupported(get_cam_supported::ResponseData),
    GetCurrentCam(get_current_cam::ResponseData),
    GetPdos(get_pdos::ResponseData),
    GetCableProperty(get_cable_property::ResponseData),
}

impl Encode for ResponseData {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        match self {
            ResponseData::ConnectorReset => Ok(()), // No response data
            ResponseData::GetConnectorStatus(data) => data.encode(encoder),
            ResponseData::GetConnectorCapability(data) => data.encode(encoder),
            ResponseData::GetErrorStatus(data) => data.encode(encoder),
            ResponseData::GetAlternateModes(data) => data.encode(encoder),
            ResponseData::GetCamSupported(data) => data.encode(encoder),
            ResponseData::GetCurrentCam(data) => data.encode(encoder),
            ResponseData::GetPdos(data) => data.encode(encoder),
            ResponseData::GetCableProperty(data) => data.encode(encoder),
        }
    }
}

impl Decode<CommandType> for ResponseData {
    fn decode<D: Decoder<Context = CommandType>>(decoder: &mut D) -> Result<Self, DecodeError> {
        match decoder.context() {
            CommandType::ConnectorReset => Ok(ResponseData::ConnectorReset),
            CommandType::GetConnectorStatus => Ok(ResponseData::GetConnectorStatus(
                get_connector_status::ResponseData::decode(decoder)?,
            )),
            CommandType::GetConnectorCapability => Ok(ResponseData::GetConnectorCapability(
                get_connector_capability::ResponseData::decode(decoder)?,
            )),
            CommandType::GetErrorStatus => Ok(ResponseData::GetErrorStatus(get_error_status::ResponseData::decode(
                decoder,
            )?)),
            CommandType::GetAlternateModes => Ok(ResponseData::GetAlternateModes(
                get_alternate_modes::ResponseData::decode(decoder)?,
            )),
            CommandType::GetCamSupported => Ok(ResponseData::GetCamSupported(get_cam_supported::ResponseData::decode(
                decoder,
            )?)),
            CommandType::GetCurrentCam => Ok(ResponseData::GetCurrentCam(get_current_cam::ResponseData::decode(
                decoder,
            )?)),
            CommandType::GetPdos => Ok(ResponseData::GetPdos(get_pdos::ResponseData::decode(decoder)?)),
            CommandType::GetCableProperty => Ok(ResponseData::GetCableProperty(
                get_cable_property::ResponseData::decode(decoder)?,
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

/// Common recipient type used by multiple commands
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Recipient {
    /// Connector
    Connector,
    /// SOP
    Sop,
    /// SOP'
    SopP,
    /// SOP''
    SopPp,
}

/// Invalid recipient error, contains the invalid recipient value
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct InvalidRecipient(pub u8);

impl TryFrom<u8> for Recipient {
    type Error = InvalidRecipient;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x0 => Ok(Recipient::Connector),
            0x1 => Ok(Recipient::Sop),
            0x2 => Ok(Recipient::SopP),
            0x3 => Ok(Recipient::SopPp),
            v => Err(InvalidRecipient(v)),
        }
    }
}

impl From<Recipient> for u8 {
    fn from(value: Recipient) -> Self {
        match value {
            Recipient::Connector => 0x0,
            Recipient::Sop => 0x1,
            Recipient::SopP => 0x2,
            Recipient::SopPp => 0x3,
        }
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
    fn test_decode_connector_reset() {
        let mut bytes = [0u8; COMMAND_LEN];
        bytes[0] = CommandType::ConnectorReset as u8;
        bytes[2] = 0x81;

        let (connector_reset, consumed): (GlobalCommand, usize) =
            decode_from_slice(&bytes, standard().with_fixed_int_encoding()).unwrap();
        assert_eq!(consumed, bytes.len());
        assert_eq!(
            connector_reset,
            GlobalCommand {
                port: GlobalPortId(1),
                operation: CommandData::ConnectorReset,
            }
        );
    }

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
                )
            }
        )
    }

    #[test]
    fn test_decode_get_alternate_modes() {
        let mut bytes = [0u8; COMMAND_LEN];
        bytes[0] = CommandType::GetAlternateModes as u8;
        bytes[2] = 0x1; // SOP recipient

        let (get_alternate_modes, consumed): (GlobalCommand, usize) =
            decode_from_slice(&bytes, standard().with_fixed_int_encoding()).unwrap();
        assert_eq!(consumed, bytes.len());
        assert_eq!(
            get_alternate_modes,
            GlobalCommand {
                port: GlobalPortId(0),
                operation: CommandData::GetAlternateModes(
                    *get_alternate_modes::Args::default().set_recipient(Recipient::Sop)
                ),
            }
        );
    }

    #[test]
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

    #[test]
    fn test_decode_set_uor() {
        let mut bytes = [9, 0, 129, 0, 0, 0, 0, 0];

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

    #[test]
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

    #[test]
    fn test_get_cam_supported() {
        let mut bytes = [0u8; COMMAND_LEN];
        bytes[0] = CommandType::GetCamSupported as u8;
        bytes[2] = 0x1;

        let (get_cam_supported, consumed): (GlobalCommand, usize) =
            decode_from_slice(&bytes, standard().with_fixed_int_encoding()).unwrap();
        assert_eq!(consumed, bytes.len());
        assert_eq!(
            get_cam_supported,
            GlobalCommand {
                port: GlobalPortId(1),
                operation: CommandData::GetCamSupported,
            }
        );
    }

    #[test]
    fn test_get_current_cam() {
        let mut bytes = [0u8; COMMAND_LEN];
        bytes[0] = CommandType::GetCurrentCam as u8;
        bytes[2] = 0x1;
        let (get_current_cam, consumed): (GlobalCommand, usize) =
            decode_from_slice(&bytes, standard().with_fixed_int_encoding()).unwrap();
        assert_eq!(consumed, bytes.len());
        assert_eq!(
            get_current_cam,
            GlobalCommand {
                port: GlobalPortId(1),
                operation: CommandData::GetCurrentCam,
            }
        );
    }

    #[test]
    fn test_get_pdos() {
        let mut bytes = [0u8; COMMAND_LEN];
        bytes[0] = CommandType::GetPdos as u8;
        bytes[2] = 0x81;

        let (get_pdos, consumed): (GlobalCommand, usize) =
            decode_from_slice(&bytes, standard().with_fixed_int_encoding()).unwrap();
        assert_eq!(consumed, bytes.len());
        assert_eq!(
            get_pdos,
            GlobalCommand {
                port: GlobalPortId(1),
                operation: CommandData::GetPdos(*get_pdos::Args::default().set_connector_number(1).set_partner(true)),
            }
        );
    }

    #[test]
    fn test_get_cable_property() {
        let mut bytes = [0u8; COMMAND_LEN];
        bytes[0] = CommandType::GetCableProperty as u8;
        bytes[2] = 0x1;

        let (get_cable_property, consumed): (GlobalCommand, usize) =
            decode_from_slice(&bytes, standard().with_fixed_int_encoding()).unwrap();
        assert_eq!(consumed, bytes.len());
        assert_eq!(
            get_cable_property,
            GlobalCommand {
                port: GlobalPortId(1),
                operation: CommandData::GetCableProperty,
            }
        );
    }
}

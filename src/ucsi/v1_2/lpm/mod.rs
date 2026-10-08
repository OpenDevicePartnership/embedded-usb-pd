use bitfield::bitfield;

use crate::ucsi::v1_2::{cci, CommandHeaderRaw, CommandRaw, CommandType, InvalidCommandType, COMMAND_LEN};
use crate::{GlobalPortId, LocalPortId, PortId};

pub mod connector_reset;
pub mod get_alternate_modes;
pub mod get_cable_property;
pub mod get_cam_supported;
pub mod get_connector_capability;
pub mod get_connector_status;
pub mod get_current_cam;
pub mod get_error_status;
pub mod get_pd_message;
pub mod get_pdos;
pub mod set_ccom;
pub mod set_new_cam;
pub mod set_pdr;
pub mod set_power_level;
pub mod set_uor;

/// Length of an LPM command payload, the command minus its header
pub const COMMAND_PAYLOAD_LEN: usize = COMMAND_LEN - size_of::<CommandHeaderRaw>();

/// Copies `src` into a new buffer, truncating or zero-padding it as needed
fn resize<const SRC: usize, const DST: usize>(src: [u8; SRC]) -> [u8; DST] {
    let mut dst = [0u8; DST];
    dst.iter_mut().zip(src.iter()).for_each(|(dst, src)| *dst = *src);
    dst
}

/// Error returned when a command cannot be converted to or from its payload
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum InvalidCommand {
    /// Not an LPM command
    InvalidCommandType(InvalidCommandType),
    /// SET_POWER_LEVEL argument out of range
    Overflow(set_power_level::OverflowError),
    /// Invalid SET_POWER_LEVEL current
    InvalidCurrent(set_power_level::InvalidCurrent),
    /// Invalid GET_PDOS number of PDOs
    InvalidNumPdos(get_pdos::InvalidNumPdos),
    /// Invalid GET_PDOS source capability type
    InvalidSourceCapabilityType(get_pdos::InvalidSourceCapabilityType),
    /// Invalid recipient
    InvalidRecipient(InvalidRecipient),
    /// Invalid GET_PD_MESSAGE arguments
    InvalidPdMessageArgs(get_pd_message::InvalidArgs),
}

impl From<InvalidCommandType> for InvalidCommand {
    fn from(value: InvalidCommandType) -> Self {
        InvalidCommand::InvalidCommandType(value)
    }
}

impl From<set_power_level::OverflowError> for InvalidCommand {
    fn from(value: set_power_level::OverflowError) -> Self {
        InvalidCommand::Overflow(value)
    }
}

impl From<set_power_level::InvalidCurrent> for InvalidCommand {
    fn from(value: set_power_level::InvalidCurrent) -> Self {
        InvalidCommand::InvalidCurrent(value)
    }
}

impl From<get_pdos::InvalidNumPdos> for InvalidCommand {
    fn from(value: get_pdos::InvalidNumPdos) -> Self {
        InvalidCommand::InvalidNumPdos(value)
    }
}

impl From<get_pdos::InvalidSourceCapabilityType> for InvalidCommand {
    fn from(value: get_pdos::InvalidSourceCapabilityType) -> Self {
        InvalidCommand::InvalidSourceCapabilityType(value)
    }
}

impl From<InvalidRecipient> for InvalidCommand {
    fn from(value: InvalidRecipient) -> Self {
        InvalidCommand::InvalidRecipient(value)
    }
}

impl From<get_pd_message::InvalidArgs> for InvalidCommand {
    fn from(value: get_pd_message::InvalidArgs) -> Self {
        InvalidCommand::InvalidPdMessageArgs(value)
    }
}

/// Error returned when response data cannot be reconstructed from its bytes
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum InvalidResponseData {
    /// Not an LPM command
    InvalidCommandType(InvalidCommandType),
    /// Invalid GET_CONNECTOR_STATUS response data
    ConnectorStatus(get_connector_status::InvalidResponseData),
}

impl From<InvalidCommandType> for InvalidResponseData {
    fn from(value: InvalidCommandType) -> Self {
        InvalidResponseData::InvalidCommandType(value)
    }
}

impl From<get_connector_status::InvalidResponseData> for InvalidResponseData {
    fn from(value: get_connector_status::InvalidResponseData) -> Self {
        InvalidResponseData::ConnectorStatus(value)
    }
}

/// LPM command data
///
/// Every LPM command targets a connector, so the connector number is carried by
/// each variant's arguments rather than alongside them.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CommandData<T: PortId> {
    ConnectorReset(connector_reset::Args<T>),
    GetConnectorStatus(get_connector_status::Args<T>),
    GetConnectorCapability(get_connector_capability::Args<T>),
    SetPowerLevel(set_power_level::Args<T>),
    SetNewCam(set_new_cam::Args<T>),
    GetErrorStatus(get_error_status::Args<T>),
    SetCcom(set_ccom::Args<T>),
    SetUor(set_uor::Args<T>),
    SetPdr(set_pdr::Args<T>),
    GetAlternateModes(get_alternate_modes::Args<T>),
    GetCamSupported(get_cam_supported::Args<T>),
    GetCurrentCam(get_current_cam::Args<T>),
    GetPdos(get_pdos::Args<T>),
    GetCableProperty(get_cable_property::Args<T>),
    GetPdMessage(get_pd_message::Args<T>),
}

impl<T: PortId> CommandData<T> {
    /// Returns the command type for this command
    pub const fn command_type(&self) -> CommandType {
        match self {
            CommandData::ConnectorReset(_) => CommandType::ConnectorReset,
            CommandData::GetConnectorStatus(_) => CommandType::GetConnectorStatus,
            CommandData::GetConnectorCapability(_) => CommandType::GetConnectorCapability,
            CommandData::SetPowerLevel(_) => CommandType::SetPowerLevel,
            CommandData::SetNewCam(_) => CommandType::SetNewCam,
            CommandData::GetErrorStatus(_) => CommandType::GetErrorStatus,
            CommandData::SetCcom(_) => CommandType::SetCcom,
            CommandData::SetUor(_) => CommandType::SetUor,
            CommandData::SetPdr(_) => CommandType::SetPdr,
            CommandData::GetAlternateModes(_) => CommandType::GetAlternateModes,
            CommandData::GetCamSupported(_) => CommandType::GetCamSupported,
            CommandData::GetCurrentCam(_) => CommandType::GetCurrentCam,
            CommandData::GetPdos(_) => CommandType::GetPdos,
            CommandData::GetCableProperty(_) => CommandType::GetCableProperty,
            CommandData::GetPdMessage(_) => CommandType::GetPdMessage,
        }
    }

    /// Returns the connector this command targets
    pub fn connector_number(&self) -> T {
        match self {
            CommandData::ConnectorReset(args) => args.connector_number,
            CommandData::GetConnectorStatus(args) => args.connector_number,
            CommandData::GetConnectorCapability(args) => args.connector_number,
            CommandData::SetPowerLevel(args) => args.connector_number,
            CommandData::SetNewCam(args) => args.connector_number,
            CommandData::GetErrorStatus(args) => args.connector_number,
            CommandData::SetCcom(args) => args.connector_number,
            CommandData::SetUor(args) => args.connector_number,
            CommandData::SetPdr(args) => args.connector_number,
            CommandData::GetAlternateModes(args) => args.connector_number,
            CommandData::GetCamSupported(args) => args.connector_number,
            CommandData::GetCurrentCam(args) => args.connector_number,
            CommandData::GetPdos(args) => args.connector_number,
            CommandData::GetCableProperty(args) => args.connector_number,
            CommandData::GetPdMessage(args) => args.connector_number,
        }
    }

    /// Sets the connector this command targets
    pub fn set_connector_number(&mut self, connector: T) -> &mut Self {
        match self {
            CommandData::ConnectorReset(args) => args.connector_number = connector,
            CommandData::GetConnectorStatus(args) => args.connector_number = connector,
            CommandData::GetConnectorCapability(args) => args.connector_number = connector,
            CommandData::SetPowerLevel(args) => args.connector_number = connector,
            CommandData::SetNewCam(args) => args.connector_number = connector,
            CommandData::GetErrorStatus(args) => args.connector_number = connector,
            CommandData::SetCcom(args) => args.connector_number = connector,
            CommandData::SetUor(args) => args.connector_number = connector,
            CommandData::SetPdr(args) => args.connector_number = connector,
            CommandData::GetAlternateModes(args) => args.connector_number = connector,
            CommandData::GetCamSupported(args) => args.connector_number = connector,
            CommandData::GetCurrentCam(args) => args.connector_number = connector,
            CommandData::GetPdos(args) => args.connector_number = connector,
            CommandData::GetCableProperty(args) => args.connector_number = connector,
            CommandData::GetPdMessage(args) => args.connector_number = connector,
        }

        self
    }
}

/// LPM commands
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Command<T: PortId> {
    operation: CommandData<T>,
}

impl<T: PortId> Command<T> {
    pub const fn new(operation: CommandData<T>) -> Self {
        Command { operation }
    }

    /// Returns the connector this command targets
    pub fn connector_number(&self) -> T {
        self.operation.connector_number()
    }

    /// Sets the connector this command targets
    pub fn set_connector_number(&mut self, connector: T) -> &mut Self {
        self.operation.set_connector_number(connector);
        self
    }

    pub fn operation(&self) -> CommandData<T> {
        self.operation
    }

    pub fn set_operation(&mut self, operation: CommandData<T>) -> &mut Self {
        self.operation = operation;
        self
    }
}

impl<T: PortId> Command<T> {
    /// Length of an LPM command payload, the command minus its header
    pub const PAYLOAD_LEN: usize = COMMAND_PAYLOAD_LEN;

    /// Returns the command type for this command
    pub const fn command_type(&self) -> CommandType {
        self.operation.command_type()
    }
}

impl<T: PortId> TryFrom<Command<T>> for CommandRaw {
    type Error = InvalidCommand;

    /// Converts a command into its raw wire format
    ///
    /// Returns an error if the arguments cannot be represented on the wire.
    fn try_from(command: Command<T>) -> Result<Self, Self::Error> {
        let payload = match command.operation {
            CommandData::ConnectorReset(args) => bytemuck::must_cast(connector_reset::ArgsRaw::from(args)),
            CommandData::GetConnectorStatus(args) => bytemuck::must_cast(get_connector_status::ArgsRaw::from(args)),
            CommandData::GetConnectorCapability(args) => {
                bytemuck::must_cast(get_connector_capability::ArgsRaw::from(args))
            }
            CommandData::SetPowerLevel(args) => bytemuck::must_cast(set_power_level::ArgsRaw::try_from(args)?),
            CommandData::SetNewCam(args) => bytemuck::must_cast(set_new_cam::ArgsRaw::from(args)),
            CommandData::GetErrorStatus(args) => bytemuck::must_cast(get_error_status::ArgsRaw::from(args)),
            CommandData::SetCcom(args) => bytemuck::must_cast(set_ccom::ArgsRaw::from(args)),
            CommandData::SetUor(args) => bytemuck::must_cast(set_uor::ArgsRaw::from(args)),
            CommandData::SetPdr(args) => bytemuck::must_cast(set_pdr::ArgsRaw::from(args)),
            CommandData::GetAlternateModes(args) => bytemuck::must_cast(get_alternate_modes::ArgsRaw::from(args)),
            CommandData::GetCamSupported(args) => bytemuck::must_cast(get_cam_supported::ArgsRaw::from(args)),
            CommandData::GetCurrentCam(args) => bytemuck::must_cast(get_current_cam::ArgsRaw::from(args)),
            CommandData::GetPdos(args) => bytemuck::must_cast(get_pdos::ArgsRaw::try_from(args)?),
            CommandData::GetCableProperty(args) => bytemuck::must_cast(get_cable_property::ArgsRaw::from(args)),
            CommandData::GetPdMessage(args) => bytemuck::must_cast(get_pd_message::ArgsRaw::from(args)),
        };

        Ok(CommandRaw {
            command: command.command_type().into(),
            // Data length is only non-zero for vendor-defined commands, none of which are modelled here
            data_len: 0,
            payload,
        })
    }
}

impl<T: PortId> TryFrom<CommandRaw> for Command<T> {
    type Error = InvalidCommand;

    /// Reconstructs a command from its raw wire format
    ///
    /// Returns an error if the command type is not an LPM command or if the
    /// payload does not contain valid arguments.
    fn try_from(raw: CommandRaw) -> Result<Self, Self::Error> {
        let payload = raw.payload;
        let operation = match CommandType::try_from(raw.command)? {
            CommandType::ConnectorReset => {
                CommandData::ConnectorReset(bytemuck::must_cast::<_, connector_reset::ArgsRaw>(payload).into())
            }
            CommandType::GetConnectorStatus => {
                CommandData::GetConnectorStatus(bytemuck::must_cast::<_, get_connector_status::ArgsRaw>(payload).into())
            }
            CommandType::GetConnectorCapability => CommandData::GetConnectorCapability(
                bytemuck::must_cast::<_, get_connector_capability::ArgsRaw>(payload).into(),
            ),
            CommandType::SetPowerLevel => {
                CommandData::SetPowerLevel(bytemuck::must_cast::<_, set_power_level::ArgsRaw>(payload).try_into()?)
            }
            CommandType::SetNewCam => {
                CommandData::SetNewCam(bytemuck::must_cast::<_, set_new_cam::ArgsRaw>(payload).into())
            }
            CommandType::GetErrorStatus => {
                CommandData::GetErrorStatus(bytemuck::must_cast::<_, get_error_status::ArgsRaw>(payload).into())
            }
            CommandType::SetCcom => CommandData::SetCcom(bytemuck::must_cast::<_, set_ccom::ArgsRaw>(payload).into()),
            CommandType::SetUor => CommandData::SetUor(bytemuck::must_cast::<_, set_uor::ArgsRaw>(payload).into()),
            CommandType::SetPdr => CommandData::SetPdr(bytemuck::must_cast::<_, set_pdr::ArgsRaw>(payload).into()),
            CommandType::GetAlternateModes => CommandData::GetAlternateModes(
                bytemuck::must_cast::<_, get_alternate_modes::ArgsRaw>(payload).try_into()?,
            ),
            CommandType::GetCamSupported => {
                CommandData::GetCamSupported(bytemuck::must_cast::<_, get_cam_supported::ArgsRaw>(payload).into())
            }
            CommandType::GetCurrentCam => {
                CommandData::GetCurrentCam(bytemuck::must_cast::<_, get_current_cam::ArgsRaw>(payload).into())
            }
            CommandType::GetPdos => {
                CommandData::GetPdos(bytemuck::must_cast::<_, get_pdos::ArgsRaw>(payload).try_into()?)
            }
            CommandType::GetCableProperty => {
                CommandData::GetCableProperty(bytemuck::must_cast::<_, get_cable_property::ArgsRaw>(payload).into())
            }
            CommandType::GetPdMessage => {
                CommandData::GetPdMessage(bytemuck::must_cast::<_, get_pd_message::ArgsRaw>(payload).try_into()?)
            }
            command_type => {
                return Err(InvalidCommand::InvalidCommandType(InvalidCommandType(
                    command_type as u8,
                )))
            }
        };

        Ok(Command::new(operation))
    }
}

pub type GlobalCommand = Command<GlobalPortId>;
pub type LocalCommand = Command<LocalPortId>;

/// Maximum length of any LPM response data
pub const RESPONSE_DATA_MAX_LEN: usize = get_current_cam::ResponseDataRaw::LEN;

/// LPM response data
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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
    GetPdMessage(get_pd_message::ResponseData),
}

impl ResponseData {
    /// Returns the command type that produces this response data
    pub const fn command_type(&self) -> CommandType {
        match self {
            ResponseData::ConnectorReset => CommandType::ConnectorReset,
            ResponseData::GetConnectorStatus(_) => CommandType::GetConnectorStatus,
            ResponseData::GetConnectorCapability(_) => CommandType::GetConnectorCapability,
            ResponseData::GetErrorStatus(_) => CommandType::GetErrorStatus,
            ResponseData::GetAlternateModes(_) => CommandType::GetAlternateModes,
            ResponseData::GetCamSupported(_) => CommandType::GetCamSupported,
            ResponseData::GetCurrentCam(_) => CommandType::GetCurrentCam,
            ResponseData::GetPdos(_) => CommandType::GetPdos,
            ResponseData::GetCableProperty(_) => CommandType::GetCableProperty,
            ResponseData::GetPdMessage(_) => CommandType::GetPdMessage,
        }
    }

    /// Number of valid bytes this response data occupies on the wire
    pub fn data_len(&self) -> usize {
        match self {
            // No response data
            ResponseData::ConnectorReset => 0,
            ResponseData::GetConnectorStatus(_) => get_connector_status::ResponseDataRaw::LEN,
            ResponseData::GetConnectorCapability(_) => get_connector_capability::ResponseDataRaw::LEN,
            ResponseData::GetErrorStatus(_) => get_error_status::ResponseDataRaw::LEN,
            ResponseData::GetAlternateModes(_) => get_alternate_modes::ResponseDataRaw::LEN,
            ResponseData::GetCamSupported(_) => get_cam_supported::ResponseDataRaw::LEN,
            ResponseData::GetCurrentCam(_) => get_current_cam::ResponseDataRaw::LEN,
            // Only the valid PDOs are sent, the response is shorter than the raw type when fewer are present
            ResponseData::GetPdos(data) => data.iter().len() * size_of::<u32>(),
            ResponseData::GetCableProperty(_) => get_cable_property::ResponseDataRaw::LEN,
            ResponseData::GetPdMessage(_) => get_pd_message::ResponseDataRaw::LEN,
        }
    }
}

impl From<ResponseData> for super::ResponseDataRaw {
    /// Converts response data into a [`ResponseDataRaw::MAX_LEN`] sized buffer
    ///
    /// Only the first [`ResponseData::data_len`] bytes are valid.
    fn from(data: ResponseData) -> Self {
        match data {
            // No response data
            ResponseData::ConnectorReset => super::ResponseDataRaw {
                payload: [0; super::ResponseDataRaw::MAX_LEN],
            },
            ResponseData::GetConnectorStatus(data) => super::ResponseDataRaw {
                payload: resize(
                    bytemuck::must_cast::<_, [u8; get_connector_status::ResponseDataRaw::LEN]>(
                        get_connector_status::ResponseDataRaw::from(data),
                    ),
                ),
            },
            ResponseData::GetConnectorCapability(data) => super::ResponseDataRaw {
                payload: resize(bytemuck::must_cast::<
                    _,
                    [u8; get_connector_capability::ResponseDataRaw::LEN],
                >(get_connector_capability::ResponseDataRaw::from(data))),
            },
            ResponseData::GetErrorStatus(data) => super::ResponseDataRaw {
                payload: resize(bytemuck::must_cast::<_, [u8; get_error_status::ResponseDataRaw::LEN]>(
                    get_error_status::ResponseDataRaw::from(data),
                )),
            },
            ResponseData::GetAlternateModes(data) => super::ResponseDataRaw {
                payload: resize(
                    bytemuck::must_cast::<_, [u8; get_alternate_modes::ResponseDataRaw::LEN]>(
                        get_alternate_modes::ResponseDataRaw::from(data),
                    ),
                ),
            },
            ResponseData::GetCamSupported(data) => super::ResponseDataRaw {
                payload: resize(bytemuck::must_cast::<_, [u8; get_cam_supported::ResponseDataRaw::LEN]>(
                    get_cam_supported::ResponseDataRaw::from(data),
                )),
            },
            ResponseData::GetCurrentCam(data) => super::ResponseDataRaw {
                payload: resize(bytemuck::must_cast::<_, [u8; get_current_cam::ResponseDataRaw::LEN]>(
                    get_current_cam::ResponseDataRaw::from(data),
                )),
            },
            ResponseData::GetPdos(data) => super::ResponseDataRaw {
                payload: resize(bytemuck::must_cast::<_, [u8; get_pdos::ResponseDataRaw::LEN]>(
                    get_pdos::ResponseDataRaw::from(data),
                )),
            },
            ResponseData::GetCableProperty(data) => super::ResponseDataRaw {
                payload: resize(
                    bytemuck::must_cast::<_, [u8; get_cable_property::ResponseDataRaw::LEN]>(
                        get_cable_property::ResponseDataRaw::from(data),
                    ),
                ),
            },
            ResponseData::GetPdMessage(data) => super::ResponseDataRaw {
                payload: resize(bytemuck::must_cast::<_, [u8; get_pd_message::ResponseDataRaw::LEN]>(
                    get_pd_message::ResponseDataRaw::from(data),
                )),
            },
        }
    }
}

impl TryFrom<(CommandType, super::ResponseDataRaw)> for ResponseData {
    type Error = InvalidResponseData;

    /// Reconstructs response data from its command type and raw bytes
    ///
    /// Returns an error if `command_type` is not an LPM command or if the bytes
    /// do not contain valid response data.
    fn try_from((command_type, raw): (CommandType, super::ResponseDataRaw)) -> Result<Self, Self::Error> {
        let bytes = raw.payload;
        match command_type {
            CommandType::ConnectorReset => Ok(ResponseData::ConnectorReset),
            CommandType::GetConnectorStatus => {
                let raw = bytemuck::must_cast::<_, get_connector_status::ResponseDataRaw>(resize::<
                    { RESPONSE_DATA_MAX_LEN },
                    { get_connector_status::ResponseDataRaw::LEN },
                >(bytes));
                Ok(ResponseData::GetConnectorStatus(
                    get_connector_status::ResponseData::try_from(raw)?,
                ))
            }
            CommandType::GetConnectorCapability => Ok(ResponseData::GetConnectorCapability(
                bytemuck::must_cast::<_, get_connector_capability::ResponseDataRaw>(resize::<
                    { RESPONSE_DATA_MAX_LEN },
                    { get_connector_capability::ResponseDataRaw::LEN },
                >(bytes))
                .into(),
            )),
            CommandType::GetErrorStatus => Ok(ResponseData::GetErrorStatus(
                bytemuck::must_cast::<_, get_error_status::ResponseDataRaw>(resize::<
                    { RESPONSE_DATA_MAX_LEN },
                    { get_error_status::ResponseDataRaw::LEN },
                >(bytes))
                .into(),
            )),
            CommandType::GetAlternateModes => Ok(ResponseData::GetAlternateModes(
                bytemuck::must_cast::<_, get_alternate_modes::ResponseDataRaw>(resize::<
                    { RESPONSE_DATA_MAX_LEN },
                    { get_alternate_modes::ResponseDataRaw::LEN },
                >(bytes))
                .into(),
            )),
            CommandType::GetCamSupported => Ok(ResponseData::GetCamSupported(
                bytemuck::must_cast::<_, get_cam_supported::ResponseDataRaw>(resize::<
                    { RESPONSE_DATA_MAX_LEN },
                    { get_cam_supported::ResponseDataRaw::LEN },
                >(bytes))
                .into(),
            )),
            CommandType::GetCurrentCam => Ok(ResponseData::GetCurrentCam(
                bytemuck::must_cast::<_, get_current_cam::ResponseDataRaw>(resize::<
                    { RESPONSE_DATA_MAX_LEN },
                    { get_current_cam::ResponseDataRaw::LEN },
                >(bytes))
                .into(),
            )),
            CommandType::GetPdos => Ok(ResponseData::GetPdos(
                bytemuck::must_cast::<_, get_pdos::ResponseDataRaw>(resize::<
                    { RESPONSE_DATA_MAX_LEN },
                    { get_pdos::ResponseDataRaw::LEN },
                >(bytes))
                .into(),
            )),
            CommandType::GetCableProperty => Ok(ResponseData::GetCableProperty(
                bytemuck::must_cast::<_, get_cable_property::ResponseDataRaw>(resize::<
                    { RESPONSE_DATA_MAX_LEN },
                    { get_cable_property::ResponseDataRaw::LEN },
                >(bytes))
                .into(),
            )),
            CommandType::GetPdMessage => Ok(ResponseData::GetPdMessage(
                bytemuck::must_cast::<_, get_pd_message::ResponseDataRaw>(resize::<
                    { RESPONSE_DATA_MAX_LEN },
                    { get_pd_message::ResponseDataRaw::LEN },
                >(bytes))
                .into(),
            )),
            _ => Err(InvalidResponseData::InvalidCommandType(InvalidCommandType(
                command_type as u8,
            ))),
        }
    }
}

/// LPM command response
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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
    #[derive(Copy, Clone, Default, PartialEq, Eq)]
    pub(self) struct ConnectorNumberRaw(u8);
    impl Debug;

    // Connector number
    pub u8, connector_number, set_connector_number: 6, 0;
    // Only 7-bits used for the connector number, some commands use this bit as part of their arguments
    pub bool, high_bit, set_high_bit: 7;
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
    use super::*;
    use crate::PowerRole;

    /// Decodes a command from a full command buffer
    fn decode(bytes: [u8; COMMAND_LEN]) -> Result<GlobalCommand, InvalidCommand> {
        GlobalCommand::try_from(bytemuck::must_cast::<_, CommandRaw>(bytes))
    }

    #[test]
    fn test_decode_connector_reset() {
        let mut bytes = [0u8; COMMAND_LEN];
        bytes[0] = CommandType::ConnectorReset as u8;
        bytes[2] = 0x81;

        assert_eq!(
            decode(bytes).unwrap(),
            GlobalCommand::new(CommandData::ConnectorReset(connector_reset::Args {
                connector_number: GlobalPortId(1),
                hard_reset: true,
            }))
        );
    }

    #[test]
    fn test_decode_get_connector_status() {
        let mut bytes = [0u8; COMMAND_LEN];
        bytes[0] = CommandType::GetConnectorStatus as u8;
        bytes[2] = 0x1;

        assert_eq!(
            decode(bytes).unwrap(),
            GlobalCommand::new(CommandData::GetConnectorStatus(get_connector_status::Args {
                connector_number: GlobalPortId(1)
            }))
        );
    }

    #[test]
    fn test_decode_get_connector_capability() {
        let mut bytes = [0u8; COMMAND_LEN];
        bytes[0] = CommandType::GetConnectorCapability as u8;
        bytes[2] = 0x1;

        assert_eq!(
            decode(bytes).unwrap(),
            GlobalCommand::new(CommandData::GetConnectorCapability(get_connector_capability::Args {
                connector_number: GlobalPortId(1)
            }))
        );
    }

    #[test]
    fn test_decode_set_power_level() {
        let mut bytes = [0u8; COMMAND_LEN];
        bytes[0] = CommandType::SetPowerLevel as u8;
        bytes[2] = 0x81;

        assert_eq!(
            decode(bytes).unwrap(),
            GlobalCommand::new(CommandData::SetPowerLevel(set_power_level::Args {
                connector_number: GlobalPortId(1),
                power_role: PowerRole::Source,
                ..Default::default()
            }))
        )
    }

    #[test]
    fn test_decode_set_power_level_invalid_current() {
        let mut bytes = [0u8; COMMAND_LEN];
        bytes[0] = CommandType::SetPowerLevel as u8;
        bytes[2] = 0x01;
        // Invalid type_c_current value (0x4) at bits 18:16
        bytes[4] = 0x04;

        assert_eq!(
            decode(bytes),
            Err(InvalidCommand::InvalidCurrent(set_power_level::InvalidCurrent(0x04)))
        );
    }

    #[test]
    fn test_decode_get_alternate_modes() {
        let mut bytes = [0u8; COMMAND_LEN];
        bytes[0] = CommandType::GetAlternateModes as u8;
        bytes[2] = 0x1; // SOP recipient

        assert_eq!(
            decode(bytes).unwrap(),
            GlobalCommand::new(CommandData::GetAlternateModes(get_alternate_modes::Args {
                recipient: Recipient::Sop,
                ..Default::default()
            }))
        );
    }

    #[test]
    fn test_decode_get_alternate_modes_invalid_recipient() {
        let mut bytes = [0u8; COMMAND_LEN];
        bytes[0] = CommandType::GetAlternateModes as u8;
        bytes[2] = 0x7; // Invalid recipient

        assert_eq!(
            decode(bytes),
            Err(InvalidCommand::InvalidRecipient(InvalidRecipient(0x7)))
        );
    }

    #[test]
    fn test_decode_set_ccom() {
        let mut bytes = [0u8; COMMAND_LEN];
        bytes[0] = CommandType::SetCcom as u8;
        bytes[2] = 0x81;

        assert_eq!(
            decode(bytes).unwrap(),
            GlobalCommand::new(CommandData::SetCcom(set_ccom::Args {
                connector_number: GlobalPortId(1),
                rp: true,
                ..Default::default()
            }))
        );
    }

    #[test]
    fn test_decode_set_new_cam() {
        let mut bytes = [0u8; COMMAND_LEN];
        bytes[0] = CommandType::SetNewCam as u8;
        bytes[2] = 0x1;

        assert_eq!(
            decode(bytes).unwrap(),
            GlobalCommand::new(CommandData::SetNewCam(set_new_cam::Args {
                connector_number: GlobalPortId(1),
                enter: false,
                am_offset: 0,
                am_specific: 0,
            }))
        );
    }

    #[test]
    fn test_decode_set_uor() {
        let mut bytes = [0u8; COMMAND_LEN];
        bytes[0] = CommandType::SetUor as u8;
        bytes[2] = 0x81;

        assert_eq!(
            decode(bytes).unwrap(),
            GlobalCommand::new(CommandData::SetUor(set_uor::Args {
                connector_number: GlobalPortId(1),
                dfp: true,
                ..Default::default()
            }))
        );
    }

    #[test]
    fn test_decode_get_error_status() {
        let mut bytes = [0u8; COMMAND_LEN];
        bytes[0] = CommandType::GetErrorStatus as u8;
        bytes[2] = 0x1;

        assert_eq!(
            decode(bytes).unwrap(),
            GlobalCommand::new(CommandData::GetErrorStatus(get_error_status::Args {
                connector_number: GlobalPortId(1)
            }))
        );
    }

    #[test]
    fn test_decode_set_pdr() {
        let mut bytes = [0u8; COMMAND_LEN];
        bytes[0] = CommandType::SetPdr as u8;
        bytes[2] = 0x81;

        assert_eq!(
            decode(bytes).unwrap(),
            GlobalCommand::new(CommandData::SetPdr(set_pdr::Args {
                connector_number: GlobalPortId(1),
                swap_source: true,
                ..Default::default()
            }))
        );
    }

    #[test]
    fn test_get_cam_supported() {
        let mut bytes = [0u8; COMMAND_LEN];
        bytes[0] = CommandType::GetCamSupported as u8;
        bytes[2] = 0x1;

        assert_eq!(
            decode(bytes).unwrap(),
            GlobalCommand::new(CommandData::GetCamSupported(get_cam_supported::Args {
                connector_number: GlobalPortId(1)
            }))
        );
    }

    #[test]
    fn test_get_current_cam() {
        let mut bytes = [0u8; COMMAND_LEN];
        bytes[0] = CommandType::GetCurrentCam as u8;
        bytes[2] = 0x1;

        assert_eq!(
            decode(bytes).unwrap(),
            GlobalCommand::new(CommandData::GetCurrentCam(get_current_cam::Args {
                connector_number: GlobalPortId(1)
            }))
        );
    }

    #[test]
    fn test_get_pdos() {
        let mut bytes = [0u8; COMMAND_LEN];
        bytes[0] = CommandType::GetPdos as u8;
        bytes[2] = 0x81;

        assert_eq!(
            decode(bytes).unwrap(),
            GlobalCommand::new(CommandData::GetPdos(get_pdos::Args {
                connector_number: GlobalPortId(1),
                partner: true,
                ..Default::default()
            }))
        );
    }

    #[test]
    fn test_decode_get_pdos_invalid_source_capability_type() {
        let mut bytes = [0u8; COMMAND_LEN];
        bytes[0] = CommandType::GetPdos as u8;
        bytes[2] = 0x1;
        bytes[4] = 0x18; // Source capability type 0x3, bits 20:19

        assert_eq!(
            decode(bytes),
            Err(InvalidCommand::InvalidSourceCapabilityType(
                get_pdos::InvalidSourceCapabilityType(0x3)
            ))
        );
    }

    #[test]
    fn test_encode_get_pdos_invalid_num_pdos() {
        let command = GlobalCommand::new(CommandData::GetPdos(get_pdos::Args {
            num_pdos: 0,
            ..Default::default()
        }));

        assert_eq!(
            CommandRaw::try_from(command),
            Err(InvalidCommand::InvalidNumPdos(get_pdos::InvalidNumPdos(0)))
        );
    }

    #[test]
    fn test_encode_get_pdos_response_variable_length() {
        let response = ResponseData::GetPdos(get_pdos::ResponseData {
            pdos: [0x11223344, 0x55667788, 0, 0],
        });

        let len = response.data_len();
        let raw: crate::ucsi::v1_2::ResponseDataRaw = response.into();

        assert_eq!(len, 2 * size_of::<u32>());
        assert_eq!(raw.payload[..len], [0x44, 0x33, 0x22, 0x11, 0x88, 0x77, 0x66, 0x55]);
    }

    #[test]
    fn test_get_cable_property() {
        let mut bytes = [0u8; COMMAND_LEN];
        bytes[0] = CommandType::GetCableProperty as u8;
        bytes[2] = 0x1;

        assert_eq!(
            decode(bytes).unwrap(),
            GlobalCommand::new(CommandData::GetCableProperty(get_cable_property::Args {
                connector_number: GlobalPortId(1)
            }))
        );
    }

    #[test]
    fn test_get_pd_message() {
        let mut bytes = [0u8; COMMAND_LEN];
        bytes[0] = CommandType::GetPdMessage as u8;
        bytes[2] = 0x83;
        bytes[3] = 0x08;
        bytes[4] = 0x01;
        bytes[5] = 0x02;

        assert_eq!(
            decode(bytes).unwrap(),
            GlobalCommand::new(CommandData::GetPdMessage(get_pd_message::Args {
                connector_number: GlobalPortId(3),
                recipient: Recipient::Sop,
                message_offset: 2,
                num_bytes: 1,
                message_type: get_pd_message::MessageType::BatteryCap,
            }))
        );
    }

    #[test]
    fn test_decode_get_pd_message_invalid_recipient() {
        let mut bytes = [0u8; COMMAND_LEN];
        bytes[0] = CommandType::GetPdMessage as u8;
        bytes[2] = 0x83;
        bytes[3] = 0x0B; // Recipient 0x7, bits 9:7 cross the byte boundary
        bytes[4] = 0x01;
        bytes[5] = 0x02;

        assert_eq!(
            decode(bytes),
            Err(InvalidCommand::InvalidPdMessageArgs(
                get_pd_message::InvalidArgs::InvalidRecipient(InvalidRecipient(0x7))
            ))
        );
    }

    #[test]
    fn test_decode_get_pd_message_invalid_message_type() {
        let mut bytes = [0u8; COMMAND_LEN];
        bytes[0] = CommandType::GetPdMessage as u8;
        bytes[2] = 0x83;
        bytes[3] = 0x38;
        bytes[4] = 0x01;
        bytes[5] = 0x0f; // Invalid message type

        assert_eq!(
            decode(bytes),
            Err(InvalidCommand::InvalidPdMessageArgs(
                get_pd_message::InvalidArgs::InvalidMessageType(get_pd_message::InvalidMessageType(0x0f))
            ))
        );
    }
    /// Every LPM command carries a connector, so it must survive a set/get round trip
    #[test]
    fn test_set_connector_all_commands() {
        let commands = [
            CommandData::ConnectorReset(Default::default()),
            CommandData::GetConnectorStatus(Default::default()),
            CommandData::GetConnectorCapability(Default::default()),
            CommandData::SetPowerLevel(Default::default()),
            CommandData::SetNewCam(Default::default()),
            CommandData::GetErrorStatus(Default::default()),
            CommandData::SetCcom(Default::default()),
            CommandData::SetUor(Default::default()),
            CommandData::SetPdr(Default::default()),
            CommandData::GetAlternateModes(Default::default()),
            CommandData::GetCamSupported(Default::default()),
            CommandData::GetCurrentCam(Default::default()),
            CommandData::GetPdos(Default::default()),
            CommandData::GetCableProperty(Default::default()),
            CommandData::GetPdMessage(Default::default()),
        ];

        for operation in commands {
            let mut command = GlobalCommand::new(operation);
            assert_eq!(command.connector_number(), GlobalPortId(0));

            command.set_connector_number(GlobalPortId(3));
            assert_eq!(command.connector_number(), GlobalPortId(3));
            // The connector must also reach the wire format
            assert_eq!(
                GlobalCommand::try_from(CommandRaw::try_from(command).unwrap())
                    .unwrap()
                    .connector_number(),
                GlobalPortId(3)
            );
        }
    }
}

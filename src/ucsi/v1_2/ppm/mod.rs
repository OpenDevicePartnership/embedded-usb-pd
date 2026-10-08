use crate::ucsi::v1_2::{cci, CommandHeaderRaw, CommandRaw, CommandType, InvalidCommandType, COMMAND_LEN};
use crate::{GlobalPortId, LocalPortId, PortId};

pub mod ack_cc_ci;
pub mod cancel;
pub mod get_capability;
pub mod ppm_reset;
pub mod set_notification_enable;
pub mod state_machine;

pub const RESPONSE_DATA_MAX_LEN: usize = get_capability::ResponseDataRaw::LEN;

/// Commands that only affect the PPM level and don't need to be sent to an LPM
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Command {
    PpmReset,
    Cancel,
    AckCcCi(ack_cc_ci::Args),
    SetNotificationEnable(set_notification_enable::Args),
    GetCapability,
}

impl Command {
    /// Length of a PPM command payload, the command minus its header
    pub const PAYLOAD_LEN: usize = COMMAND_LEN - size_of::<CommandHeaderRaw>();

    /// Returns the command type for this command
    pub const fn command_type(&self) -> CommandType {
        match self {
            Command::PpmReset => CommandType::PpmReset,
            Command::Cancel => CommandType::Cancel,
            Command::AckCcCi(_) => CommandType::AckCcCi,
            Command::SetNotificationEnable(_) => CommandType::SetNotificationEnable,
            Command::GetCapability => CommandType::GetCapability,
        }
    }
}

impl From<Command> for CommandRaw {
    fn from(command: Command) -> Self {
        let payload = match command {
            Command::PpmReset => bytemuck::must_cast(ppm_reset::ArgsRaw::from(ppm_reset::Args)),
            Command::Cancel => bytemuck::must_cast(cancel::ArgsRaw::from(cancel::Args)),
            Command::AckCcCi(args) => bytemuck::must_cast(ack_cc_ci::ArgsRaw::from(args)),
            Command::SetNotificationEnable(args) => bytemuck::must_cast(set_notification_enable::ArgsRaw::from(args)),
            Command::GetCapability => bytemuck::must_cast(get_capability::ArgsRaw::from(get_capability::Args)),
        };

        CommandRaw {
            command: command.command_type().into(),
            // Data length is only non-zero for vendor-defined commands, none of which are modelled here
            data_len: 0,
            payload,
        }
    }
}

impl TryFrom<CommandRaw> for Command {
    type Error = InvalidCommandType;

    /// Reconstructs a command from its raw wire format
    ///
    /// Returns [`InvalidCommandType`] if the command type is not a PPM command.
    fn try_from(raw: CommandRaw) -> Result<Self, Self::Error> {
        match CommandType::try_from(raw.command)? {
            CommandType::PpmReset => Ok(Command::PpmReset),
            CommandType::Cancel => Ok(Command::Cancel),
            CommandType::AckCcCi => Ok(Command::AckCcCi(
                bytemuck::must_cast::<_, ack_cc_ci::ArgsRaw>(raw.payload).into(),
            )),
            CommandType::SetNotificationEnable => Ok(Command::SetNotificationEnable(
                bytemuck::must_cast::<_, set_notification_enable::ArgsRaw>(raw.payload).into(),
            )),
            CommandType::GetCapability => Ok(Command::GetCapability),
            command_type => Err(InvalidCommandType(command_type as u8)),
        }
    }
}

/// PPM command response data
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ResponseData {
    GetCapability(get_capability::ResponseData),
}

impl ResponseData {
    /// Returns the command type that produces this response data
    pub const fn command_type(&self) -> CommandType {
        match self {
            ResponseData::GetCapability(_) => CommandType::GetCapability,
        }
    }

    /// Number of valid bytes this response data occupies on the wire
    pub const fn data_len(&self) -> usize {
        match self {
            ResponseData::GetCapability(_) => get_capability::ResponseDataRaw::LEN,
        }
    }
}

impl From<ResponseData> for super::ResponseDataRaw {
    /// Converts response data into a [`ResponseDataRaw::MAX_LEN`] sized buffer
    ///
    /// Only the first [`ResponseData::data_len`] bytes are valid.
    fn from(data: ResponseData) -> Self {
        match data {
            ResponseData::GetCapability(data) => {
                bytemuck::must_cast::<_, super::ResponseDataRaw>(get_capability::ResponseDataRaw::from(data))
            }
        }
    }
}

impl TryFrom<(CommandType, super::ResponseDataRaw)> for ResponseData {
    type Error = InvalidCommandType;

    /// Reconstructs response data from its command type and raw bytes
    ///
    /// Returns [`InvalidCommandType`] if `command_type` is not a PPM command
    /// that produces response data.
    fn try_from((command_type, raw): (CommandType, super::ResponseDataRaw)) -> Result<Self, Self::Error> {
        match command_type {
            CommandType::GetCapability => Ok(ResponseData::GetCapability(
                bytemuck::must_cast::<_, get_capability::ResponseDataRaw>(super::ResponseDataRaw {
                    payload: raw.payload,
                })
                .into(),
            )),
            _ => Err(InvalidCommandType(command_type as u8)),
        }
    }
}

/// PPM command response
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ucsi::v1_2::ResponseDataRaw;

    /// Builds the raw wire format of a PPM command from its type and payload
    fn raw(command_type: CommandType, payload: [u8; Command::PAYLOAD_LEN]) -> CommandRaw {
        CommandRaw {
            command: command_type.into(),
            data_len: 0,
            payload,
        }
    }

    /// Asserts that `command` round-trips through [`CommandRaw`] as `payload`
    fn assert_payload_roundtrip(command: Command, payload: [u8; Command::PAYLOAD_LEN]) {
        let expected = raw(command.command_type(), payload);

        assert_eq!(CommandRaw::from(command), expected);
        assert_eq!(Command::try_from(expected), Ok(command));
    }

    #[test]
    fn test_ppm_reset_payload() {
        assert_payload_roundtrip(Command::PpmReset, [0u8; Command::PAYLOAD_LEN]);
    }

    #[test]
    fn test_cancel_payload() {
        assert_payload_roundtrip(Command::Cancel, [0u8; Command::PAYLOAD_LEN]);
    }

    #[test]
    fn test_ack_cc_ci_payload() {
        let mut payload = [0u8; Command::PAYLOAD_LEN];
        payload[0] = 0x2; // Ack command complete

        assert_payload_roundtrip(
            Command::AckCcCi(ack_cc_ci::Args {
                ack: ack_cc_ci::Ack::from(0x2),
            }),
            payload,
        );
    }

    #[test]
    fn test_set_notification_enable_payload() {
        let mut payload = [0u8; Command::PAYLOAD_LEN];
        payload[0] = 0x1; // Enable command complete notification

        assert_payload_roundtrip(
            Command::SetNotificationEnable(set_notification_enable::Args {
                notification_enable: set_notification_enable::NotificationEnable::from(0x1),
            }),
            payload,
        );
    }

    #[test]
    fn test_get_capability_payload() {
        assert_payload_roundtrip(Command::GetCapability, [0u8; Command::PAYLOAD_LEN]);
    }

    #[test]
    fn test_try_from_raw_non_ppm_command() {
        assert_eq!(
            Command::try_from(raw(CommandType::GetConnectorStatus, [0u8; Command::PAYLOAD_LEN])),
            Err(InvalidCommandType(CommandType::GetConnectorStatus as u8))
        );
    }

    #[test]
    fn test_try_from_raw_invalid_command_type() {
        assert_eq!(Command::try_from(CommandRaw::default()), Err(InvalidCommandType(0)));
    }

    #[test]
    fn test_response_data_bytes() {
        let data = get_capability::ResponseData {
            num_connectors: 1,
            ..Default::default()
        };
        let expected = ResponseData::GetCapability(data);

        let bytes: ResponseDataRaw = expected.into();

        assert_eq!(expected.data_len(), get_capability::ResponseDataRaw::LEN);
        assert_eq!(
            ResponseData::try_from((CommandType::GetCapability, bytes)),
            Ok(expected)
        );
    }

    #[test]
    fn test_response_data_from_bytes_invalid_command() {
        let bytes = ResponseDataRaw {
            payload: [0u8; ResponseDataRaw::MAX_LEN],
        };

        assert_eq!(
            ResponseData::try_from((CommandType::PpmReset, bytes)),
            Err(InvalidCommandType(CommandType::PpmReset as u8))
        );
    }
}

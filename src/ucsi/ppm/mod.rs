use crate::ucsi::{cci, CommandHeader, CommandType};
use crate::{GlobalPortId, LocalPortId, PortId};

pub mod ack_cc_ci;
pub mod cancel;
pub mod get_capability;
pub mod ppm_reset;
pub mod set_notification_enable;
pub mod state_machine;

use bincode::de::{Decode, Decoder};
use bincode::enc::{Encode, Encoder};
use bincode::error::{AllowedEnumVariants, DecodeError, EncodeError};

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

impl Encode for Command {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        match self {
            Command::PpmReset => ppm_reset::Args.encode(encoder),
            Command::Cancel => cancel::Args.encode(encoder),
            Command::AckCcCi(args) => args.encode(encoder),
            Command::SetNotificationEnable(args) => args.encode(encoder),
            Command::GetCapability => get_capability::Args.encode(encoder),
        }
    }
}

impl Decode<CommandHeader> for Command {
    fn decode<D: Decoder<Context = CommandHeader>>(decoder: &mut D) -> Result<Self, DecodeError> {
        match decoder.context().command() {
            CommandType::PpmReset => {
                // Don't actually have args, but we need to consume the bytes
                let _args = ppm_reset::Args::decode(decoder)?;
                Ok(Command::PpmReset)
            }
            CommandType::Cancel => {
                // Don't actually have args, but we need to consume the bytes
                let _args = cancel::Args::decode(decoder)?;
                Ok(Command::Cancel)
            }
            CommandType::AckCcCi => Ok(Command::AckCcCi(ack_cc_ci::Args::decode(decoder)?)),
            CommandType::SetNotificationEnable => Ok(Command::SetNotificationEnable(
                set_notification_enable::Args::decode(decoder)?,
            )),
            CommandType::GetCapability => {
                // Don't actually have args, but we need to consume the bytes
                let _args = get_capability::Args::decode(decoder)?;
                Ok(Command::GetCapability)
            }
            command_type => Err(DecodeError::UnexpectedVariant {
                type_name: "CommandType",
                allowed: &AllowedEnumVariants::Allowed(&[
                    CommandType::PpmReset as u32,
                    CommandType::Cancel as u32,
                    CommandType::AckCcCi as u32,
                    CommandType::SetNotificationEnable as u32,
                    CommandType::GetCapability as u32,
                ]),
                found: command_type as u32,
            }),
        }
    }
}

impl Decode<()> for Command {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, DecodeError> {
        let header = CommandHeader::decode(decoder)?;
        Command::decode(&mut decoder.with_context(header))
    }
}

/// PPM command response data
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ResponseData {
    GetCapability(get_capability::ResponseData),
}

impl Encode for ResponseData {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        match self {
            ResponseData::GetCapability(data) => data.encode(encoder),
        }
    }
}

impl Decode<CommandType> for ResponseData {
    fn decode<D: Decoder<Context = CommandType>>(decoder: &mut D) -> Result<Self, DecodeError> {
        match decoder.context() {
            CommandType::GetCapability => Ok(ResponseData::GetCapability(get_capability::ResponseData::decode(
                decoder,
            )?)),
            _ => Err(DecodeError::UnexpectedVariant {
                type_name: "CommandType",
                allowed: &AllowedEnumVariants::Allowed(&[CommandType::GetCapability as u32]),
                found: *decoder.context() as u32,
            }),
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
    use bincode::config::standard;
    use bincode::decode_from_slice;

    use super::*;
    use crate::ucsi::COMMAND_LEN;

    #[test]
    fn test_decode_ppm_reset() {
        let mut bytes = [0u8; COMMAND_LEN];
        bytes[0] = CommandType::PpmReset as u8;

        let (ppm_reset, consumed): (Command, usize) =
            decode_from_slice(&bytes, standard().with_fixed_int_encoding()).unwrap();
        assert_eq!(consumed, bytes.len());
        assert_eq!(ppm_reset, Command::PpmReset);
    }

    #[test]
    fn test_decode_cancel() {
        let mut bytes = [0u8; COMMAND_LEN];
        bytes[0] = CommandType::Cancel as u8;

        let (cancel, consumed): (Command, usize) =
            decode_from_slice(&bytes, standard().with_fixed_int_encoding()).unwrap();
        assert_eq!(consumed, bytes.len());
        assert_eq!(cancel, Command::Cancel);
    }

    #[test]
    fn test_decode_ack_cc_ci() {
        let mut bytes = [0u8; COMMAND_LEN];
        bytes[0] = CommandType::AckCcCi as u8;
        bytes[2] = 0x2; // Set connector change ack

        let (ack_cc_ci, consumed): (Command, usize) =
            decode_from_slice(&bytes, standard().with_fixed_int_encoding()).unwrap();
        assert_eq!(consumed, bytes.len());
        assert_eq!(
            ack_cc_ci,
            Command::AckCcCi(ack_cc_ci::Args {
                ack: ack_cc_ci::Ack::from(0x2)
            })
        );
    }

    #[test]
    fn test_decode_set_notification_enable() {
        let mut bytes = [0u8; COMMAND_LEN];
        bytes[0] = CommandType::SetNotificationEnable as u8;
        bytes[2] = 0x1; // Enable command complete notification

        let (set_notification_enable, consumed): (Command, usize) =
            decode_from_slice(&bytes, standard().with_fixed_int_encoding()).unwrap();
        assert_eq!(consumed, bytes.len());
        assert_eq!(
            set_notification_enable,
            Command::SetNotificationEnable(set_notification_enable::Args {
                notification_enable: set_notification_enable::NotificationEnable::from(0x1)
            })
        );
    }

    #[test]
    fn test_decode_get_capability() {
        let mut bytes = [0u8; COMMAND_LEN];
        bytes[0] = CommandType::GetCapability as u8;

        let (get_capability, consumed): (Command, usize) =
            decode_from_slice(&bytes, standard().with_fixed_int_encoding()).unwrap();
        assert_eq!(consumed, bytes.len());
        assert_eq!(get_capability, Command::GetCapability);
    }
}

//! [`Header`] defines the VDM Header for a Structured VDM Message.

use crate::vdm::structured::Svid;

/// The VDM Header for a Structured VDM Message.
///
/// See PD spec 6.4.4.2 Structured VDM, table 6.29 Structured VDM Header.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Header {
    pub command: Command,
    pub command_type: CommandType,
    pub object_position: ObjectPosition,
    pub structured_vdm_version: StructuredVdmVersion,
    pub svid: Svid,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Command {
    DiscoverIdentity,
    DiscoverSvids,
    DiscoverModes,
    EnterMode,
    ExitMode,
    Attention,

    /// SVID-specific Commands as defined by the vendor in [`Header::svid`].
    SvidSpecific(u8),
}

impl TryFrom<u8> for Command {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::DiscoverIdentity),
            2 => Ok(Self::DiscoverSvids),
            3 => Ok(Self::DiscoverModes),
            4 => Ok(Self::EnterMode),
            5 => Ok(Self::ExitMode),
            6 => Ok(Self::Attention),
            cmd if cmd >= 16 => Ok(Self::SvidSpecific(cmd)),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CommandType {
    Request = 0b00,
    Ack = 0b01,
    Nak = 0b10,
    Busy = 0b11,
}

impl From<u8> for CommandType {
    fn from(value: u8) -> Self {
        match value & 0b11 {
            0b00 => Self::Request,
            0b01 => Self::Ack,
            0b10 => Self::Nak,
            // technically >0b11 is unreachable since we masked with 0b11, but we'll
            // treat any cosmic rays as a Busy response to make this From, not TryFrom
            // 0b11 | _ bothers clippy::wildcard_in_or_patterns, so we'll just use a wildcard arm
            _ => Self::Busy,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ObjectPosition(pub u8);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
/// Version number of the Structured VDM (not the specification).
pub struct StructuredVdmVersion(pub u8);

bitfield::bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub struct Raw(u32);
    impl Debug;

    pub u8, command, set_command: 4, 0;
    pub u8, command_type, set_command_type: 7, 6;
    pub u8, object_position, set_object_position: 10, 8;
    pub u8, structured_vdm_version, set_structured_vdm_version: 14, 11;
    pub u16, svid, set_svid: 31, 16;
}

/// Errors that can occur when parsing a [`Header`].
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ParseError {
    InvalidCommand,
}

impl TryFrom<Raw> for Header {
    type Error = ParseError;
    fn try_from(raw: Raw) -> Result<Self, Self::Error> {
        Ok(Self {
            command: raw.command().try_into().map_err(|()| ParseError::InvalidCommand)?,
            command_type: raw.command_type().into(),
            object_position: ObjectPosition(raw.object_position()),
            structured_vdm_version: StructuredVdmVersion(raw.structured_vdm_version()),
            svid: Svid(raw.svid()),
        })
    }
}

impl TryFrom<u32> for Header {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Raw(value).try_into()
    }
}

impl TryFrom<[u8; 4]> for Header {
    type Error = ParseError;
    fn try_from(bytes: [u8; 4]) -> Result<Self, Self::Error> {
        u32::from_le_bytes(bytes).try_into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod command {
        use super::*;

        #[test]
        fn all_valid_variants() {
            let cases: [(u8, Command); 7] = [
                (1, Command::DiscoverIdentity),
                (2, Command::DiscoverSvids),
                (3, Command::DiscoverModes),
                (4, Command::EnterMode),
                (5, Command::ExitMode),
                (6, Command::Attention),
                (16, Command::SvidSpecific(16)),
            ];
            for (raw, expected) in cases {
                assert_eq!(Command::try_from(raw), Ok(expected), "raw={raw}");
            }
        }

        #[test]
        fn svid_specific_upper_range() {
            for v in [16u8, 17, 31, 127, 255] {
                assert_eq!(Command::try_from(v), Ok(Command::SvidSpecific(v)));
            }
        }

        #[test]
        fn invalid_values() {
            // 0 and 7..=15 are invalid
            assert!(Command::try_from(0u8).is_err());
            for v in 7..=15u8 {
                assert!(Command::try_from(v).is_err(), "raw={v} should be invalid");
            }
        }
    }

    mod command_type {
        use super::*;

        #[test]
        fn all_valid_variants() {
            let cases: [(u8, CommandType); 4] = [
                (0, CommandType::Request),
                (1, CommandType::Ack),
                (2, CommandType::Nak),
                (3, CommandType::Busy),
            ];
            for (raw, expected) in cases {
                assert_eq!(CommandType::from(raw), expected, "raw={raw}");
            }
        }

        #[test]
        fn invalid_values() {
            // since the value is masked with 0b11, all values >0b11 are technically invalid but actually unreachable
            // assert that we loop in order rather than panic
            let expected = [
                CommandType::Request,
                CommandType::Ack,
                CommandType::Nak,
                CommandType::Busy,
            ];

            for (raw, expected) in (4..=255u8).zip(expected.iter().cycle().copied()) {
                assert_eq!(
                    CommandType::from(raw),
                    expected,
                    "raw={raw} should parse as {expected:?}"
                );
            }
        }
    }
}

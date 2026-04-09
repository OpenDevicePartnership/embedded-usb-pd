//! [`Header]` defines the VDM Header for a Structured VDM Message.

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
    Request,
    Ack,
    Nak,
    Busy,
}

impl TryFrom<u8> for CommandType {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Request),
            1 => Ok(Self::Ack),
            2 => Ok(Self::Nak),
            3 => Ok(Self::Busy),
            _ => Err(()),
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
    pub u16, svid, set_svid: 30, 16;
}

pub enum ParseError {
    InvalidCommand,
    InvalidCommandType,
}

impl TryFrom<Raw> for Header {
    type Error = ParseError;
    fn try_from(raw: Raw) -> Result<Self, Self::Error> {
        Ok(Self {
            command: raw.command().try_into().map_err(|()| ParseError::InvalidCommand)?,
            command_type: raw
                .command_type()
                .try_into()
                .map_err(|()| ParseError::InvalidCommandType)?,
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

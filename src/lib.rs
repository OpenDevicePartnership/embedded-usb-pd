#![no_std]

pub mod ado;
pub mod constants;
pub mod pdinfo;
pub mod pdo;
pub mod type_c;
pub mod ucsi;
pub mod vdm;

use core::hash::Hash;

/// Common port trait
///
/// Ports are identified by a numeric ID. However, a port may have a different numeric ID in different contexts.
/// Newtypes can provide type-checking of this at compile time. This trait provides a way to work with port IDs in
/// a generic manner.
#[cfg(not(feature = "defmt"))]
pub trait PortId: Into<u8> + From<u8> + Copy + Clone + core::fmt::Debug + PartialEq + Eq + Hash {}

/// Common port trait
///
/// Ports are identified by a numeric ID. However, a port may have a different numeric ID in different contexts.
/// Newtypes can provide type-checking of this at compile time. This trait provides a way to work with port IDs in
/// a generic manner.
#[cfg(feature = "defmt")]
pub trait PortId:
    Into<u8> + From<u8> + Copy + Clone + core::fmt::Debug + PartialEq + Eq + Hash + defmt::Format
{
}

/// Port ID new type.
///
/// This differs from [`GlobalPortId`] in that it refers to a port on a specific controller. If
/// there are multiple controllers, the same port ID may be used on different controllers.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(transparent)]
pub struct LocalPortId(pub u8);

impl From<u8> for LocalPortId {
    fn from(port: u8) -> Self {
        LocalPortId(port)
    }
}

impl From<LocalPortId> for u8 {
    fn from(port: LocalPortId) -> Self {
        port.0
    }
}

impl PortId for LocalPortId {}

/// Global port ID, used to unique identify a port
///
/// This differs from [`LocalPortId`] in that it is not limited to the number of ports on a single
/// controller. If there are multiple controllers, each port should have a unique global port ID.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(transparent)]
pub struct GlobalPortId(pub u8);

impl From<u8> for GlobalPortId {
    fn from(port: u8) -> Self {
        GlobalPortId(port)
    }
}

impl From<GlobalPortId> for u8 {
    fn from(port: GlobalPortId) -> Self {
        port.0
    }
}

impl PortId for GlobalPortId {}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
/// General PD-related errors
pub enum PdError {
    /// Invalid controller
    InvalidController,
    /// Invalid response
    InvalidResponse,
    /// Unrecognized command
    UnrecognizedCommand,
    /// Invalid port
    InvalidPort,
    /// Invalid parameters
    InvalidParams,
    /// Incompatible partner
    IncompatiblePartner,
    /// CC communication error,
    CcCommunication,
    /// Failed due to dead battery condition
    DeadBattery,
    /// Contract negociation failed
    ContractNegociation,
    /// Overcurrent
    Overcurrent,
    /// Swap rejected by port partner
    SwapRejectedPartner,
    /// Hard reset
    HardReset,
    /// Policy conflict
    PolicyConflict,
    /// Swap rejected
    SwapRejected,
    /// Reverse current protection
    ReverseCurrent,
    /// Set sink path rejected
    SetSinkPath,
    /// The requested action has not yet completed
    Busy,
    /// The requested action timed out
    Timeout,
    /// Generic failure
    Failed,
    /// The device is in the incorrect mode
    InvalidMode,
    /// Data serialization error
    Serialize,
    /// Command not yet completed
    InProgress,
    /// Command was valid, but could not be executed at this time
    Rejected,
}

/// Top-level error type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error<BE> {
    /// Bus error
    Bus(BE),
    /// General PD error
    Pd(PdError),
}

/// Power role
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PowerRole {
    /// Sink role
    #[default]
    Sink,
    /// Source role
    Source,
}

/// Data role
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DataRole {
    /// Upward-facing port
    Ufp,
    /// Downward-facing port
    Dfp,
}

/// Plug orientation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PlugOrientation {
    /// Upside-up orientation
    #[default]
    CC1,
    /// Upside-down orientation
    CC2,
}

impl PlugOrientation {
    /// Returns true if the plug orientation is flipped
    pub fn flipped(&self) -> bool {
        *self == PlugOrientation::CC2
    }
}

impl<T, BE> From<PdError> for Result<T, Error<BE>> {
    fn from(err: PdError) -> Self {
        Err(err.into())
    }
}

impl<BE> From<PdError> for Error<BE> {
    fn from(err: PdError) -> Self {
        Error::Pd(err)
    }
}

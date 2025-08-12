#![no_std]

pub mod ado;
pub mod constants;
pub mod pdinfo;
pub mod pdo;
pub mod type_c;
pub mod ucsi;

/// Port ID new type.
///
/// This differs from [`GlobalPortId`] in that it refers to a port on a specific controller. If
/// there are multiple controllers, the same port ID may be used on different controllers.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(transparent)]
pub struct PortId(pub u8);

impl From<u8> for PortId {
    fn from(port: u8) -> Self {
        PortId(port)
    }
}

/// Global port ID, used to unique identify a port
///
/// This differs from [`PortId`] in that it is not limited to the number of ports on a single
/// controller. If there are multiple controllers, each port should have a unique global port ID.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(transparent)]
pub struct GlobalPortId(pub u8);

impl From<u8> for GlobalPortId {
    fn from(port: u8) -> Self {
        GlobalPortId(port)
    }
}

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

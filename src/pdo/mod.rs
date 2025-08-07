//! Power data object (PDO) definitions
//! This module defines source and sink PDOs. Each PDO type has a corresponding *Raw and *Data struct.
//! The raw struct just provides a structured version of the raw PDO data, while the data struct provides
//! a type-safe version.
use crate::PdError;

mod rdo;
pub mod sink;
pub mod source;

pub use rdo::Rdo;

/// 5 mA unit
pub const MA5_UNIT: u16 = 5;
/// 10 mA unit
pub const MA10_UNIT: u16 = 10;
/// 50 mA unit
pub const MA50_UNIT: u16 = 50;
/// 5 mV unit
pub const MV5_UNIT: u16 = 5;
/// 20 mV unit
pub const MV20_UNIT: u16 = 20;
/// 50 mV unit
pub const MV50_UNIT: u16 = 50;
/// 100 mV unit
pub const MV100_UNIT: u16 = 100;
/// 250 mV unit
pub const MW250_UNIT: u32 = 250;
/// 1000 mW unit
pub const MW1000_UNIT: u32 = 1000;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PdoKind {
    Fixed,
    Battery,
    Variable,
    Augmented,
}

impl From<u32> for PdoKind {
    fn from(pdo: u32) -> Self {
        const PDO_KIND_SHIFT: u8 = 30;
        PdoKind::from((pdo >> PDO_KIND_SHIFT) as u8)
    }
}

impl From<u8> for PdoKind {
    fn from(value: u8) -> Self {
        const PDO_KIND_MASK: u8 = 0x3;
        match value & PDO_KIND_MASK {
            0x0 => PdoKind::Fixed,
            0x1 => PdoKind::Battery,
            0x2 => PdoKind::Variable,
            0x3 => PdoKind::Augmented,
            _ => unreachable!(),
        }
    }
}

impl From<PdoKind> for u8 {
    fn from(value: PdoKind) -> Self {
        match value {
            PdoKind::Fixed => 0x0,
            PdoKind::Battery => 0x1,
            PdoKind::Variable => 0x2,
            PdoKind::Augmented => 0x3,
        }
    }
}

/// Invalid APDO kind error, contains the raw value that failed to decode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct InvalidApdoKind(pub u8);

impl From<InvalidApdoKind> for PdError {
    fn from(_: InvalidApdoKind) -> Self {
        PdError::InvalidParams
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ApdoKind {
    /// SPR Programable power supply
    SprPps,
    /// EPR Adjustable voltage supply
    EprAvs,
    /// SPR Adjustable voltage supply
    SprAvs,
}

impl From<ApdoKind> for u8 {
    fn from(value: ApdoKind) -> u8 {
        match value {
            ApdoKind::SprPps => 0x0,
            ApdoKind::EprAvs => 0x1,
            ApdoKind::SprAvs => 0x2,
        }
    }
}

impl TryFrom<u8> for ApdoKind {
    type Error = InvalidApdoKind;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x0 => Ok(ApdoKind::SprPps),
            0x1 => Ok(ApdoKind::EprAvs),
            0x2 => Ok(ApdoKind::SprAvs),
            _ => Err(InvalidApdoKind(value)),
        }
    }
}

impl TryFrom<u32> for ApdoKind {
    type Error = InvalidApdoKind;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        const APDO_KIND_SHIFT: u8 = 28;
        const APDO_KIND_MASK: u32 = 0x3;
        let kind = ((value >> APDO_KIND_SHIFT) & APDO_KIND_MASK) as u8;
        match kind {
            0x0 => Ok(ApdoKind::SprPps),
            0x1 => Ok(ApdoKind::EprAvs),
            0x2 => Ok(ApdoKind::SprAvs),
            _ => Err(InvalidApdoKind(kind)),
        }
    }
}

/// Common PDO trait
pub trait Common {
    /// Get the PDO kind
    fn kind(&self) -> PdoKind;
    /// Get the APDO kind
    fn apdo_kind(&self) -> Option<ApdoKind>;
    /// Return true if the PDO is a dual-role power PDO
    fn dual_role_power(&self) -> bool;
    /// Return true if the PDO has unconstrained power
    fn unconstrained_power(&self) -> bool;
}

/// Top-level PDO type
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdo {
    Source(source::Pdo),
    Sink(sink::Pdo),
}

impl Common for Pdo {
    fn kind(&self) -> PdoKind {
        match self {
            Pdo::Source(pdo) => pdo.kind(),
            Pdo::Sink(pdo) => pdo.kind(),
        }
    }

    fn apdo_kind(&self) -> Option<ApdoKind> {
        match self {
            Pdo::Source(pdo) => pdo.apdo_kind(),
            Pdo::Sink(pdo) => pdo.apdo_kind(),
        }
    }

    fn dual_role_power(&self) -> bool {
        match self {
            Pdo::Source(pdo) => pdo.dual_role_power(),
            Pdo::Sink(pdo) => pdo.dual_role_power(),
        }
    }

    fn unconstrained_power(&self) -> bool {
        match self {
            Pdo::Source(pdo) => pdo.unconstrained_power(),
            Pdo::Sink(pdo) => pdo.unconstrained_power(),
        }
    }
}

/// Error type for decoding PDOs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ExpectedPdo {
    /// Expected PDO kind
    pub kind: PdoKind,
    /// Expected APDO kind, if applicable
    pub apdo_kind: Option<ApdoKind>,
    /// Raw PDO value that failed to be decoded
    pub raw: u32,
}

impl From<ExpectedPdo> for PdError {
    fn from(_: ExpectedPdo) -> Self {
        PdError::InvalidParams
    }
}

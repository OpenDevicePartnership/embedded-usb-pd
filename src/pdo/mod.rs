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
/// 25 mV unit
pub const MV25_UNIT: u16 = 25;
/// 50 mV unit
pub const MV50_UNIT: u16 = 50;
/// 100 mV unit
pub const MV100_UNIT: u16 = 100;
/// 250 mV unit
pub const MW250_UNIT: u32 = 250;
/// 500 mW unit
pub const MW500_UNIT: u32 = 500;
/// 1000 mW unit
pub const MW1000_UNIT: u32 = 1000;

/// Length of a PDO in bytes
pub const PDO_LEN: usize = 4;

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
        // NOTE: If this mask changes, the panic safety comment below must be reevaluated
        const PDO_KIND_MASK: u8 = 0x3;
        match value & PDO_KIND_MASK {
            0x0 => PdoKind::Fixed,
            0x1 => PdoKind::Battery,
            0x2 => PdoKind::Variable,
            0x3 => PdoKind::Augmented,
            // Panic safety: This will never panic if the mask above does not change
            #[allow(clippy::unreachable)]
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
pub trait Common: Copy + Clone + PartialEq + Eq + Into<Pdo> + Into<u32> {
    /// Get the PDO kind
    fn kind(&self) -> PdoKind;
    /// Get the APDO kind
    fn apdo_kind(&self) -> Option<ApdoKind>;
    /// Return true if the PDO is a dual-role power PDO
    fn dual_role_power(&self) -> bool;
    /// Return true if the PDO has unconstrained power
    fn unconstrained_power(&self) -> bool;
    /// Max voltage in mV
    fn max_voltage_mv(&self) -> u16;
    /// Min voltage in mV
    fn min_voltage_mv(&self) -> u16;
}

/// This trait is for PDO values that have a definite power role. The power role of a PDO
/// is not contained in the PDO itself so [`Common`] cannot have `TryFrom<u32>` as a supertrait.
pub trait RoleCommon: Common + Default + TryFrom<u32, Error = ExpectedPdo> {}

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

    fn max_voltage_mv(&self) -> u16 {
        match self {
            Pdo::Source(pdo) => pdo.max_voltage_mv(),
            Pdo::Sink(pdo) => pdo.max_voltage_mv(),
        }
    }

    fn min_voltage_mv(&self) -> u16 {
        match self {
            Pdo::Source(pdo) => pdo.min_voltage_mv(),
            Pdo::Sink(pdo) => pdo.min_voltage_mv(),
        }
    }
}

impl From<Pdo> for u32 {
    fn from(value: Pdo) -> Self {
        match value {
            Pdo::Source(data) => data.into(),
            Pdo::Sink(data) => data.into(),
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

/// Full PD contract containing PDO and RDO
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Contract {
    pub pdo: Pdo,
    pub rdo: Rdo,
}

impl Contract {
    /// Create a new contract from a PDO and RDO
    pub fn new(pdo: Pdo, rdo: Rdo) -> Self {
        Contract { pdo, rdo }
    }

    /// Create a new contract from a sink PDO and RDO
    pub fn from_sink(pdo: sink::Pdo, rdo: Rdo) -> Self {
        Contract {
            pdo: Pdo::Sink(pdo),
            rdo,
        }
    }

    /// Create a new contract from a source PDO and RDO
    pub fn from_source(pdo: source::Pdo, rdo: Rdo) -> Self {
        Contract {
            pdo: Pdo::Source(pdo),
            rdo,
        }
    }

    /// Returns the operating current in mA, uses maximum voltage for battery PDO calculation
    /// Returns None on an attempted division by zero
    pub fn operating_current_ma(&self) -> Option<u16> {
        match self.rdo {
            Rdo::Fixed(data) => Some(data.operating_current_ma),
            Rdo::Battery(data) => data
                .operating_power_mw
                .checked_div(self.pdo.max_voltage_mv() as u32)
                .map(|v| (1000 * v) as u16),
            Rdo::Variable(data) => Some(data.operating_current_ma),
            Rdo::Avs(data) => Some(data.operating_current_ma),
            Rdo::Pps(data) => Some(data.operating_current_ma),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::pdo::sink::{BatteryData, EprAvsData, FixedData, SprPpsData};

    #[test]
    fn test_contract_operating_current_ma_fixed() {
        let contract = Contract::from_sink(
            sink::Pdo::Fixed(FixedData {
                operational_current_ma: 3000,
                voltage_mv: 5000,
                dual_role_power: false,
                higher_capability: false,
                unconstrained_power: false,
                usb_comms_capable: false,
                dual_role_data: false,
                frs_required_current: sink::FrsRequiredCurrent::Current1A5,
            }),
            Rdo::Fixed(rdo::FixedVarData {
                operating_current_ma: 1500,
                max_operating_current_ma: 2000,
                object_position: 0,
                capability_mismatch: false,
                usb_comm_capable: false,
                no_usb_suspend: false,
                unchunked_extended_messages_support: false,
                epr_capable: false,
            }),
        );

        assert_eq!(contract.operating_current_ma(), Some(1500));
    }

    #[test]
    fn test_contract_operating_current_ma_battery() {
        let contract = Contract::from_sink(
            sink::Pdo::Battery(BatteryData {
                max_voltage_mv: 20000,
                min_voltage_mv: 15000,
                operational_power_mw: 60000,
            }),
            Rdo::Battery(rdo::BatteryData {
                operating_power_mw: 40000,
                max_operating_power_mw: 45000,
                object_position: 0,
                capability_mismatch: false,
                usb_comm_capable: false,
                no_usb_suspend: false,
                unchunked_extended_messages_support: false,
                epr_capable: false,
            }),
        );

        assert_eq!(contract.operating_current_ma(), Some(2000));
    }

    #[test]
    fn test_contract_operating_current_ma_battery_fail() {
        let contract = Contract::from_sink(
            sink::Pdo::Battery(BatteryData {
                max_voltage_mv: 0,
                min_voltage_mv: 15000,
                operational_power_mw: 60000,
            }),
            Rdo::Battery(rdo::BatteryData {
                operating_power_mw: 40000,
                max_operating_power_mw: 45000,
                object_position: 0,
                capability_mismatch: false,
                usb_comm_capable: false,
                no_usb_suspend: false,
                unchunked_extended_messages_support: false,
                epr_capable: false,
            }),
        );

        assert_eq!(contract.operating_current_ma(), None);
    }

    #[test]
    fn test_contract_operating_current_ma_variable() {
        let contract = Contract::from_sink(
            sink::Pdo::Variable(sink::VariableData {
                operational_current_ma: 3000,
                max_voltage_mv: 20000,
                min_voltage_mv: 15000,
            }),
            Rdo::Variable(rdo::FixedVarData {
                operating_current_ma: 2000,
                max_operating_current_ma: 2500,
                object_position: 0,
                capability_mismatch: false,
                usb_comm_capable: false,
                no_usb_suspend: false,
                unchunked_extended_messages_support: false,
                epr_capable: false,
            }),
        );

        assert_eq!(contract.operating_current_ma(), Some(2000));
    }

    #[test]
    fn test_contract_operating_current_ma_avs() {
        let contract = Contract::from_sink(
            sink::Pdo::Augmented(sink::Apdo::EprAvs(EprAvsData {
                max_voltage_mv: 21000,
                min_voltage_mv: 15000,
                pdp_mw: 3000,
            })),
            Rdo::Avs(rdo::AvsData {
                operating_current_ma: 2500,
                output_voltage_mv: 20000,
                object_position: 0,
                capability_mismatch: false,
                usb_comm_capable: false,
                no_usb_suspend: false,
                unchunked_extended_messages_support: false,
                epr_capable: false,
            }),
        );

        assert_eq!(contract.operating_current_ma(), Some(2500));
    }

    #[test]
    fn test_contract_operating_current_ma_pps() {
        let contract = Contract::from_sink(
            sink::Pdo::Augmented(sink::Apdo::SprPps(SprPpsData {
                max_voltage_mv: 21000,
                min_voltage_mv: 15000,
                max_current_ma: 3000,
            })),
            Rdo::Pps(rdo::PpsData {
                operating_current_ma: 2500,
                output_voltage_mv: 20000,
                object_position: 0,
                capability_mismatch: false,
                usb_comm_capable: false,
                no_usb_suspend: false,
                unchunked_extended_messages_support: false,
                epr_capable: false,
            }),
        );

        assert_eq!(contract.operating_current_ma(), Some(2500));
    }
}

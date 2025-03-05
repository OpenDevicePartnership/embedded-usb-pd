//! Sink PDOs as defined in USB Power Delivery specification rev 3.2 section 6.4.1.3
use bitfield::bitfield;

use super::*;
use crate::PdError;

/// Sink PDO
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdo {
    /// Fixed
    Fixed(FixedData),
    /// Battery supply
    Battery(BatteryData),
    /// Variable supply
    Variable(VariableData),
    /// Augmented supply
    Augmented(Apdo),
}

impl Common for Pdo {
    fn kind(&self) -> PdoKind {
        match self {
            Pdo::Fixed(_) => PdoKind::Fixed,
            Pdo::Battery(_) => PdoKind::Battery,
            Pdo::Variable(_) => PdoKind::Variable,
            Pdo::Augmented(_) => PdoKind::Augmented,
        }
    }

    fn apdo_kind(&self) -> Option<ApdoKind> {
        match self {
            Pdo::Augmented(apdo) => Some(match apdo {
                Apdo::SprPps(_) => ApdoKind::SprPps,
                Apdo::EprAvs(_) => ApdoKind::EprAvs,
                Apdo::SprAvs(_) => ApdoKind::SprAvs,
            }),
            _ => None,
        }
    }

    fn is_dual_role(&self) -> bool {
        match self {
            Pdo::Fixed(data) => data.dual_role_power,
            _ => false,
        }
    }
}

impl TryFrom<u32> for Pdo {
    type Error = PdError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match PdoKind::from(value) {
            PdoKind::Fixed => FixedData::try_from(value).map(Pdo::Fixed),
            PdoKind::Battery => BatteryData::try_from(value).map(Pdo::Battery),
            PdoKind::Variable => VariableData::try_from(value).map(Pdo::Variable),
            PdoKind::Augmented => Apdo::try_from(value).map(Pdo::Augmented),
        }
    }
}

/// FRS required current
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FrsRequiredCurrent {
    /// Not supported
    None,
    /// USB default current
    Default,
    /// 1.5A @ 5V
    Current1A5,
    /// 3A @ 5V
    Current3A,
}

impl From<u8> for FrsRequiredCurrent {
    fn from(value: u8) -> Self {
        const FRS_REQUIRED_CURRENT_MASK: u8 = 0x3;
        match value & FRS_REQUIRED_CURRENT_MASK {
            0 => FrsRequiredCurrent::None,
            1 => FrsRequiredCurrent::Default,
            2 => FrsRequiredCurrent::Current1A5,
            3 => FrsRequiredCurrent::Current3A,
            _ => unreachable!(),
        }
    }
}

bitfield! {
    /// Fixed PDO raw data
    #[derive(Copy, Clone, PartialEq, Eq)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub struct FixedRaw(u32);
    impl Debug;

    /// PDO kind
    pub u8, kind, set_kind: 31, 30;
    /// Dual role power capable
    pub u8, dual_role_power, set_dual_role_power: 29, 29;
    /// Higher capability
    pub u8, higher_capability, set_higher_capability: 28, 28;
    /// Unconstrained power
    pub u8, unconstrained_power, set_unconstrained_power: 27, 27;
    /// USB comms capable
    pub u8, usb_comms_capable, set_usb_comms_capable: 26, 26;
    /// Dual role data capable
    pub u8, dual_role_data, set_dual_role_data: 25, 25;
    /// Required FRS current
    pub u8, frs_required_current, set_frs_required_current: 24, 23;
    /// Voltage in 50mV units
    pub u16, voltage, set_voltage: 19, 10;
    /// Operating current in 10mA units
    pub u16, operational_current, set_operational_current: 9, 0;
}

/// Fixed supply data
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct FixedData {
    /// Dual role power
    pub dual_role_power: bool,
    /// Higher capability
    pub higher_capability: bool,
    /// Unconstrained power
    pub unconstrained_power: bool,
    /// USB comms capable
    pub usb_comms_capable: bool,
    /// Dual role data capable
    pub dual_role_data: bool,
    /// FRS required current
    pub frs_required_current: FrsRequiredCurrent,
    /// Voltage in mV
    pub voltage_mv: u16,
    /// Operational current in mA
    pub operational_current_ma: u16,
}

impl TryFrom<u32> for FixedData {
    type Error = PdError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if PdoKind::from(value) != PdoKind::Fixed {
            return Err(PdError::InvalidParams);
        }

        let raw = FixedRaw(value);
        Ok(FixedData {
            dual_role_power: raw.dual_role_power() != 0,
            higher_capability: raw.higher_capability() != 0,
            unconstrained_power: raw.unconstrained_power() != 0,
            usb_comms_capable: raw.usb_comms_capable() != 0,
            dual_role_data: raw.dual_role_data() != 0,
            frs_required_current: raw.frs_required_current().into(),
            voltage_mv: raw.voltage() * MV50_UNIT,
            operational_current_ma: raw.operational_current() * MA10_UNIT,
        })
    }
}

bitfield! {
    /// Raw battery PDO data
    #[derive(Copy, Clone, PartialEq, Eq)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub struct BatteryRaw(u32);
    impl Debug;

    /// PDO kind
    pub u8, kind, set_kind: 31, 30;
    /// Maximum voltage in 50 mV units
    pub u16, max_voltage, set_max_voltage: 29, 20;
    /// Minimum voltage in 50 mV units
    pub u16, min_voltage, set_min_voltage: 19, 10;
    /// Operational power in 250 mW units
    pub u32, operational_power, set_operational_power: 9, 0;
}

/// Battery supply data
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct BatteryData {
    /// Maximum voltage in mV
    pub max_voltage_mv: u16,
    /// Minimum voltage in mV
    pub min_voltage_mv: u16,
    /// Operational power in mW
    pub operational_power_mw: u32,
}

impl TryFrom<u32> for BatteryData {
    type Error = PdError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if PdoKind::from(value) != PdoKind::Battery {
            return Err(PdError::InvalidParams);
        }

        let raw = BatteryRaw(value);
        Ok(BatteryData {
            max_voltage_mv: raw.max_voltage() * MV50_UNIT,
            min_voltage_mv: raw.min_voltage() * MV50_UNIT,
            operational_power_mw: raw.operational_power() * MW250_UNIT,
        })
    }
}

bitfield! {
    /// Raw variable supply PDO data
    #[derive(Copy, Clone, PartialEq, Eq)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub struct VariableRaw(u32);
    impl Debug;

    /// PDO kind
    pub u8, kind, set_kind: 31, 30;
    /// Maximum voltage in 50 mV units
    pub u16, max_voltage, set_max_voltage: 29, 20;
    /// Minimum voltage in 50 mV units
    pub u16, min_voltage, set_min_voltage: 19, 10;
    ///  current in 10 mA units
    pub u16, operational_current, set_operational_current: 9, 0;
}

/// Variable supply data
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct VariableData {
    /// Maximum voltage in mV
    pub max_voltage_mv: u16,
    /// Minimum voltage in mV
    pub min_voltage_mv: u16,
    /// Operational current in mA
    pub operational_current_ma: u16,
}

impl TryFrom<u32> for VariableData {
    type Error = PdError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if PdoKind::from(value) != PdoKind::Variable {
            return Err(PdError::InvalidParams);
        }

        let raw = VariableRaw(value);
        Ok(VariableData {
            max_voltage_mv: raw.max_voltage() * MV50_UNIT,
            min_voltage_mv: raw.min_voltage() * MV50_UNIT,
            operational_current_ma: raw.operational_current() * MA10_UNIT,
        })
    }
}

/// Augmented PDO
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Apdo {
    /// SPR Programable power supply
    SprPps(SprPpsData),
    /// EPR Adjustable voltage supply
    EprAvs(EprAvsData),
    /// SPR Adjustable voltage supply
    SprAvs(SprAvsData),
}

impl TryFrom<u32> for Apdo {
    type Error = PdError;

    fn try_from(value: u32) -> Result<Self, PdError> {
        match ApdoKind::try_from(value)? {
            ApdoKind::SprPps => SprPpsData::try_from(value).map(Apdo::SprPps),
            ApdoKind::EprAvs => EprAvsData::try_from(value).map(Apdo::EprAvs),
            ApdoKind::SprAvs => SprAvsData::try_from(value).map(Apdo::SprAvs),
        }
    }
}

bitfield! {
    /// Raw SPR Programable power supply data
    #[derive(Copy, Clone, PartialEq, Eq)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub struct SprPpsRaw(u32);
    impl Debug;

    /// PDO kind
    pub u8, kind, set_kind: 31, 30;
    /// APDO kind
    pub u8, apdo_kind, set_apdo_kind: 29, 28;
    /// Maximum voltage in 100mV units
    pub u16, max_voltage, set_max_voltage: 24, 17;
    /// Minimum voltage in 100mV units
    pub u16, min_voltage, set_min_voltage: 15, 8;
    /// Maximum current in 50mA units
    pub u16, max_current, set_max_current: 6, 0;
}

/// ADO SPR Programable power supply data
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SprPpsData {
    /// Maximum voltage in mV
    pub max_voltage_mv: u16,
    /// Minimum voltage in mV
    pub min_voltage_mv: u16,
    /// Maximum current in mA
    pub max_current_ma: u16,
}

impl TryFrom<u32> for SprPpsData {
    type Error = PdError;

    fn try_from(value: u32) -> Result<Self, PdError> {
        if PdoKind::from(value) != PdoKind::Augmented || ApdoKind::try_from(value)? != ApdoKind::SprPps {
            return Err(PdError::InvalidParams);
        }

        let raw = SprPpsRaw(value);
        Ok(SprPpsData {
            max_voltage_mv: raw.max_voltage() * MV100_UNIT,
            min_voltage_mv: raw.min_voltage() * MV100_UNIT,
            max_current_ma: raw.max_current() * MA50_UNIT,
        })
    }
}

bitfield! {
    /// Raw EPR Adjustable voltage supply data
    #[derive(Copy, Clone, PartialEq, Eq)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub struct EprAvsRaw(u32);
    impl Debug;

    /// PDO kind
    pub u8, kind, set_kind: 31, 30;
    /// APDO kind
    pub u8, apdo_kind, set_apdo_kind: 29, 28;
    /// Maximum voltage in 100mV units
    pub u16, max_voltage, set_max_voltage: 25, 17;
    /// Minimum voltage in 100mV units
    pub u16, min_voltage, set_min_voltage: 15, 8;
    /// PDP in 1W units
    pub u32, pdp, set_pdp: 7, 0;
}

/// EPR Adjustable voltage supply data
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct EprAvsData {
    /// Maximum voltage in mV
    pub max_voltage_mv: u16,
    /// Minimum voltage in mV
    pub min_voltage_mv: u16,
    /// PDP in mW
    pub pdp_mw: u32,
}

impl TryFrom<u32> for EprAvsData {
    type Error = PdError;

    fn try_from(value: u32) -> Result<Self, PdError> {
        if PdoKind::from(value) != PdoKind::Augmented || ApdoKind::try_from(value)? != ApdoKind::EprAvs {
            return Err(PdError::InvalidParams);
        }

        let raw = EprAvsRaw(value);
        Ok(EprAvsData {
            max_voltage_mv: raw.max_voltage() * MV100_UNIT,
            min_voltage_mv: raw.min_voltage() * MV100_UNIT,
            pdp_mw: raw.pdp() * MW1000_UNIT,
        })
    }
}

bitfield! {
    /// Raw SPR adjustable voltage supply
    #[derive(Copy, Clone, PartialEq, Eq)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub struct SprAvsRaw(u32);
    impl Debug;

    /// PDO kind
    pub u8, kind, set_kind: 31, 30;
    /// APDO kind
    pub u8, apdo_kind, set_apdo_kind: 29, 28;
    /// Maximum current for 9-15 V range in 10mA units
    pub u16, max_current_15v, set_max_current_15v: 19, 10;
    /// Maximum current for 15-20 V range in 10mA units
    pub u16, max_current_20v, set_max_current_20v: 9, 0;
}

/// SPR Adjustable voltage supply data
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SprAvsData {
    /// Maximum current for 9-15 V range in mA
    pub max_current_15v_ma: u16,
    /// Maximum current for 15-20 V range in mA
    pub max_current_20v_ma: u16,
}

impl TryFrom<u32> for SprAvsData {
    type Error = PdError;

    fn try_from(value: u32) -> Result<Self, PdError> {
        if PdoKind::from(value) != PdoKind::Augmented || ApdoKind::try_from(value)? != ApdoKind::SprAvs {
            return Err(PdError::InvalidParams);
        }

        let raw = SprAvsRaw(value);
        Ok(SprAvsData {
            max_current_15v_ma: raw.max_current_15v() * MA10_UNIT,
            max_current_20v_ma: raw.max_current_20v() * MA10_UNIT,
        })
    }
}

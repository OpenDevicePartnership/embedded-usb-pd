//! Source PDOs as defined in USB Power Delivery specification rev 3.2 section 6.4.1.2
use bitfield::bitfield;

use super::*;
use crate::PdError;

/// Power data object type
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdo {
    /// Fixed supply
    Fixed(FixedData),
    /// Battery
    Battery(BatteryData),
    /// Variable supply
    Variable(VariableData),
    /// Augmented fixed supply
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

/// Fixed supply peak current, names based on 10 ms @ 50% duty cycle values
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PeakCurrent {
    Pct100,
    Pct110,
    Pct125,
    Pct150,
}

const PEAK_CURRENT_MASK: u8 = 0x3;
impl From<u8> for PeakCurrent {
    fn from(value: u8) -> Self {
        match value & PEAK_CURRENT_MASK {
            0x0 => PeakCurrent::Pct100,
            0x1 => PeakCurrent::Pct110,
            0x2 => PeakCurrent::Pct125,
            0x3 => PeakCurrent::Pct150,
            _ => unreachable!(),
        }
    }
}

impl From<PeakCurrent> for u8 {
    fn from(value: PeakCurrent) -> Self {
        match value {
            PeakCurrent::Pct100 => 0x0,
            PeakCurrent::Pct110 => 0x1,
            PeakCurrent::Pct125 => 0x2,
            PeakCurrent::Pct150 => 0x3,
        }
    }
}

bitfield! {
    /// Raw fixed supply PDO data
    #[derive(Copy, Clone, PartialEq, Eq)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub struct FixedRaw(u32);
    impl Debug;

    /// PDO kind
    pub u8, kind, set_kind: 31, 30;
    /// Dual role power capable
    pub u8, dual_role_power, set_dual_role_power: 29, 29;
    /// USB suspend supported
    pub u8, usb_suspend_supported, set_usb_suspend_supported: 28, 28;
    /// Unconstrained power
    pub u8, unconstrained_power, set_unconstrained_power: 27, 27;
    /// USB comms capable
    pub u8, usb_comms_capable, set_usb_comms_capable: 26, 26;
    /// Dual role data capable
    pub u8, dual_role_data, set_dual_role_data: 25, 25;
    /// Unchunked extended messages support
    pub u8, unchunked_extended_messages_support, set_unchunked_extended_messages_support: 24, 24;
    /// EPR capable
    pub u8, epr_capable, set_epr_capable: 23, 23;
    /// Peak current
    pub u8, peak_current, set_peak_current: 21, 20;
    /// Voltage in 50 mV units
    pub u16, voltage, set_voltage: 19, 10;
    /// Peak current in 10 mA units
    pub u16, current, set_current: 9, 0;
}

/// Fixed supply PDO data
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct FixedData {
    /// Dual role power capable
    pub dual_role_power: bool,
    /// USB suspend supported
    pub usb_suspend_supported: bool,
    /// Unconstrained power
    pub unconstrained_power: bool,
    /// USB comms capable
    pub usb_comms_capable: bool,
    /// Dual role data capable
    pub dual_role_data: bool,
    /// Unchunked extended messages support
    pub unchunked_extended_messages_support: bool,
    /// EPR capable
    pub epr_capable: bool,
    /// Peak current
    pub peak_current: PeakCurrent,
    /// Voltage in mV
    pub voltage_mv: u16,
    /// Current in mA
    pub current_ma: u16,
}

impl From<FixedRaw> for FixedData {
    fn from(raw: FixedRaw) -> Self {
        FixedData {
            dual_role_power: raw.dual_role_power() != 0,
            usb_suspend_supported: raw.usb_suspend_supported() != 0,
            unconstrained_power: raw.unconstrained_power() != 0,
            usb_comms_capable: raw.usb_comms_capable() != 0,
            dual_role_data: raw.dual_role_data() != 0,
            unchunked_extended_messages_support: raw.unchunked_extended_messages_support() != 0,
            epr_capable: raw.epr_capable() != 0,
            peak_current: raw.peak_current().into(),
            voltage_mv: raw.voltage() * MV50_UNIT,
            current_ma: raw.current() * MA10_UNIT,
        }
    }
}

impl TryFrom<u32> for FixedData {
    type Error = PdError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if PdoKind::from(value) != PdoKind::Fixed {
            return Err(PdError::InvalidParams);
        }
        Ok(FixedRaw(value).into())
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
    /// Maximum power in 250 mW units
    pub u32, max_power, set_max_power: 9, 0;
}

/// Battery PDO data
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct BatteryData {
    /// Maximum voltage in mV
    pub max_voltage_mv: u16,
    /// Minimum voltage in mV
    pub min_voltage_mv: u16,
    /// Maximum power in mW
    pub max_power_mw: u32,
}

impl From<BatteryRaw> for BatteryData {
    fn from(raw: BatteryRaw) -> Self {
        BatteryData {
            max_voltage_mv: raw.max_voltage() * MV50_UNIT,
            min_voltage_mv: raw.min_voltage() * MV50_UNIT,
            max_power_mw: raw.max_power() * MW250_UNIT,
        }
    }
}

impl TryFrom<u32> for BatteryData {
    type Error = PdError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if PdoKind::from(value) != PdoKind::Battery {
            return Err(PdError::InvalidParams);
        }
        Ok(BatteryRaw(value).into())
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
    /// Maximum current in 10 mA units
    pub u16, max_current, set_max_current: 9, 0;
}

/// Variable supply PDO data
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct VariableData {
    /// Maximum voltage in mV
    pub max_voltage_mv: u16,
    /// Minimum voltage in mV
    pub min_voltage_mv: u16,
    /// Maximum current in mA
    pub max_current_ma: u16,
}

impl From<VariableRaw> for VariableData {
    fn from(raw: VariableRaw) -> Self {
        VariableData {
            max_voltage_mv: raw.max_voltage() * MV50_UNIT,
            min_voltage_mv: raw.min_voltage() * MA50_UNIT,
            max_current_ma: raw.max_current() * MA10_UNIT,
        }
    }
}

impl TryFrom<u32> for VariableData {
    type Error = PdError;

    fn try_from(value: u32) -> Result<Self, PdError> {
        if PdoKind::from(value) != PdoKind::Variable {
            return Err(PdError::InvalidParams);
        }
        Ok(VariableRaw(value).into())
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
    /// PPS power limited
    pub u8, pps_power_limited, set_pps_power_limited: 27, 27;
    /// Maximum voltage in 100mV units
    pub u16, max_voltage, set_max_voltage: 24, 17;
    /// Minimum voltage in 100mV units
    pub u16, min_voltage, set_min_voltage: 15, 8;
    /// Maximum current in 50mA units
    pub u16, max_current, set_max_current: 6, 0;
}

/// SPR Programable power supply data
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SprPpsData {
    /// PPS power limited
    pub pps_power_limited: bool,
    /// Maximum voltage in mV
    pub max_voltage_mv: u16,
    /// Minimum voltage in mV
    pub min_voltage_mv: u16,
    /// Maximum current in mA
    pub max_current_ma: u16,
}

impl From<SprPpsRaw> for SprPpsData {
    fn from(raw: SprPpsRaw) -> Self {
        SprPpsData {
            pps_power_limited: raw.pps_power_limited() != 0,
            max_voltage_mv: raw.max_voltage() * MV100_UNIT,
            min_voltage_mv: raw.min_voltage() * MV100_UNIT,
            max_current_ma: raw.max_current() * MA50_UNIT,
        }
    }
}

impl TryFrom<u32> for SprPpsData {
    type Error = PdError;

    fn try_from(value: u32) -> Result<Self, PdError> {
        if PdoKind::from(value) != PdoKind::Augmented || ApdoKind::try_from(value)? != ApdoKind::SprPps {
            return Err(PdError::InvalidParams);
        }

        Ok(SprPpsRaw(value).into())
    }
}

bitfield! {
    /// Raw EPR adjustable voltage supply data
    #[derive(Copy, Clone, PartialEq, Eq)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub struct EprAvsRaw(u32);
    impl Debug;

    /// PDO kind
    pub u8, kind, set_kind: 31, 30;
    /// APDO kind
    pub u8, apdo_kind, set_apdo_kind: 29, 28;
    /// Peak current
    pub u8, peak_current, set_peak_current: 27, 26;
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
    /// Peak current
    pub peak_current: PeakCurrent,
    /// Maximum voltage in mV
    pub max_voltage_mv: u16,
    /// Minimum voltage in mV
    pub min_voltage_mv: u16,
    /// PDP in mW
    pub pdp_mw: u32,
}

impl From<EprAvsRaw> for EprAvsData {
    fn from(raw: EprAvsRaw) -> Self {
        EprAvsData {
            peak_current: raw.peak_current().into(),
            max_voltage_mv: raw.max_voltage() * MV100_UNIT,
            min_voltage_mv: raw.min_voltage() * MV100_UNIT,
            pdp_mw: raw.pdp() * MW1000_UNIT,
        }
    }
}

impl TryFrom<u32> for EprAvsData {
    type Error = PdError;

    fn try_from(value: u32) -> Result<Self, PdError> {
        if PdoKind::from(value) != PdoKind::Augmented || ApdoKind::try_from(value)? != ApdoKind::EprAvs {
            return Err(PdError::InvalidParams);
        }

        Ok(EprAvsRaw(value).into())
    }
}

bitfield! {
    /// Raw SPR adjustable voltage supply data
    #[derive(Copy, Clone, PartialEq, Eq)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub struct SprAvsRaw(u32);
    impl Debug;

    /// PDO kind
    pub u8, kind, set_kind: 31, 30;
    /// APDO kind
    pub u8, apdo_kind, set_apdo_kind: 29, 28;
    /// Peak current
    pub u8, peak_current, set_peak_current: 27, 26;
    /// Maximum current for 9-15 V range in 10mA units
    pub u16, max_current_15v, set_max_current_15v: 19, 10;
    /// Maximum current for 15-20 V range in 10mA units
    pub u16, max_current_20v, set_max_current_20v: 9, 0;
}

/// SPR Adjustable voltage supply data
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SprAvsData {
    /// Peak current
    pub peak_current: PeakCurrent,
    /// Maximum current for 9-15 V range in mA
    pub max_current_15v_ma: u16,
    /// Maximum current for 15-20 V range in mA
    pub max_current_20v_ma: u16,
}

impl From<SprAvsRaw> for SprAvsData {
    fn from(raw: SprAvsRaw) -> Self {
        SprAvsData {
            peak_current: raw.peak_current().into(),
            max_current_15v_ma: raw.max_current_15v() * MA10_UNIT,
            max_current_20v_ma: raw.max_current_20v() * MA10_UNIT,
        }
    }
}

impl TryFrom<u32> for SprAvsData {
    type Error = PdError;

    fn try_from(value: u32) -> Result<Self, PdError> {
        if PdoKind::from(value) != PdoKind::Augmented || ApdoKind::try_from(value)? != ApdoKind::SprAvs {
            return Err(PdError::InvalidParams);
        }

        Ok(SprAvsRaw(value).into())
    }
}

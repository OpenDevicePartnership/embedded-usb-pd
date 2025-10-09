//! Source PDOs as defined in USB Power Delivery specification rev 3.2 section 6.4.1.2
use bitfield::bitfield;

use super::*;

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

impl Default for Pdo {
    fn default() -> Self {
        Pdo::Fixed(FixedData::default())
    }
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

    fn dual_role_power(&self) -> bool {
        match self {
            Pdo::Fixed(data) => data.flags.dual_role_power,
            _ => false,
        }
    }

    fn unconstrained_power(&self) -> bool {
        match self {
            Pdo::Fixed(data) => data.flags.unconstrained_power,
            _ => false,
        }
    }

    fn max_voltage_mv(&self) -> u16 {
        match self {
            Pdo::Fixed(data) => data.voltage_mv,
            Pdo::Battery(data) => data.max_voltage_mv,
            Pdo::Variable(data) => data.max_voltage_mv,
            Pdo::Augmented(apdo) => match apdo {
                Apdo::SprPps(data) => data.max_voltage_mv,
                Apdo::EprAvs(data) => data.max_voltage_mv,
                // 20V maximum only if 15-20V range is supported
                Apdo::SprAvs(data) => {
                    if data.max_current_20v_ma > 0 {
                        20000
                    } else {
                        15000
                    }
                }
            },
        }
    }

    fn min_voltage_mv(&self) -> u16 {
        match self {
            Pdo::Fixed(data) => data.voltage_mv,
            Pdo::Battery(data) => data.min_voltage_mv,
            Pdo::Variable(data) => data.min_voltage_mv,
            Pdo::Augmented(apdo) => match apdo {
                Apdo::SprPps(data) => data.min_voltage_mv,
                Apdo::EprAvs(data) => data.min_voltage_mv,
                // 15V minimum only if 15-20V range is supported
                Apdo::SprAvs(data) => {
                    if data.max_current_20v_ma > 0 {
                        15000
                    } else {
                        9000
                    }
                }
            },
        }
    }
}

impl From<Pdo> for super::Pdo {
    fn from(pdo: Pdo) -> Self {
        super::Pdo::Source(pdo)
    }
}

impl RoleCommon for Pdo {}

impl TryFrom<u32> for Pdo {
    type Error = ExpectedPdo;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match PdoKind::from(value) {
            PdoKind::Fixed => FixedData::try_from(value).map(Pdo::Fixed),
            PdoKind::Battery => BatteryData::try_from(value).map(Pdo::Battery),
            PdoKind::Variable => VariableData::try_from(value).map(Pdo::Variable),
            PdoKind::Augmented => Apdo::try_from(value).map(Pdo::Augmented),
        }
    }
}

impl From<Pdo> for u32 {
    fn from(value: Pdo) -> Self {
        match value {
            Pdo::Fixed(data) => data.into(),
            Pdo::Battery(data) => data.into(),
            Pdo::Variable(data) => data.into(),
            Pdo::Augmented(data) => data.into(),
        }
    }
}

/// Fixed supply peak current, names based on 10 ms @ 50% duty cycle values
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PeakCurrent {
    /// 100% of nominal current
    #[default]
    Pct100,
    /// 110% of nominal current
    Pct110,
    /// 125% of nominal current
    Pct125,
    /// 150% of nominal current
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
    /// Raw fixed supply flags
    #[derive(Copy, Clone, PartialEq, Eq)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub struct FixedFlagsRaw(u8);
    impl Debug;

     /// Dual role power capable
    pub bool, dual_role_power, set_dual_role_power: 6;
    /// USB suspend supported
    pub bool, usb_suspend_supported, set_usb_suspend_supported: 5;
    /// Unconstrained power
    pub bool, unconstrained_power, set_unconstrained_power: 4;
    /// USB comms capable
    pub bool, usb_comms_capable, set_usb_comms_capable: 3;
    /// Dual role data capable
    pub bool, dual_role_data, set_dual_role_data: 2;
    /// Unchunked extended messages support
    pub bool, unchunked_extended_messages_support, set_unchunked_extended_messages_support: 1;
    /// EPR capable
    pub bool, epr_capable, set_epr_capable: 0;
}

impl From<u8> for FixedFlagsRaw {
    fn from(value: u8) -> Self {
        FixedFlagsRaw(value)
    }
}

/// Fixed supply flags
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct FixedFlags {
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
}

impl From<FixedFlagsRaw> for FixedFlags {
    fn from(raw: FixedFlagsRaw) -> Self {
        FixedFlags {
            dual_role_power: raw.dual_role_power(),
            usb_suspend_supported: raw.usb_suspend_supported(),
            unconstrained_power: raw.unconstrained_power(),
            usb_comms_capable: raw.usb_comms_capable(),
            dual_role_data: raw.dual_role_data(),
            unchunked_extended_messages_support: raw.unchunked_extended_messages_support(),
            epr_capable: raw.epr_capable(),
        }
    }
}

impl From<u8> for FixedFlags {
    fn from(value: u8) -> Self {
        FixedFlagsRaw::from(value).into()
    }
}

impl From<FixedFlags> for u8 {
    fn from(value: FixedFlags) -> Self {
        let mut raw = FixedFlagsRaw(0);
        raw.set_dual_role_power(value.dual_role_power);
        raw.set_usb_suspend_supported(value.usb_suspend_supported);
        raw.set_unconstrained_power(value.unconstrained_power);
        raw.set_usb_comms_capable(value.usb_comms_capable);
        raw.set_dual_role_data(value.dual_role_data);
        raw.set_unchunked_extended_messages_support(value.unchunked_extended_messages_support);
        raw.set_epr_capable(value.epr_capable);
        raw.0
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
    // Fixed PDO flags
    pub u8, flags, set_flags: 29, 23;
    /// Peak current
    pub u8, peak_current, set_peak_current: 21, 20;
    /// Voltage in 50 mV units
    pub u16, voltage, set_voltage: 19, 10;
    /// Peak current in 10 mA units
    pub u16, current, set_current: 9, 0;
}

/// Fixed supply PDO data
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct FixedData {
    /// Fixed flags
    pub flags: FixedFlags,
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
            flags: FixedFlags::from(raw.flags()),
            peak_current: raw.peak_current().into(),
            voltage_mv: raw.voltage() * MV50_UNIT,
            current_ma: raw.current() * MA10_UNIT,
        }
    }
}

impl TryFrom<u32> for FixedData {
    type Error = ExpectedPdo;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if PdoKind::from(value) == PdoKind::Fixed {
            Ok(FixedRaw(value).into())
        } else {
            Err(ExpectedPdo {
                kind: PdoKind::Fixed,
                apdo_kind: None,
                raw: value,
            })
        }
    }
}

impl From<FixedData> for u32 {
    fn from(data: FixedData) -> Self {
        let mut raw = FixedRaw(0);
        raw.set_kind(PdoKind::Fixed as u8);
        raw.set_flags(data.flags.into());
        raw.set_peak_current(data.peak_current.into());
        raw.set_voltage(data.voltage_mv / MV50_UNIT);
        raw.set_current(data.current_ma / MA10_UNIT);
        raw.0
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
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
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
    type Error = ExpectedPdo;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if PdoKind::from(value) == PdoKind::Battery {
            Ok(BatteryRaw(value).into())
        } else {
            Err(ExpectedPdo {
                kind: PdoKind::Battery,
                apdo_kind: None,
                raw: value,
            })
        }
    }
}

impl From<BatteryData> for u32 {
    fn from(data: BatteryData) -> Self {
        let mut raw = BatteryRaw(0);
        raw.set_kind(PdoKind::Battery as u8);
        raw.set_max_voltage(data.max_voltage_mv / MV50_UNIT);
        raw.set_min_voltage(data.min_voltage_mv / MV50_UNIT);
        raw.set_max_power(data.max_power_mw / MW250_UNIT);
        raw.0
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
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
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
    type Error = ExpectedPdo;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if PdoKind::from(value) == PdoKind::Variable {
            Ok(VariableRaw(value).into())
        } else {
            Err(ExpectedPdo {
                kind: PdoKind::Variable,
                apdo_kind: None,
                raw: value,
            })
        }
    }
}

impl From<VariableData> for u32 {
    fn from(data: VariableData) -> Self {
        let mut raw = VariableRaw(0);
        raw.set_kind(PdoKind::Variable as u8);
        raw.set_max_voltage(data.max_voltage_mv / MV50_UNIT);
        raw.set_min_voltage(data.min_voltage_mv / MV50_UNIT);
        raw.set_max_current(data.max_current_ma / MA10_UNIT);
        raw.0
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

impl Default for Apdo {
    fn default() -> Self {
        Apdo::SprPps(SprPpsData::default())
    }
}

impl TryFrom<u32> for Apdo {
    type Error = ExpectedPdo;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match ApdoKind::try_from(value).map_err(|_| ExpectedPdo {
            kind: PdoKind::Augmented,
            apdo_kind: None,
            raw: value,
        })? {
            ApdoKind::SprPps => SprPpsData::try_from(value).map(Apdo::SprPps),
            ApdoKind::EprAvs => EprAvsData::try_from(value).map(Apdo::EprAvs),
            ApdoKind::SprAvs => SprAvsData::try_from(value).map(Apdo::SprAvs),
        }
    }
}

impl From<Apdo> for u32 {
    fn from(value: Apdo) -> Self {
        match value {
            Apdo::SprPps(data) => data.into(),
            Apdo::EprAvs(data) => data.into(),
            Apdo::SprAvs(data) => data.into(),
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
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
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
    type Error = ExpectedPdo;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match (PdoKind::from(value), ApdoKind::try_from(value)) {
            (PdoKind::Augmented, Ok(ApdoKind::SprPps)) => Ok(SprPpsRaw(value).into()),
            _ => Err(ExpectedPdo {
                kind: PdoKind::Augmented,
                apdo_kind: Some(ApdoKind::SprPps),
                raw: value,
            }),
        }
    }
}

impl From<SprPpsData> for u32 {
    fn from(data: SprPpsData) -> Self {
        let mut raw = SprPpsRaw(0);
        raw.set_kind(PdoKind::Augmented as u8);
        raw.set_apdo_kind(ApdoKind::SprPps as u8);
        raw.set_pps_power_limited(data.pps_power_limited as u8);
        raw.set_max_voltage(data.max_voltage_mv / MV100_UNIT);
        raw.set_min_voltage(data.min_voltage_mv / MV100_UNIT);
        raw.set_max_current(data.max_current_ma / MA50_UNIT);
        raw.0
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
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
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
    type Error = ExpectedPdo;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match (PdoKind::from(value), ApdoKind::try_from(value)) {
            (PdoKind::Augmented, Ok(ApdoKind::EprAvs)) => Ok(EprAvsRaw(value).into()),
            _ => Err(ExpectedPdo {
                kind: PdoKind::Augmented,
                apdo_kind: Some(ApdoKind::EprAvs),
                raw: value,
            }),
        }
    }
}

impl From<EprAvsData> for u32 {
    fn from(data: EprAvsData) -> Self {
        let mut raw = EprAvsRaw(0);
        raw.set_kind(PdoKind::Augmented as u8);
        raw.set_apdo_kind(ApdoKind::EprAvs as u8);
        raw.set_peak_current(data.peak_current.into());
        raw.set_max_voltage(data.max_voltage_mv / MV100_UNIT);
        raw.set_min_voltage(data.min_voltage_mv / MV100_UNIT);
        raw.set_pdp(data.pdp_mw / MW1000_UNIT);
        raw.0
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
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
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
    type Error = ExpectedPdo;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match (PdoKind::from(value), ApdoKind::try_from(value)) {
            (PdoKind::Augmented, Ok(ApdoKind::SprAvs)) => Ok(SprAvsRaw(value).into()),
            _ => Err(ExpectedPdo {
                kind: PdoKind::Augmented,
                apdo_kind: Some(ApdoKind::SprAvs),
                raw: value,
            }),
        }
    }
}

impl From<SprAvsData> for u32 {
    fn from(data: SprAvsData) -> Self {
        let mut raw = SprAvsRaw(0);
        raw.set_kind(PdoKind::Augmented as u8);
        raw.set_apdo_kind(ApdoKind::SprAvs as u8);
        raw.set_peak_current(data.peak_current.into());
        raw.set_max_current_15v(data.max_current_15v_ma / MA10_UNIT);
        raw.set_max_current_20v(data.max_current_20v_ma / MA10_UNIT);
        raw.0
    }
}

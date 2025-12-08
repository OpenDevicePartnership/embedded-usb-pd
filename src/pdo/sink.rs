//! Sink PDOs as defined in USB Power Delivery specification rev 3.2 section 6.4.1.3
use bitfield::bitfield;

use super::*;

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
            Pdo::Fixed(data) => data.dual_role_power,
            _ => false,
        }
    }

    fn unconstrained_power(&self) -> bool {
        match self {
            Pdo::Fixed(data) => data.unconstrained_power,
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
        super::Pdo::Sink(pdo)
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
            PdoKind::Augmented => Apdo::try_from(value).map(Pdo::Augmented).map_err(|_| ExpectedPdo {
                kind: PdoKind::Augmented,
                apdo_kind: None,
                raw: value,
            }),
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

/// FRS required current
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FrsRequiredCurrent {
    #[default]
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

impl From<FrsRequiredCurrent> for u8 {
    fn from(value: FrsRequiredCurrent) -> Self {
        match value {
            FrsRequiredCurrent::None => 0,
            FrsRequiredCurrent::Default => 1,
            FrsRequiredCurrent::Current1A5 => 2,
            FrsRequiredCurrent::Current3A => 3,
        }
    }
}

bitfield! {
    /// Fixed PDO raw data
    #[derive(Copy, Clone, Default, PartialEq, Eq)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    struct FixedRaw(u32);
    impl Debug;

    /// PDO kind
    pub u8, kind, set_kind: 31, 30;
    /// Dual role power capable
    pub bool, dual_role_power, set_dual_role_power: 29;
    /// Higher capability
    pub bool, higher_capability, set_higher_capability: 28;
    /// Unconstrained power
    pub bool, unconstrained_power, set_unconstrained_power: 27;
    /// USB comms capable
    pub bool, usb_comms_capable, set_usb_comms_capable: 26;
    /// Dual role data capable
    pub bool, dual_role_data, set_dual_role_data: 25;
    /// Required FRS current
    pub u8, frs_required_current, set_frs_required_current: 24, 23;
    /// Voltage in 50mV units
    pub u16, voltage, set_voltage: 19, 10;
    /// Operating current in 10mA units
    pub u16, operational_current, set_operational_current: 9, 0;
}

/// Fixed supply data
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
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

impl From<FixedRaw> for FixedData {
    fn from(raw: FixedRaw) -> Self {
        FixedData {
            dual_role_power: raw.dual_role_power(),
            higher_capability: raw.higher_capability(),
            unconstrained_power: raw.unconstrained_power(),
            usb_comms_capable: raw.usb_comms_capable(),
            dual_role_data: raw.dual_role_data(),
            frs_required_current: FrsRequiredCurrent::from(raw.frs_required_current()),
            voltage_mv: raw.voltage() * MV50_UNIT,
            operational_current_ma: raw.operational_current() * MA10_UNIT,
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
        raw.set_dual_role_power(data.dual_role_power);
        raw.set_higher_capability(data.higher_capability);
        raw.set_unconstrained_power(data.unconstrained_power);
        raw.set_usb_comms_capable(data.usb_comms_capable);
        raw.set_dual_role_data(data.dual_role_data);
        raw.set_frs_required_current(data.frs_required_current.into());
        raw.set_voltage(data.voltage_mv / MV50_UNIT);
        raw.set_operational_current(data.operational_current_ma / MA10_UNIT);
        raw.0
    }
}

bitfield! {
    /// Raw battery PDO data
    #[derive(Copy, Clone, PartialEq, Eq)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    struct BatteryRaw(u32);
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
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct BatteryData {
    /// Maximum voltage in mV
    pub max_voltage_mv: u16,
    /// Minimum voltage in mV
    pub min_voltage_mv: u16,
    /// Operational power in mW
    pub operational_power_mw: u32,
}

impl From<BatteryRaw> for BatteryData {
    fn from(raw: BatteryRaw) -> Self {
        BatteryData {
            max_voltage_mv: raw.max_voltage() * MV50_UNIT,
            min_voltage_mv: raw.min_voltage() * MV50_UNIT,
            operational_power_mw: raw.operational_power() * MW250_UNIT,
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
        raw.set_operational_power(data.operational_power_mw / MW250_UNIT);
        raw.0
    }
}

bitfield! {
    /// Raw variable supply PDO data
    #[derive(Copy, Clone, PartialEq, Eq)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    struct VariableRaw(u32);
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
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct VariableData {
    /// Maximum voltage in mV
    pub max_voltage_mv: u16,
    /// Minimum voltage in mV
    pub min_voltage_mv: u16,
    /// Operational current in mA
    pub operational_current_ma: u16,
}

impl From<VariableRaw> for VariableData {
    fn from(raw: VariableRaw) -> Self {
        VariableData {
            max_voltage_mv: raw.max_voltage() * MV50_UNIT,
            min_voltage_mv: raw.min_voltage() * MV50_UNIT,
            operational_current_ma: raw.operational_current() * MA10_UNIT,
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
        raw.set_operational_current(data.operational_current_ma / MA10_UNIT);
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
    fn from(data: Apdo) -> u32 {
        match data {
            Apdo::SprPps(data) => data.into(),
            Apdo::EprAvs(data) => data.into(),
            Apdo::SprAvs(data) => data.into(),
        }
    }
}

impl Default for Apdo {
    fn default() -> Self {
        Apdo::SprPps(SprPpsData::default())
    }
}

bitfield! {
    /// Raw SPR Programable power supply data
    #[derive(Copy, Clone, PartialEq, Eq)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    struct SprPpsRaw(u32);
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
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SprPpsData {
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
        raw.set_max_voltage(data.max_voltage_mv / MV100_UNIT);
        raw.set_min_voltage(data.min_voltage_mv / MV100_UNIT);
        raw.set_max_current(data.max_current_ma / MA50_UNIT);
        raw.0
    }
}

bitfield! {
    /// Raw EPR Adjustable voltage supply data
    #[derive(Copy, Clone, PartialEq, Eq)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    struct EprAvsRaw(u32);
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
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct EprAvsData {
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
        raw.set_max_voltage(data.max_voltage_mv / MV100_UNIT);
        raw.set_min_voltage(data.min_voltage_mv / MV100_UNIT);
        raw.set_pdp(data.pdp_mw / MW1000_UNIT);
        raw.0
    }
}

bitfield! {
    /// Raw SPR adjustable voltage supply
    #[derive(Copy, Clone, PartialEq, Eq)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    struct SprAvsRaw(u32);
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
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SprAvsData {
    /// Maximum current for 9-15 V range in mA
    pub max_current_15v_ma: u16,
    /// Maximum current for 15-20 V range in mA
    pub max_current_20v_ma: u16,
}

impl From<SprAvsRaw> for SprAvsData {
    fn from(raw: SprAvsRaw) -> Self {
        SprAvsData {
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
        raw.set_max_current_15v(data.max_current_15v_ma / MA10_UNIT);
        raw.set_max_current_20v(data.max_current_20v_ma / MA10_UNIT);
        raw.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed_roundtrip() {
        const RAW_FIXED: u32 = 0x2A800805;
        let parsed = Pdo::try_from(RAW_FIXED).expect("Failed to parse fixed PDO");
        let expected = Pdo::Fixed(FixedData {
            dual_role_power: true,
            higher_capability: false,
            unconstrained_power: true,
            usb_comms_capable: false,
            dual_role_data: true,
            frs_required_current: FrsRequiredCurrent::Default,
            voltage_mv: 100,
            operational_current_ma: 50,
        });
        assert_eq!(parsed, expected);
        assert_eq!(u32::from(expected), RAW_FIXED);
    }

    #[test]
    fn test_battery_roundtrip() {
        const RAW_BATTERY: u32 = 0x40300801;
        let parsed = Pdo::try_from(RAW_BATTERY).expect("Failed to parse battery PDO");
        let expected = Pdo::Battery(BatteryData {
            max_voltage_mv: 150,
            min_voltage_mv: 100,
            operational_power_mw: 250,
        });
        assert_eq!(parsed, expected);
        assert_eq!(u32::from(expected), RAW_BATTERY);
    }

    #[test]
    fn test_variable_roundtrip() {
        const RAW_VARIABLE: u32 = 0x80300801;
        let parsed = Pdo::try_from(RAW_VARIABLE).expect("Failed to parse variable PDO");
        let expected = Pdo::Variable(VariableData {
            max_voltage_mv: 150,
            min_voltage_mv: 100,
            operational_current_ma: 10,
        });
        assert_eq!(parsed, expected);
        assert_eq!(u32::from(expected), RAW_VARIABLE);
    }

    #[test]
    fn test_apdo_spr_pps_roundtrip() {
        const RAW_SPR_PPS: u32 = 0xC0060201;
        let parsed = Pdo::try_from(RAW_SPR_PPS).expect("Failed to parse SPR PPS PDO");
        let expected = Pdo::Augmented(Apdo::SprPps(SprPpsData {
            max_voltage_mv: 300,
            min_voltage_mv: 200,
            max_current_ma: 50,
        }));
        assert_eq!(parsed, expected);
        assert_eq!(u32::from(expected), RAW_SPR_PPS);
    }

    #[test]
    fn test_apdo_epr_avs_roundtrip() {
        const RAW_EPR_AVS: u32 = 0xD0060201;
        let parsed = Pdo::try_from(RAW_EPR_AVS).expect("Failed to parse EPR AVS PDO");
        let expected = Pdo::Augmented(Apdo::EprAvs(EprAvsData {
            max_voltage_mv: 300,
            min_voltage_mv: 200,
            pdp_mw: 1000,
        }));
        assert_eq!(parsed, expected);
        assert_eq!(u32::from(expected), RAW_EPR_AVS);
    }

    #[test]
    fn test_apdo_spr_avs_roundtrip() {
        const RAW_SPR_AVS: u32 = 0xE0000401;
        let parsed = Pdo::try_from(RAW_SPR_AVS).expect("Failed to parse SPR AVS PDO");
        let expected = Pdo::Augmented(Apdo::SprAvs(SprAvsData {
            max_current_15v_ma: 10,
            max_current_20v_ma: 10,
        }));
        assert_eq!(parsed, expected);
        assert_eq!(u32::from(expected), RAW_SPR_AVS);
    }

    #[test]
    fn test_invalid_apdo_type() {
        const RAW_INVALID: u32 = 0xF0000401;
        let parsed = Pdo::try_from(RAW_INVALID);
        assert_eq!(
            parsed,
            Err(ExpectedPdo {
                kind: PdoKind::Augmented,
                apdo_kind: None,
                raw: RAW_INVALID,
            })
        );
    }
}

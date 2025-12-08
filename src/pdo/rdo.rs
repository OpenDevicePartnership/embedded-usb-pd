//! Request data object as defined in the USB PD specification 6.4.2
use bitfield::bitfield;

use super::{ApdoKind, Common, PdoKind};
use crate::pdo::*;

/// Request data object type
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rdo {
    /// Fixed
    Fixed(FixedVarData),
    /// Variable
    Variable(FixedVarData),
    /// Battery
    Battery(BatteryData),
    /// PPS
    Pps(PpsData),
    /// AVS
    Avs(AvsData),
}

impl Rdo {
    /// Create a new RDO from the raw data and the corresponding PDO
    pub fn for_pdo(rdo: u32, pdo: impl Common) -> Self {
        match pdo.kind() {
            PdoKind::Fixed => Rdo::Fixed(FixedVarRaw(rdo).into()),
            PdoKind::Variable => Rdo::Variable(FixedVarRaw(rdo).into()),
            PdoKind::Battery => Rdo::Battery(BatteryRaw(rdo).into()),
            PdoKind::Augmented => match pdo.apdo_kind().unwrap() {
                ApdoKind::SprPps => Rdo::Pps(PpsRaw(rdo).into()),
                ApdoKind::EprAvs => Rdo::Pps(PpsRaw(rdo).into()),
                ApdoKind::SprAvs => Rdo::Avs(AvsRaw(rdo).into()),
            },
        }
    }
}

impl From<Rdo> for u32 {
    fn from(rdo: Rdo) -> Self {
        match rdo {
            Rdo::Fixed(data) | Rdo::Variable(data) => u32::from(data),
            Rdo::Battery(data) => u32::from(data),
            Rdo::Pps(data) => u32::from(data),
            Rdo::Avs(data) => u32::from(data),
        }
    }
}

bitfield! {
    /// Fixed and variable RDO raw data
    #[derive(Copy, Clone, PartialEq, Eq)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    struct FixedVarRaw(u32);
    impl Debug;

    /// Object position
    pub u8, object_position, set_object_position: 31, 28;
    /// Capability mismatch
    pub bool, capability_mismatch, set_capability_mismatch: 26;
    /// USB communications capable
    pub bool, usb_comm_capable, set_usb_comm_capable: 25;
    /// No USB suspend
    pub bool, no_usb_suspend, set_no_usb_suspend: 24;
    /// Unchunked extended messages supported
    pub bool, unchunked_extended_messages_support, set_unchunked_extended_messages_support: 23;
    /// EPR capable
    pub bool, epr_capable, set_epr_capable: 22;
    /// Operating current in 10mA units
    pub u16, operating_current, set_operating_current: 19, 10;
    /// Max operating current in 10mA units
    pub u16, max_operating_current, set_max_operating_current: 9, 0;
}

/// Fixed and variable RDO data
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct FixedVarData {
    /// Object position
    pub object_position: u8,
    /// Capability mismatch
    pub capability_mismatch: bool,
    /// USB communications capable
    pub usb_comm_capable: bool,
    /// No USB suspend
    pub no_usb_suspend: bool,
    /// Unchunked extended messages supported
    pub unchunked_extended_messages_support: bool,
    /// EPR capable
    pub epr_capable: bool,
    /// Operating current in mA
    pub operating_current_ma: u16,
    /// Max operating current in mA
    pub max_operating_current_ma: u16,
}

impl From<FixedVarRaw> for FixedVarData {
    fn from(raw: FixedVarRaw) -> Self {
        FixedVarData {
            object_position: raw.object_position(),
            capability_mismatch: raw.capability_mismatch(),
            usb_comm_capable: raw.usb_comm_capable(),
            no_usb_suspend: raw.no_usb_suspend(),
            unchunked_extended_messages_support: raw.unchunked_extended_messages_support(),
            epr_capable: raw.epr_capable(),
            operating_current_ma: raw.operating_current() * MA10_UNIT,
            max_operating_current_ma: raw.max_operating_current() * MA10_UNIT,
        }
    }
}

impl From<FixedVarData> for u32 {
    fn from(data: FixedVarData) -> Self {
        let mut raw = FixedVarRaw(0);
        raw.set_object_position(data.object_position);
        raw.set_capability_mismatch(data.capability_mismatch);
        raw.set_usb_comm_capable(data.usb_comm_capable);
        raw.set_no_usb_suspend(data.no_usb_suspend);
        raw.set_unchunked_extended_messages_support(data.unchunked_extended_messages_support);
        raw.set_epr_capable(data.epr_capable);
        raw.set_operating_current(data.operating_current_ma / MA10_UNIT);
        raw.set_max_operating_current(data.max_operating_current_ma / MA10_UNIT);
        raw.0
    }
}

bitfield! {
    /// Battery RDO raw data
    #[derive(Copy, Clone, PartialEq, Eq)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    struct BatteryRaw(u32);
    impl Debug;

    /// Object position
    pub u8, object_position, set_object_position: 31, 28;
    /// Capability mismatch
    pub bool, capability_mismatch, set_capability_mismatch: 26;
    /// USB communications capable
    pub bool, usb_comm_capable, set_usb_comm_capable: 25;
    /// No USB suspend
    pub bool, no_usb_suspend, set_no_usb_suspend: 24;
    /// Unchunked extended messages supported
    pub bool, unchunked_extended_messages_support, set_unchunked_extended_messages_support: 23;
    /// EPR capable
    pub bool, epr_capable, set_epr_capable: 22;
    /// Operating power in 250mW units
    pub u32, operating_power, set_operating_power: 19, 10;
    /// Max operating power in 250mW units
    pub u32, max_operating_power, set_max_operating_power: 9, 0;
}

/// Battery RDO data
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct BatteryData {
    /// Object position
    pub object_position: u8,
    /// Capability mismatch
    pub capability_mismatch: bool,
    /// USB communications capable
    pub usb_comm_capable: bool,
    /// No USB suspend
    pub no_usb_suspend: bool,
    /// Unchunked extended messages supported
    pub unchunked_extended_messages_support: bool,
    /// EPR capable
    pub epr_capable: bool,
    /// Operating power in mW
    pub operating_power_mw: u32,
    /// Max operating power in mW
    pub max_operating_power_mw: u32,
}

impl From<BatteryRaw> for BatteryData {
    fn from(raw: BatteryRaw) -> Self {
        BatteryData {
            object_position: raw.object_position(),
            capability_mismatch: raw.capability_mismatch(),
            usb_comm_capable: raw.usb_comm_capable(),
            no_usb_suspend: raw.no_usb_suspend(),
            unchunked_extended_messages_support: raw.unchunked_extended_messages_support(),
            epr_capable: raw.epr_capable(),
            operating_power_mw: raw.operating_power() * MW250_UNIT,
            max_operating_power_mw: raw.max_operating_power() * MW250_UNIT,
        }
    }
}

impl From<BatteryData> for u32 {
    fn from(data: BatteryData) -> Self {
        let mut raw = BatteryRaw(0);
        raw.set_object_position(data.object_position);
        raw.set_capability_mismatch(data.capability_mismatch);
        raw.set_usb_comm_capable(data.usb_comm_capable);
        raw.set_no_usb_suspend(data.no_usb_suspend);
        raw.set_unchunked_extended_messages_support(data.unchunked_extended_messages_support);
        raw.set_epr_capable(data.epr_capable);
        raw.set_operating_power(data.operating_power_mw / MW250_UNIT);
        raw.set_max_operating_power(data.max_operating_power_mw / MW250_UNIT);
        raw.0
    }
}

bitfield! {
    /// PPS RDO raw data
    #[derive(Copy, Clone, PartialEq, Eq)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    struct PpsRaw(u32);
    impl Debug;

    /// Object position
    pub u8, object_position, set_object_position: 31, 28;
    /// Capability mismatch
    pub bool, capability_mismatch, set_capability_mismatch: 26;
    /// USB communications capable
    pub bool, usb_comm_capable, set_usb_comm_capable: 25;
    /// No USB suspend
    pub bool, no_usb_suspend, set_no_usb_suspend: 24;
    /// Unchunked extended messages supported
    pub bool, unchunked_extended_messages_support, set_unchunked_extended_messages_support: 23;
    /// EPR capable
    pub bool, epr_capable, set_epr_capable: 22;
    /// Output voltage in 20mV units
    pub u16, output_voltage, set_output_voltage: 20, 9;
    /// Operating current in 50mA units
    pub u16, operating_current, set_operating_current: 6, 0;
}

/// PPS RDO data
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct PpsData {
    /// Object position
    pub object_position: u8,
    /// Capability mismatch
    pub capability_mismatch: bool,
    /// USB communications capable
    pub usb_comm_capable: bool,
    /// No USB suspend
    pub no_usb_suspend: bool,
    /// Unchunked extended messages supported
    pub unchunked_extended_messages_support: bool,
    /// EPR capable
    pub epr_capable: bool,
    /// Output voltage in mV
    pub output_voltage_mv: u16,
    /// Operating current in mA
    pub operating_current_ma: u16,
}

impl From<PpsRaw> for PpsData {
    fn from(raw: PpsRaw) -> Self {
        PpsData {
            object_position: raw.object_position(),
            capability_mismatch: raw.capability_mismatch(),
            usb_comm_capable: raw.usb_comm_capable(),
            no_usb_suspend: raw.no_usb_suspend(),
            unchunked_extended_messages_support: raw.unchunked_extended_messages_support(),
            epr_capable: raw.epr_capable(),
            output_voltage_mv: raw.output_voltage() * MV20_UNIT,
            operating_current_ma: raw.operating_current() * MA50_UNIT,
        }
    }
}

impl From<PpsData> for u32 {
    fn from(data: PpsData) -> Self {
        let mut raw = PpsRaw(0);
        raw.set_object_position(data.object_position);
        raw.set_capability_mismatch(data.capability_mismatch);
        raw.set_usb_comm_capable(data.usb_comm_capable);
        raw.set_no_usb_suspend(data.no_usb_suspend);
        raw.set_unchunked_extended_messages_support(data.unchunked_extended_messages_support);
        raw.set_epr_capable(data.epr_capable);
        raw.set_output_voltage(data.output_voltage_mv / MV20_UNIT);
        raw.set_operating_current(data.operating_current_ma / MA50_UNIT);
        raw.0
    }
}

bitfield! {
    /// AVS RDO raw data
    #[derive(Copy, Clone, PartialEq, Eq)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    struct AvsRaw(u32);
    impl Debug;

    /// Object position
    pub u8, object_position, set_object_position: 31, 28;
    /// Capability mismatch
    pub bool, capability_mismatch, set_capability_mismatch: 26;
    /// USB communications capable
    pub bool, usb_comm_capable, set_usb_comm_capable: 25;
    /// No USB suspend
    pub bool, no_usb_suspend, set_no_usb_suspend: 24;
    /// Unchunked extended messages supported
    pub bool, unchunked_extended_messages_support, set_unchunked_extended_messages_support: 23;
    /// EPR capable
    pub bool, epr_capable, set_epr_capable: 22;
    /// Output voltage in 20mV units
    pub u16, output_voltage, set_output_voltage: 20, 9;
    /// Operating current in 50mA units
    pub u16, operating_current, set_operating_current: 6, 0;
}

/// AVS RDO data
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct AvsData {
    /// Object position
    pub object_position: u8,
    /// Capability mismatch
    pub capability_mismatch: bool,
    /// USB communications capable
    pub usb_comm_capable: bool,
    /// No USB suspend
    pub no_usb_suspend: bool,
    /// Unchunked extended messages supported
    pub unchunked_extended_messages_support: bool,
    /// EPR capable
    pub epr_capable: bool,
    /// Output voltage in mV
    pub output_voltage_mv: u16,
    /// Operating current in mA
    pub operating_current_ma: u16,
}

impl From<AvsRaw> for AvsData {
    fn from(raw: AvsRaw) -> Self {
        AvsData {
            object_position: raw.object_position(),
            capability_mismatch: raw.capability_mismatch(),
            usb_comm_capable: raw.usb_comm_capable(),
            no_usb_suspend: raw.no_usb_suspend(),
            unchunked_extended_messages_support: raw.unchunked_extended_messages_support(),
            epr_capable: raw.epr_capable(),
            output_voltage_mv: raw.output_voltage() * MV20_UNIT,
            operating_current_ma: raw.operating_current() * MA50_UNIT,
        }
    }
}

impl From<AvsData> for u32 {
    fn from(data: AvsData) -> Self {
        let mut raw = AvsRaw(0);
        raw.set_object_position(data.object_position);
        raw.set_capability_mismatch(data.capability_mismatch);
        raw.set_usb_comm_capable(data.usb_comm_capable);
        raw.set_no_usb_suspend(data.no_usb_suspend);
        raw.set_unchunked_extended_messages_support(data.unchunked_extended_messages_support);
        raw.set_epr_capable(data.epr_capable);
        raw.set_output_voltage(data.output_voltage_mv / MV20_UNIT);
        raw.set_operating_current(data.operating_current_ma / MA50_UNIT);
        raw.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed_roundtrip() {
        const RAW_FIXED: u32 = 0x3540C864;
        let rdo = Rdo::for_pdo(
            RAW_FIXED,
            // These values don't matter, only the kind is used
            sink::Pdo::Fixed(sink::FixedData {
                dual_role_power: false,
                higher_capability: false,
                unconstrained_power: false,
                usb_comms_capable: false,
                dual_role_data: false,
                frs_required_current: sink::FrsRequiredCurrent::None,
                voltage_mv: 0,
                operational_current_ma: 0,
            }),
        );
        let expected = Rdo::Fixed(FixedVarData {
            object_position: 3,
            capability_mismatch: true,
            usb_comm_capable: false,
            no_usb_suspend: true,
            unchunked_extended_messages_support: false,
            epr_capable: true,
            operating_current_ma: 500,
            max_operating_current_ma: 1000,
        });
        assert_eq!(rdo, expected);
        assert_eq!(u32::from(expected), RAW_FIXED);
    }

    #[test]
    fn test_var_roundtrip() {
        const RAW_VARIABLE: u32 = 0x3540C864;
        let rdo = Rdo::for_pdo(
            RAW_VARIABLE,
            // These values don't matter, only the kind is used
            sink::Pdo::Variable(sink::VariableData {
                max_voltage_mv: 0,
                min_voltage_mv: 0,
                operational_current_ma: 0,
            }),
        );
        let expected = Rdo::Variable(FixedVarData {
            object_position: 3,
            capability_mismatch: true,
            usb_comm_capable: false,
            no_usb_suspend: true,
            unchunked_extended_messages_support: false,
            epr_capable: true,
            operating_current_ma: 500,
            max_operating_current_ma: 1000,
        });
        assert_eq!(rdo, expected);
        assert_eq!(u32::from(expected), RAW_VARIABLE);
    }

    #[test]
    fn test_battery_roundtrip() {
        const RAW_BATTERY: u32 = 0x35400402;
        let rdo = Rdo::for_pdo(
            RAW_BATTERY,
            // These values don't matter, only the kind is used
            sink::Pdo::Battery(sink::BatteryData {
                max_voltage_mv: 0,
                min_voltage_mv: 0,
                operational_power_mw: 0,
            }),
        );
        let expected = Rdo::Battery(BatteryData {
            object_position: 3,
            capability_mismatch: true,
            usb_comm_capable: false,
            no_usb_suspend: true,
            unchunked_extended_messages_support: false,
            epr_capable: true,
            operating_power_mw: 250,
            max_operating_power_mw: 500,
        });
        assert_eq!(rdo, expected);
        assert_eq!(u32::from(expected), RAW_BATTERY);
    }

    #[test]
    fn test_pps_roundtrip() {
        const RAW_PPS: u32 = 0x35400202;
        let rdo = Rdo::for_pdo(
            RAW_PPS,
            // These values don't matter, only the kind is used
            sink::Pdo::Augmented(sink::Apdo::SprPps(sink::SprPpsData {
                max_voltage_mv: 0,
                min_voltage_mv: 0,
                max_current_ma: 0,
            })),
        );
        let expected = Rdo::Pps(PpsData {
            object_position: 3,
            capability_mismatch: true,
            usb_comm_capable: false,
            no_usb_suspend: true,
            unchunked_extended_messages_support: false,
            epr_capable: true,
            output_voltage_mv: 20,
            operating_current_ma: 100,
        });
        assert_eq!(rdo, expected);
        assert_eq!(u32::from(expected), RAW_PPS);
    }

    #[test]
    fn test_avs_roundtrip() {
        const RAW_AVS: u32 = 0x35400202;
        let rdo = Rdo::for_pdo(
            RAW_AVS,
            // These values don't matter, only the kind is used
            sink::Pdo::Augmented(sink::Apdo::SprAvs(sink::SprAvsData {
                max_current_15v_ma: 0,
                max_current_20v_ma: 0,
            })),
        );
        let expected = Rdo::Avs(AvsData {
            object_position: 3,
            capability_mismatch: true,
            usb_comm_capable: false,
            no_usb_suspend: true,
            unchunked_extended_messages_support: false,
            epr_capable: true,
            output_voltage_mv: 20,
            operating_current_ma: 100,
        });
        assert_eq!(rdo, expected);
        assert_eq!(u32::from(expected), RAW_AVS);
    }
}

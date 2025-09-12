//! Alert data object as defined in the USB PD specification 6.4.6
use bitfield::{bitfield, Bit};

use crate::PdError;

/// Error type for ADO conversion, contains the complete undecoded ADO
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct InvalidType(pub u32);

impl From<InvalidType> for PdError {
    fn from(_: InvalidType) -> Self {
        PdError::InvalidParams
    }
}

bitfield! {
    /// Battery status change flags
    #[derive(Copy, Clone, PartialEq, Eq)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub struct BatteryStatusChangeRaw(u8);
    impl Debug;

    /// Fixed battery status change
    pub u8, fixed_battery_status_change, set_fixed_battery_status_change: 7, 4;
    /// Hot swappable battery status
    pub u8, hot_swappable_battery_status, set_hot_swappable_battery_status: 3, 0;
}

/// Battery status change event
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct BatteryStatusChange(BatteryStatusChangeRaw);

/// Maximum battery alert bit index
pub const MAX_BATTERY_INDEX: usize = 3;

impl BatteryStatusChange {
    /// Returns the fixed battery status change at the given index
    pub fn fixed_battery_status_change(&self, index: usize) -> Result<bool, PdError> {
        if index > MAX_BATTERY_INDEX {
            return Err(PdError::InvalidParams);
        }
        Ok(self.0.fixed_battery_status_change().bit(index))
    }

    /// Returns the hot swappable battery status change at the given index
    pub fn hot_swappable_battery_status(&self, index: usize) -> Result<bool, PdError> {
        if index > MAX_BATTERY_INDEX {
            return Err(PdError::InvalidParams);
        }
        Ok(self.0.hot_swappable_battery_status().bit(index))
    }
}

bitfield! {
    /// Raw ADO type
    #[derive(Copy, Clone, PartialEq, Eq)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub struct AdoRaw(u32);
    impl Debug;

    /// Type of alert
    pub u8, alert_type, set_alert_type: 31, 24;
    /// Fixed battery status change
    pub u8, battery_status_change, set_battery_status_change: 23, 16;
    /// Extended alert type
    pub u8, extended_alert_type, set_extended_alert_type: 3, 0;
}

/// ADO types
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ado {
    /// Battery status change event
    BatteryStatusChange(BatteryStatusChange),
    /// Over-current event,
    Ocp,
    /// Over-temperature event
    Otp,
    /// Operating condition change event
    OperatingConditionChange,
    /// Source input change event
    SourceInputChange,
    /// Over-voltage event
    Ovp,
    /// Power state change
    PowerStateChange,
    /// Power button press
    PowerButtonPress,
    /// Power button release
    PowerButtonRelease,
    /// Controller initiated wake
    ControllerInitiatedWake,
}

impl TryFrom<AdoRaw> for Ado {
    type Error = InvalidType;

    fn try_from(raw: AdoRaw) -> Result<Self, Self::Error> {
        match raw.alert_type() {
            // Standard alert types
            0x02 => Ok(Ado::BatteryStatusChange(BatteryStatusChange(BatteryStatusChangeRaw(
                raw.battery_status_change(),
            )))),
            0x04 => Ok(Ado::Ocp),
            0x08 => Ok(Ado::Otp),
            0x10 => Ok(Ado::OperatingConditionChange),
            0x20 => Ok(Ado::SourceInputChange),
            0x40 => Ok(Ado::Ovp),
            // Extended alert types
            0x80 => match raw.extended_alert_type() {
                0x01 => Ok(Ado::PowerStateChange),
                0x02 => Ok(Ado::PowerButtonPress),
                0x03 => Ok(Ado::PowerButtonRelease),
                0x04 => Ok(Ado::ControllerInitiatedWake),
                _ => Err(InvalidType(raw.0)),
            },
            _ => Err(InvalidType(raw.0)),
        }
    }
}

impl TryFrom<u32> for Ado {
    type Error = InvalidType;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        AdoRaw(value).try_into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_battery_status_change_fixed() {
        let bsc = BatteryStatusChange(BatteryStatusChangeRaw(0b1011_0000));

        // Only bits 0-3 are valid
        assert_eq!(bsc.fixed_battery_status_change(0).unwrap(), true);
        assert_eq!(bsc.fixed_battery_status_change(1).unwrap(), true);
        assert_eq!(bsc.fixed_battery_status_change(2).unwrap(), false);
        assert_eq!(bsc.fixed_battery_status_change(3).unwrap(), true);
        // Out of range
        assert_eq!(bsc.fixed_battery_status_change(4), Err(PdError::InvalidParams));
    }

    #[test]
    fn test_battery_status_change_hot_swappable() {
        let bsc = BatteryStatusChange(BatteryStatusChangeRaw(0b1011));

        assert_eq!(bsc.hot_swappable_battery_status(0).unwrap(), true);
        assert_eq!(bsc.hot_swappable_battery_status(1).unwrap(), true);
        assert_eq!(bsc.hot_swappable_battery_status(2).unwrap(), false);
        assert_eq!(bsc.hot_swappable_battery_status(3).unwrap(), true);

        // Out of range
        assert_eq!(bsc.hot_swappable_battery_status(4), Err(PdError::InvalidParams));
    }

    #[test]
    fn test_ado_try_from_standard_alerts() {
        let mut raw = AdoRaw(0);

        // BatteryStatusChange
        raw.set_alert_type(0x02);
        raw.set_battery_status_change(0b0010_0001);
        let ado = Ado::try_from(raw).unwrap();
        match ado {
            Ado::BatteryStatusChange(bsc) => {
                assert_eq!(bsc.fixed_battery_status_change(1).unwrap(), true);
                assert_eq!(bsc.hot_swappable_battery_status(0).unwrap(), true);
            }
            _ => panic!("Expected BatteryStatusChange"),
        }

        // Ocp
        raw.set_alert_type(0x04);
        assert_eq!(Ado::try_from(raw).unwrap(), Ado::Ocp);

        // Otp
        raw.set_alert_type(0x08);
        assert_eq!(Ado::try_from(raw).unwrap(), Ado::Otp);

        // OperatingConditionChange
        raw.set_alert_type(0x10);
        assert_eq!(Ado::try_from(raw).unwrap(), Ado::OperatingConditionChange);

        // SourceInputChange
        raw.set_alert_type(0x20);
        assert_eq!(Ado::try_from(raw).unwrap(), Ado::SourceInputChange);

        // Ovp
        raw.set_alert_type(0x40);
        assert_eq!(Ado::try_from(raw).unwrap(), Ado::Ovp);

        // Extended alert types
        raw.set_alert_type(0x80);

        raw.set_extended_alert_type(0x01);
        assert_eq!(Ado::try_from(raw).unwrap(), Ado::PowerStateChange);

        raw.set_extended_alert_type(0x02);
        assert_eq!(Ado::try_from(raw).unwrap(), Ado::PowerButtonPress);

        raw.set_extended_alert_type(0x03);
        assert_eq!(Ado::try_from(raw).unwrap(), Ado::PowerButtonRelease);

        raw.set_extended_alert_type(0x04);
        assert_eq!(Ado::try_from(raw).unwrap(), Ado::ControllerInitiatedWake);

        raw.set_extended_alert_type(0x05);
        assert_eq!(Ado::try_from(raw), Err(InvalidType(raw.0)));
    }

    #[test]
    fn test_ado_try_from_invalid_alert_type() {
        let mut raw = AdoRaw(0);
        raw.set_alert_type(0xFF);
        assert!(Ado::try_from(raw).is_err());
    }
}

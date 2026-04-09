//! A Passive Cable has a USB Plug on each end, at least one of which is a Cable
//! Plug supporting SOP' Communication.
//!
//! See PD spec 6.4.4.3.1.6 Passive Cable VDO.

use crate::vdm::structured::command::discover_identity::ProductTypeVdo;

/// A Passive Cable has a USB Plug on each end, at least one of which is a Cable
/// Plug supporting SOP' Communication.
///
/// Sent based on the value of [`sop_prime::IdHeaderVdo::product_type`][super::sop_prime::IdHeaderVdo::product_type].
///
/// See PD spec 6.4.4.3.1.6 Passive Cable VDO, table 6.41 Passive Cable VDO.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct PassiveCableVdo {
    /// The highest rate the cable supports.
    pub usb_highest_speed: UsbHighestSpeed,

    /// Indicates whether the cable can carry 3A or 5A.
    pub vbus_current_handling_capability: VbusCurrentHandlingCapability,

    /// The maximum voltage that shall be negotiated using a Fixed Supply over the
    /// cable as part of an Explicit Contract.
    pub maximum_vbus_voltage: MaximumVbusVoltage,

    /// Whether the cable needs `VCONN` only initially in order to support the Discover
    /// Identity Command, after which it can be removed, or if it needs `VCONN`
    /// to be continuously applied to power some feature of the Cable Plug.
    pub cable_termination_type: CableTerminationType,

    /// The signal latency through the cable, which can be used as an approximation
    /// for its length.
    pub cable_latency: CableLatency,

    /// Whether or not the cable is designed for safe operation when carrying up
    /// to 48 volts at 5 amps.
    pub epr_capable: bool,

    /// Whether or not the opposite end from the USB Type-C plug is another USB
    /// Type-C plug or a Captive Cable Assembly.
    pub usb_type_c_or_captive: UsbTypeCPlugOrCaptive,

    /// The FW version assigned by the VID owner.
    pub firmware_version: u8,

    /// The HW version assigned by the VID owner.
    pub hw_version: u8,
}

impl TryFrom<Raw> for PassiveCableVdo {
    type Error = ParsePassiveCableVdoError;

    fn try_from(raw: Raw) -> Result<Self, Self::Error> {
        Ok(Self {
            usb_highest_speed: raw
                .usb_highest_speed()
                .try_into()
                .map_err(|()| ParsePassiveCableVdoError::InvalidUsbHighestSpeed)?,
            vbus_current_handling_capability: raw
                .vbus_current_handling_capability()
                .try_into()
                .map_err(|()| ParsePassiveCableVdoError::InvalidVbusCurrentHandlingCapability)?,
            maximum_vbus_voltage: raw
                .maximum_vbus_voltage()
                .try_into()
                .map_err(|()| ParsePassiveCableVdoError::InvalidMaximumVbusVoltage)?,
            cable_termination_type: raw
                .cable_termination_type()
                .try_into()
                .map_err(|()| ParsePassiveCableVdoError::InvalidCableTerminationType)?,
            cable_latency: raw
                .cable_latency()
                .try_into()
                .map_err(|()| ParsePassiveCableVdoError::InvalidCableLatency)?,
            epr_capable: raw.epr_capable(),
            usb_type_c_or_captive: raw
                .usb_type_c_or_captive()
                .try_into()
                .map_err(|()| ParsePassiveCableVdoError::InvalidUsbTypeCOrCaptive)?,
            firmware_version: raw.firmware_version(),
            hw_version: raw.hw_version(),
        })
    }
}

impl TryFrom<u32> for PassiveCableVdo {
    type Error = ParsePassiveCableVdoError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Raw(value).try_into()
    }
}

impl TryFrom<ProductTypeVdo> for PassiveCableVdo {
    type Error = ParsePassiveCableVdoError;

    fn try_from(value: ProductTypeVdo) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl TryFrom<[u8; 4]> for PassiveCableVdo {
    type Error = ParsePassiveCableVdoError;

    fn try_from(bytes: [u8; 4]) -> Result<Self, Self::Error> {
        u32::from_le_bytes(bytes).try_into()
    }
}

/// Errors that can occur when parsing a [`PassiveCableVdo`] from its raw value.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ParsePassiveCableVdoError {
    /// [`PassiveCableVdo::usb_highest_speed`] contains an invalid value.
    InvalidUsbHighestSpeed,

    /// [`PassiveCableVdo::vbus_current_handling_capability`] contains an invalid value.
    InvalidVbusCurrentHandlingCapability,

    /// [`PassiveCableVdo::maximum_vbus_voltage`] contains an invalid value.
    InvalidMaximumVbusVoltage,

    /// [`PassiveCableVdo::cable_termination_type`] contains an invalid value.
    InvalidCableTerminationType,

    /// [`PassiveCableVdo::cable_latency`] contains an invalid value.
    InvalidCableLatency,

    /// [`PassiveCableVdo::usb_type_c_or_captive`] contains an invalid value.
    InvalidUsbTypeCOrCaptive,
}

bitfield::bitfield! {
    /// The raw value of a [`PassiveCableVdo`], before parsing enumerations and bitfields.
    #[derive(Copy, Clone, PartialEq, Eq)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub struct Raw(u32);
    impl Debug;

    /// See [`PassiveCableVdo::usb_highest_speed`].
    pub u8, usb_highest_speed, set_usb_highest_speed: 2, 0;

    /// See [`PassiveCableVdo::vbus_current_handling_capability`].
    pub u8, vbus_current_handling_capability, set_vbus_current_handling_capability: 6, 5;

    /// See [`PassiveCableVdo::maximum_vbus_voltage`].
    pub u8, maximum_vbus_voltage, set_maximum_vbus_voltage: 10, 9;

    /// See [`PassiveCableVdo::cable_termination_type`].
    pub u8, cable_termination_type, set_cable_termination_type: 12, 11;

    /// See [`PassiveCableVdo::cable_latency`].
    pub u8, cable_latency, set_cable_latency: 16, 13;

    /// See [`PassiveCableVdo::epr_capable`].
    pub bool, epr_capable, set_epr_capable: 17;

    /// See [`PassiveCableVdo::usb_type_c_or_captive`].
    pub u8, usb_type_c_or_captive, set_usb_type_c_or_captive: 19, 18;

    /// See [`PassiveCableVdo::firmware_version`].
    pub u8, firmware_version, set_firmware_version: 27, 24;

    /// See [`PassiveCableVdo::hw_version`].
    pub u8, hw_version, set_hw_version: 31, 28;
}

/// The highest rate the cable supports.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UsbHighestSpeed {
    /// USB 2.0 only, no SuperSpeed support.
    Usb2p0,

    /// USB 3.2 Gen1.
    Usb3p2Gen1,

    /// USB 3.2 and USB4 Gen2.
    Usb3p2,

    /// USB4 Gen3.
    Usb4Gen3,

    /// USB4 Gen4.
    Usb4Gen4,
}

impl TryFrom<u8> for UsbHighestSpeed {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0b000 => Ok(Self::Usb2p0),
            0b001 => Ok(Self::Usb3p2Gen1),
            0b010 => Ok(Self::Usb3p2),
            0b011 => Ok(Self::Usb4Gen3),
            0b100 => Ok(Self::Usb4Gen4),
            _ => Err(()),
        }
    }
}

/// Indicates whether the cable can carry 3A or 5A.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VbusCurrentHandlingCapability {
    /// 3A
    ThreeAmps,

    /// 5A
    FiveAmps,
}

impl TryFrom<u8> for VbusCurrentHandlingCapability {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0b01 => Ok(Self::ThreeAmps),
            0b10 => Ok(Self::FiveAmps),
            _ => Err(()),
        }
    }
}

/// The maximum voltage that shall be negotiated using a Fixed Supply over the
/// cable as part of an Explicit Contract.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MaximumVbusVoltage {
    /// 20V
    TwentyVolt,

    /// 30V
    ThirtyVolt,

    /// 40V
    FortyVolt,

    /// 50V
    FiftyVolt,
}

impl TryFrom<u8> for MaximumVbusVoltage {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0b00 => Ok(Self::TwentyVolt),
            0b01 => Ok(Self::ThirtyVolt),
            0b10 => Ok(Self::FortyVolt),
            0b11 => Ok(Self::FiftyVolt),
            _ => Err(()),
        }
    }
}

/// Whether the cable needs `VCONN` only initially in order to support the Discover
/// Identity Command, after which it can be removed, or if it needs `VCONN`
/// to be continuously applied to power some feature of the Cable Plug.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CableTerminationType {
    /// The cable only requires `VCONN` to support the Discover Identity Command,
    /// after which it can be removed.
    VconnNotRequired,

    /// The cable requires `VCONN` to be continuously applied to power some feature
    /// of the Cable Plug, in addition to needing `VCONN` to support the Discover
    /// Identity Command.
    VconnRequired,
}

impl TryFrom<u8> for CableTerminationType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0b00 => Ok(Self::VconnNotRequired),
            0b01 => Ok(Self::VconnRequired),
            _ => Err(()),
        }
    }
}

/// The signal latency through the cable, which can be used as an approximation
/// for its length.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CableLatency {
    /// <10ns (~1m)
    LessThan10ns,

    /// 10ns to 20ns (~2m)
    LessThan20ns,

    /// 20ns to 30ns (~3m)
    LessThan30ns,

    /// 30ns to 40ns (~4m)
    LessThan40ns,

    /// 40ns to 50ns (~5m)
    LessThan50ns,

    /// 50ns to 60ns (~6m)
    LessThan60ns,

    /// 60ns to 70ns (~7m)
    LessThan70ns,

    /// >70ns (>~7m)
    GreaterThan70ns,
}

impl TryFrom<u8> for CableLatency {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0b0001 => Ok(Self::LessThan10ns),
            0b0010 => Ok(Self::LessThan20ns),
            0b0011 => Ok(Self::LessThan30ns),
            0b0100 => Ok(Self::LessThan40ns),
            0b0101 => Ok(Self::LessThan50ns),
            0b0110 => Ok(Self::LessThan60ns),
            0b0111 => Ok(Self::LessThan70ns),
            0b1000 => Ok(Self::GreaterThan70ns),
            _ => Err(()),
        }
    }
}

/// Whether or not the opposite end from the USB Type-C plug is another USB
/// Type-C plug or a Captive Cable Assembly.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UsbTypeCPlugOrCaptive {
    /// Opposite end from the plug is another USB Type-C plug.
    UsbTypeC,

    /// Opposite end from the plug is a Captive Cable Assembly.
    Captive,
}

impl TryFrom<u8> for UsbTypeCPlugOrCaptive {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0b10 => Ok(Self::UsbTypeC),
            0b11 => Ok(Self::Captive),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod usb_highest_speed {
        use super::*;

        #[test]
        fn all_valid_variants() {
            let cases: [(u8, UsbHighestSpeed); 5] = [
                (0b000, UsbHighestSpeed::Usb2p0),
                (0b001, UsbHighestSpeed::Usb3p2Gen1),
                (0b010, UsbHighestSpeed::Usb3p2),
                (0b011, UsbHighestSpeed::Usb4Gen3),
                (0b100, UsbHighestSpeed::Usb4Gen4),
            ];
            for (raw, expected) in cases {
                assert_eq!(UsbHighestSpeed::try_from(raw), Ok(expected), "raw={raw}");
            }
        }

        #[test]
        fn invalid_values() {
            for v in 5..=255u8 {
                assert!(UsbHighestSpeed::try_from(v).is_err(), "raw={v} should be invalid");
            }
        }
    }

    mod vbus_current_handling_capability {
        use super::*;

        #[test]
        fn all_valid_variants() {
            let cases: [(u8, VbusCurrentHandlingCapability); 2] = [
                (0b01, VbusCurrentHandlingCapability::ThreeAmps),
                (0b10, VbusCurrentHandlingCapability::FiveAmps),
            ];
            for (raw, expected) in cases {
                assert_eq!(VbusCurrentHandlingCapability::try_from(raw), Ok(expected), "raw={raw}");
            }
        }

        #[test]
        fn invalid_values() {
            for v in [0u8, 3] {
                assert!(
                    VbusCurrentHandlingCapability::try_from(v).is_err(),
                    "raw={v} should be invalid"
                );
            }
        }
    }

    mod maximum_vbus_voltage {
        use super::*;

        #[test]
        fn all_valid_variants() {
            let cases: [(u8, MaximumVbusVoltage); 4] = [
                (0b00, MaximumVbusVoltage::TwentyVolt),
                (0b01, MaximumVbusVoltage::ThirtyVolt),
                (0b10, MaximumVbusVoltage::FortyVolt),
                (0b11, MaximumVbusVoltage::FiftyVolt),
            ];
            for (raw, expected) in cases {
                assert_eq!(MaximumVbusVoltage::try_from(raw), Ok(expected), "raw={raw}");
            }
        }

        #[test]
        fn invalid_values() {
            for v in 4..=255u8 {
                assert!(MaximumVbusVoltage::try_from(v).is_err(), "raw={v} should be invalid");
            }
        }
    }

    mod cable_termination_type {
        use super::*;

        #[test]
        fn all_valid_variants() {
            let cases: [(u8, CableTerminationType); 2] = [
                (0b00, CableTerminationType::VconnNotRequired),
                (0b01, CableTerminationType::VconnRequired),
            ];
            for (raw, expected) in cases {
                assert_eq!(CableTerminationType::try_from(raw), Ok(expected), "raw={raw}");
            }
        }

        #[test]
        fn invalid_values() {
            for v in 2..=255u8 {
                assert!(CableTerminationType::try_from(v).is_err(), "raw={v} should be invalid");
            }
        }
    }

    mod cable_latency {
        use super::*;

        #[test]
        fn all_valid_variants() {
            let cases: [(u8, CableLatency); 8] = [
                (0b0001, CableLatency::LessThan10ns),
                (0b0010, CableLatency::LessThan20ns),
                (0b0011, CableLatency::LessThan30ns),
                (0b0100, CableLatency::LessThan40ns),
                (0b0101, CableLatency::LessThan50ns),
                (0b0110, CableLatency::LessThan60ns),
                (0b0111, CableLatency::LessThan70ns),
                (0b1000, CableLatency::GreaterThan70ns),
            ];
            for (raw, expected) in cases {
                assert_eq!(CableLatency::try_from(raw), Ok(expected), "raw={raw}");
            }
        }

        #[test]
        fn invalid_values() {
            assert!(CableLatency::try_from(0u8).is_err());
            for v in 9..=255u8 {
                assert!(CableLatency::try_from(v).is_err(), "raw={v} should be invalid");
            }
        }
    }

    mod usb_type_c_plug_or_captive {
        use super::*;

        #[test]
        fn all_valid_variants() {
            let cases: [(u8, UsbTypeCPlugOrCaptive); 2] = [
                (0b10, UsbTypeCPlugOrCaptive::UsbTypeC),
                (0b11, UsbTypeCPlugOrCaptive::Captive),
            ];
            for (raw, expected) in cases {
                assert_eq!(UsbTypeCPlugOrCaptive::try_from(raw), Ok(expected), "raw={raw}");
            }
        }

        #[test]
        fn invalid_values() {
            for v in [0u8, 1] {
                assert!(UsbTypeCPlugOrCaptive::try_from(v).is_err(), "raw={v} should be invalid");
            }
        }
    }
}

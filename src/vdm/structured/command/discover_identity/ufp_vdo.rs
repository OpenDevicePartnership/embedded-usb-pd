//! A UFP (Upward Facing Port) is a Port that consumes power and/or data from a
//! DFP. UFPs include traditional USB peripherals, USB Hub's upstream Port, and
//! DRD-capable host Ports.
//!
//! See PD spec 6.4.4.3.1.4 UFP VDO.

use crate::vdm::structured::command::discover_identity::ProductTypeVdo;

/// Returned by Ports capable of operating as a UFP, including traditional USB peripherals,
/// USB Hub's upstream Port, and DRD-capable host Ports.
///
/// Sent based on the value of [`sop::IdHeaderVdo::product_type_ufp`][super::sop::IdHeaderVdo::product_type_ufp].
///
/// See PD spec 6.4.4.3.1.4 UFP VDO.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct UfpVdo {
    /// The port's highest speed capability.
    pub usb_highest_speed: UsbHighestSpeed,

    /// All types of Alternate Modes, if any, the device supports.
    pub alternate_modes: AlternateModes,
    pub vbus_required: bool,
    pub vconn_required: bool,

    /// Whether `VCONN` is needed for the AMA to operate.
    pub vconn_power: VconnPower,

    /// The UFP's Capabilities when operating as either a PDUSB Device or Hub.
    pub device_capability: DeviceCapability,
}

bitfield::bitfield! {
    /// The raw value of a [`UfpVdo`], before parsing enumerations and bitfields.
    #[derive(Copy, Clone, PartialEq, Eq)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub struct Raw(u32);
    impl Debug;

    /// See [`UfpVdo::usb_highest_speed`].
    pub u8, usb_highest_speed, set_usb_highest_speed: 2, 0;

    /// See [`UfpVdo::alternate_modes`].
    pub u8, alternate_modes, set_alternate_modes: 5, 3;

    /// See [`UfpVdo::vbus_required`].
    pub bool, vbus_required_n, set_vbus_required_n: 6;

    /// See [`UfpVdo::vconn_required`].
    pub bool, vconn_required, set_vconn_required: 7;

    /// See [`UfpVdo::vconn_power`].
    pub u8, vconn_power, set_vconn_power: 10, 8;

    /// See [`UfpVdo::device_capability`].
    pub u8, device_capability, set_device_capability: 27, 24;
}

/// Errors that can occur when parsing a [`UfpVdo`] from its raw value.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ParseUfpVdoError {
    /// [`UfpVdo::usb_highest_speed`] contains an invalid value.
    InvalidUsbHighestSpeed,

    /// [`UfpVdo::vconn_power`] contains an invalid value.
    InvalidVconnPower,

    /// [`UfpVdo::device_capability`] contains an invalid value.
    InvalidDeviceCapability,
}

impl TryFrom<Raw> for UfpVdo {
    type Error = ParseUfpVdoError;

    fn try_from(raw: Raw) -> Result<Self, Self::Error> {
        Ok(Self {
            usb_highest_speed: raw
                .usb_highest_speed()
                .try_into()
                .map_err(|()| ParseUfpVdoError::InvalidUsbHighestSpeed)?,
            alternate_modes: raw.alternate_modes().into(),
            vbus_required: !raw.vbus_required_n(),
            vconn_required: raw.vconn_required(),
            vconn_power: raw
                .vconn_power()
                .try_into()
                .map_err(|()| ParseUfpVdoError::InvalidVconnPower)?,
            device_capability: raw.device_capability().into(),
        })
    }
}

impl TryFrom<u32> for UfpVdo {
    type Error = ParseUfpVdoError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Raw(value).try_into()
    }
}

impl TryFrom<ProductTypeVdo> for UfpVdo {
    type Error = ParseUfpVdoError;

    fn try_from(value: ProductTypeVdo) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl TryFrom<[u8; 4]> for UfpVdo {
    type Error = ParseUfpVdoError;

    fn try_from(bytes: [u8; 4]) -> Result<Self, Self::Error> {
        u32::from_le_bytes(bytes).try_into()
    }
}

/// The port's highest speed capability.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UsbHighestSpeed {
    /// USB 2.0 only, no SuperSpeed support.
    Usb2p0,

    /// USB 3.2 Gen1.
    Usb3p2Gen1,

    /// USB 3.2 and USB4 Gen2.
    Usb3p2Gen2,

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
            0b010 => Ok(Self::Usb3p2Gen2),
            0b011 => Ok(Self::Usb4Gen3),
            0b100 => Ok(Self::Usb4Gen4),
            _ => Err(()),
        }
    }
}

/// All types of Alternate Modes, if any, the device supports.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct AlternateModes {
    /// Supports Thunderbolt 3 Alternate Mode.
    pub tbt3: bool,

    /// Supports Alternate Modes that reconfigure the signals on the USB Type-C
    /// 2.4 connector, except for Thunderbolt 3.
    pub reconfigures_type_c: bool,

    /// Supports Alternate Modes that do not reconfigure the signals on the
    /// USB Type-C 2.4 connector.
    pub does_not_reconfigure_type_c: bool,
}

impl From<u8> for AlternateModes {
    fn from(value: u8) -> Self {
        Self {
            tbt3: value & 0b001 != 0,
            reconfigures_type_c: value & 0b010 != 0,
            does_not_reconfigure_type_c: value & 0b100 != 0,
        }
    }
}

/// Whether `VCONN` is needed for the AMA to operate.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VconnPower {
    /// 1W
    OneW,

    /// 1.5W
    OnePointFiveW,

    /// 2W
    TwoW,

    /// 3W
    ThreeW,

    /// 4W
    FourW,

    /// 5W
    FiveW,

    /// 6W
    SixW,
}

impl TryFrom<u8> for VconnPower {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0b000 => Ok(Self::OneW),
            0b001 => Ok(Self::OnePointFiveW),
            0b010 => Ok(Self::TwoW),
            0b011 => Ok(Self::ThreeW),
            0b100 => Ok(Self::FourW),
            0b101 => Ok(Self::FiveW),
            0b110 => Ok(Self::SixW),
            _ => Err(()),
        }
    }
}

/// The UFP's Capabilities when operating as either a PDUSB Device or Hub.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct DeviceCapability {
    /// USB 2.0 Device Capable
    pub usb2p0: bool,

    /// USB 2.0 Device Capable (Billboard only)
    pub usb2p0_billboard_only: bool,

    /// USB 3.2 Device Capable
    pub usb3p2: bool,

    /// USB4 Device Capable
    pub usb4: bool,
}

impl From<u8> for DeviceCapability {
    fn from(value: u8) -> Self {
        Self {
            usb2p0: value & 0b0001 != 0,
            usb2p0_billboard_only: value & 0b0010 != 0,
            usb3p2: value & 0b0100 != 0,
            usb4: value & 0b1000 != 0,
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
                (0b010, UsbHighestSpeed::Usb3p2Gen2),
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

    mod vconn_power {
        use super::*;

        #[test]
        fn all_valid_variants() {
            let cases: [(u8, VconnPower); 7] = [
                (0b000, VconnPower::OneW),
                (0b001, VconnPower::OnePointFiveW),
                (0b010, VconnPower::TwoW),
                (0b011, VconnPower::ThreeW),
                (0b100, VconnPower::FourW),
                (0b101, VconnPower::FiveW),
                (0b110, VconnPower::SixW),
            ];
            for (raw, expected) in cases {
                assert_eq!(VconnPower::try_from(raw), Ok(expected), "raw={raw}");
            }
        }

        #[test]
        fn invalid_values() {
            for v in 7..=255u8 {
                assert!(VconnPower::try_from(v).is_err(), "raw={v} should be invalid");
            }
        }
    }
}

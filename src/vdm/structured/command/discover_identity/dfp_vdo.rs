//! A DFP (Downward Facing Port) is a Port that provides power and/or data to a
//! UFP. DFPs include Hosts, Hubs, and Power Bricks.
//!
//! See PD spec 6.4.4.3.1.5 DFP VDO.

use crate::vdm::structured::command::discover_identity::ProductTypeVdo;

/// Returned by Ports capable as operating as a DFP, including those implemented
/// by Hosts, Hubs, and Power Bricks.
///
/// Sent based on the value of [`sop::IdHeaderVdo::product_type_dfp`][super::sop::IdHeaderVdo::product_type_dfp].
///
/// See PD spec 6.4.4.3.1.5 DFP VDO,
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct DfpVdo {
    /// A unique number that unambiguously identifies each USB Type-C 2.4 DFP, including
    /// DRPs, on the device.
    ///
    /// This number is independent of the USB Port number.
    pub port_number: u8,

    /// Whether the FDP can operate as a PDUSB Host, and its Capabilities when operating
    /// as such.
    pub host_capability: HostCapability,
}

impl From<Raw> for DfpVdo {
    fn from(raw: Raw) -> Self {
        Self {
            port_number: raw.port_number(),
            host_capability: raw.host_capability().into(),
        }
    }
}

impl From<u32> for DfpVdo {
    fn from(value: u32) -> Self {
        Raw(value).into()
    }
}

impl From<ProductTypeVdo> for DfpVdo {
    fn from(value: ProductTypeVdo) -> Self {
        value.0.into()
    }
}

impl From<[u8; 4]> for DfpVdo {
    fn from(bytes: [u8; 4]) -> Self {
        u32::from_le_bytes(bytes).into()
    }
}

bitfield::bitfield! {
    /// The raw value of a [`DfpVdo`], before parsing enumerations and bitfields.
    #[derive(Copy, Clone, PartialEq, Eq)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub struct Raw(u32);
    impl Debug;

    /// See [`DfpVdo::port_number`].
    pub u8, port_number, set_port_number: 4, 0;

    /// See [`DfpVdo::host_capability`].
    pub u8, host_capability, set_host_capability: 26, 24;
}

/// A DFP's Capabilities when operating as a PDUSB Host.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct HostCapability {
    /// USB 2.0 Host Capable.
    pub usb2p0: bool,

    /// USB 3.2 Host Capable.
    pub usb3p2: bool,

    /// USB4 Host Capable.
    pub usb4: bool,
}

impl From<u8> for HostCapability {
    fn from(value: u8) -> Self {
        Self {
            usb2p0: value & 0b001 != 0,
            usb3p2: value & 0b010 != 0,
            usb4: value & 0b100 != 0,
        }
    }
}

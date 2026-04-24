//! An Active Cable has a USB Plug on each end, at least one of which is a Cable
//! Plug supporting SOP' Communication. It incorporates data bus signal conditioning
//! circuits.
//!
//! See PD spec 6.4.4.3.1.7 Active Cable VDO.

use crate::vdm::structured::command::discover_identity::ProductTypeVdo;

/// An Active Cable has a USB Plug on each end, at least one of which is a Cable
/// Plug supporting SOP' Communication. It incorporates data bus signal conditioning
/// circuits.
///
/// Sent based on the value of [`sop_prime::IdHeaderVdo::product_type`][super::sop_prime::IdHeaderVdo::product_type].
///
/// See PD spec 6.4.4.3.1.7 Active Cable VDO, table 6.42 Active Cable VDO1.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ActiveCableVdo1 {
    /// The highest rate the cable supports.
    pub usb_highest_speed: UsbHighestSpeed,

    /// Whether one of the Cable Plugs is capable of SOP'' Communication in addition
    /// to the normative SOP' Communication.
    pub soppp_controller_present: bool,

    /// Whether the cable contains an end-to-end `VBUS` wire.
    pub vbus_through_cable: bool,

    /// Whether the cable is capable of carrying 3A or 5A.
    pub vbus_current_handling_capability: VbusCurrentHandlingCapability,

    /// Whether the SBUs are passive or active (e.g., digital).
    pub sbu_type: SbuType,

    /// Whether the cable supports the SBUs in the cable.
    pub sbu_supported: bool,

    /// The maximum voltage that shall be negotiated as part of an Explicit Contract.
    pub maximum_vbus_voltage: MaximumVbusVoltage,

    /// Whether the Active Cable has one or two Cable Plugs requiring power from
    /// `VCONN`.
    pub cable_termination_type: CableTerminationType,

    /// The signal latency through the cable, which can be used as an approximation
    /// for its length.
    pub cable_latency: CableLatency,

    /// The cable is specifically designed for safe operation when carrying up to
    /// 48 volts at 5 amps.
    pub epr_capable: bool,

    /// Indicates whether the opposite end from the USB Type-C plug is another USB
    /// Type-C plug or is a Captive Cable Assembly.
    pub usb_type_c_or_captive: UsbTypeCOrCaptive,

    /// FW version assigned by the VID owner.
    pub firmware_version: u8,

    /// HW version assigned by the VID owner.
    pub hw_version: u8,
}

/// An Active Cable has a USB Plug on each end, at least one of which is a Cable
/// Plug supporting SOP' Communication. It incorporates data bus signal conditioning
/// circuits.
///
/// Sent based on the value of [`sop_prime::IdHeaderVdo::product_type`][super::sop_prime::IdHeaderVdo::product_type].
///
/// See PD spec 6.4.4.3.1.7 Active Cable VDO, table 6.43 Active Cable VDO2.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ActiveCableVdo2 {
    /// The signaling generation the cable supports.
    pub usb_gen: UsbGen,

    /// Whether or not the cable supports asymmetric mode, as defined by the USB4
    /// and USB Type-C 2.4 specifications.
    pub usb4_asymmetric_mode_supported: bool,

    /// Whether or not the cable is optically isolated, as defined by the USB Type-C
    /// 2.4 specification.
    pub optically_isolated_active_cable: bool,

    /// The number of lanes the cable supports.
    pub usb_lanes_supported: UsbLanesSupported,

    /// Whether or not the cable supports USB 3.2 SuperSpeed signaling.
    pub usb3p2_supported: bool,

    /// Whether or not the cable supports USB 2.0 only signaling.
    pub usb2p0_supported: bool,

    /// The number of USB 2.0 "hub hops" that are lost due to the transmission time
    /// of the cable.
    pub usb2p0_hub_hops_consumed: u8,

    /// Whether or not the cable supports USB4 operation.
    pub usb4_supported: bool,

    /// The cable's active element.
    pub active_element: ActiveElement,

    /// The cable's construction between the active elements.
    pub physical_connection: PhysicalConnection,

    /// Which U3 to U0 mode the cable supports.
    pub u3_to_u0_transition_mode: U3ToU0TransitionMode,

    /// The power the cable consumes while in USB 3.2 U3 or USB4 CLd.
    pub u3_cld_power: U3CldPower,

    /// The temperature, in degrees Celsius, at which the plug will shut down its
    /// active signaling components.
    pub shutdown_temperature: u8,

    /// The maximum allowable operating temperature inside the plug, in degrees Celsius.
    pub maximum_operating_temperature: u8,
}

/// Errors that can occur when parsing an [`ActiveCableVdo1`] from a raw value.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ParseActiveCableVdo1Error {
    /// [`ActiveCableVdo1::usb_highest_speed`] contains an invalid value.
    InvalidUsbHighestSpeed,

    /// [`ActiveCableVdo1::vbus_current_handling_capability`] contains an invalid value.
    InvalidVbusCurrentHandlingCapability,

    /// [`ActiveCableVdo1::sbu_type`] contains an invalid value.
    InvalidSbuType,

    /// [`ActiveCableVdo1::maximum_vbus_voltage`] contains an invalid value.
    InvalidMaximumVbusVoltage,

    /// [`ActiveCableVdo1::cable_termination_type`] contains an invalid value.
    InvalidCableTerminationType,

    /// [`ActiveCableVdo1::cable_latency`] contains an invalid value.
    InvalidCableLatency,

    /// [`ActiveCableVdo1::usb_type_c_or_captive`] contains an invalid value.
    InvalidUsbTypeCOrCaptive,
}

impl TryFrom<Raw1> for ActiveCableVdo1 {
    type Error = ParseActiveCableVdo1Error;

    fn try_from(raw: Raw1) -> Result<Self, Self::Error> {
        Ok(Self {
            usb_highest_speed: raw
                .usb_highest_speed()
                .try_into()
                .map_err(|()| ParseActiveCableVdo1Error::InvalidUsbHighestSpeed)?,
            soppp_controller_present: raw.soppp_controller_present(),
            vbus_through_cable: raw.vbus_through_cable(),
            vbus_current_handling_capability: raw
                .vbus_current_handling_capability()
                .try_into()
                .map_err(|()| ParseActiveCableVdo1Error::InvalidVbusCurrentHandlingCapability)?,
            sbu_type: raw
                .sbu_type()
                .try_into()
                .map_err(|()| ParseActiveCableVdo1Error::InvalidSbuType)?,
            sbu_supported: !raw.sbu_supported_n(),
            maximum_vbus_voltage: raw
                .maximum_vbus_voltage()
                .try_into()
                .map_err(|()| ParseActiveCableVdo1Error::InvalidMaximumVbusVoltage)?,
            cable_termination_type: raw
                .cable_termination_type()
                .try_into()
                .map_err(|()| ParseActiveCableVdo1Error::InvalidCableTerminationType)?,
            cable_latency: raw
                .cable_latency()
                .try_into()
                .map_err(|()| ParseActiveCableVdo1Error::InvalidCableLatency)?,
            epr_capable: raw.epr_capable(),
            usb_type_c_or_captive: raw
                .usb_type_c_or_captive()
                .try_into()
                .map_err(|()| ParseActiveCableVdo1Error::InvalidUsbTypeCOrCaptive)?,
            firmware_version: raw.firmware_version(),
            hw_version: raw.hw_version(),
        })
    }
}

impl TryFrom<u32> for ActiveCableVdo1 {
    type Error = ParseActiveCableVdo1Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Raw1(value).try_into()
    }
}

impl TryFrom<ProductTypeVdo> for ActiveCableVdo1 {
    type Error = ParseActiveCableVdo1Error;

    fn try_from(value: ProductTypeVdo) -> Result<Self, Self::Error> {
        Raw1(value.0).try_into()
    }
}

impl TryFrom<[u8; 4]> for ActiveCableVdo1 {
    type Error = ParseActiveCableVdo1Error;

    fn try_from(bytes: [u8; 4]) -> Result<Self, Self::Error> {
        let value = u32::from_le_bytes(bytes);
        Raw1(value).try_into()
    }
}

/// Errors that can occur when parsing an [`ActiveCableVdo2`] from a raw value.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ParseActiveCableVdo2Error {
    /// [`ActiveCableVdo2::usb_gen`] contains an invalid value.
    InvalidUsbGen,

    /// [`ActiveCableVdo2::usb4_asymmetric_mode_supported`] contains an invalid value.
    InvalidUsb4AsymmetricModeSupported,

    /// [`ActiveCableVdo2::optically_isolated_active_cable`] contains an invalid value.
    InvalidOpticallyIsolatedActiveCable,

    /// [`ActiveCableVdo2::usb_lanes_supported`] contains an invalid value.
    InvalidUsbLanesSupported,

    /// [`ActiveCableVdo2::usb3p2_supported`] contains an invalid value.
    InvalidUsb3p2Supported,

    /// [`ActiveCableVdo2::usb2p0_supported`] contains an invalid value.
    InvalidUsb2p0Supported,

    /// [`ActiveCableVdo2::usb2p0_hub_hops_consumed`] contains an invalid value.
    InvalidUsb2p0HubHopsConsumed,

    /// [`ActiveCableVdo2::usb4_supported`] contains an invalid value.
    InvalidUsb4Supported,

    /// [`ActiveCableVdo2::active_element`] contains an invalid value.
    InvalidActiveElement,

    /// [`ActiveCableVdo2::physical_connection`] contains an invalid value.
    InvalidPhysicalConnection,

    /// [`ActiveCableVdo2::u3_to_u0_transition_mode`] contains an invalid value.
    InvalidU3ToU0TransitionMode,

    /// [`ActiveCableVdo2::u3_cld_power`] contains an invalid value.
    InvalidU3CldPower,
}

impl TryFrom<Raw2> for ActiveCableVdo2 {
    type Error = ParseActiveCableVdo2Error;

    fn try_from(raw: Raw2) -> Result<Self, Self::Error> {
        Ok(Self {
            usb_gen: raw
                .usb_gen()
                .try_into()
                .map_err(|()| ParseActiveCableVdo2Error::InvalidUsbGen)?,
            usb4_asymmetric_mode_supported: raw.usb4_asymmetric_mode_supported(),
            optically_isolated_active_cable: raw.optically_isolated_active_cable(),
            usb_lanes_supported: raw
                .usb_lanes_supported()
                .try_into()
                .map_err(|()| ParseActiveCableVdo2Error::InvalidUsbLanesSupported)?,
            usb3p2_supported: !raw.usb3p2_supported_n(),
            usb2p0_supported: !raw.usb2p0_supported_n(),
            usb2p0_hub_hops_consumed: raw.usb2p0_hub_hops_consumed(),
            usb4_supported: !raw.usb4_supported_n(),
            active_element: raw
                .active_element()
                .try_into()
                .map_err(|()| ParseActiveCableVdo2Error::InvalidActiveElement)?,
            physical_connection: raw
                .physical_connection()
                .try_into()
                .map_err(|()| ParseActiveCableVdo2Error::InvalidPhysicalConnection)?,
            u3_to_u0_transition_mode: raw
                .u3_to_u0_transition_mode()
                .try_into()
                .map_err(|()| ParseActiveCableVdo2Error::InvalidU3ToU0TransitionMode)?,
            u3_cld_power: raw
                .u3_cld_power()
                .try_into()
                .map_err(|()| ParseActiveCableVdo2Error::InvalidU3CldPower)?,
            shutdown_temperature: raw.shutdown_temperature(),
            maximum_operating_temperature: raw.maximum_operating_temperature(),
        })
    }
}

impl TryFrom<u32> for ActiveCableVdo2 {
    type Error = ParseActiveCableVdo2Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Raw2(value).try_into()
    }
}

impl TryFrom<ProductTypeVdo> for ActiveCableVdo2 {
    type Error = ParseActiveCableVdo2Error;

    fn try_from(value: ProductTypeVdo) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl TryFrom<[u8; 4]> for ActiveCableVdo2 {
    type Error = ParseActiveCableVdo2Error;

    fn try_from(bytes: [u8; 4]) -> Result<Self, Self::Error> {
        u32::from_le_bytes(bytes).try_into()
    }
}

bitfield::bitfield! {
    /// The raw value of an [`ActiveCableVdo1`], before parsing enumerations and bitfields.
    #[derive(Copy, Clone, PartialEq, Eq)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub struct Raw1(u32);
    impl Debug;

    /// See [`ActiveCableVdo1::usb_highest_speed`].
    pub u8, usb_highest_speed, set_usb_highest_speed: 2, 0;

    /// See [`ActiveCableVdo1::soppp_controller_present`].
    pub bool, soppp_controller_present, set_soppp_controller_present: 3;

    /// See [`ActiveCableVdo1::vbus_through_cable`].
    pub bool, vbus_through_cable, set_vbus_through_cable: 4;

    /// See [`ActiveCableVdo1::vbus_current_handling_capability`].
    pub u8, vbus_current_handling_capability, set_vbus_current_handling_capability: 6, 5;

    /// See [`ActiveCableVdo1::sbu_type`].
    pub u8, sbu_type, set_sbu_type: 7, 7;

    /// See [`ActiveCableVdo1::sbu_supported`].
    pub bool, sbu_supported_n, set_sbu_supported_n: 8;

    /// See [`ActiveCableVdo1::maximum_vbus_voltage`].
    pub u8, maximum_vbus_voltage, set_maximum_vbus_voltage: 10, 9;

    /// See [`ActiveCableVdo1::cable_termination_type`].
    pub u8, cable_termination_type, set_cable_termination_type: 12, 11;

    /// See [`ActiveCableVdo1::cable_latency`].
    pub u8, cable_latency, set_cable_latency: 16, 13;

    /// See [`ActiveCableVdo1::epr_capable`].
    pub bool, epr_capable, set_epr_capable: 17;

    /// See [`ActiveCableVdo1::usb_type_c_or_captive`].
    pub u8, usb_type_c_or_captive, set_usb_type_c_or_captive: 19, 18;

    /// See [`ActiveCableVdo1::firmware_version`].
    pub u8, firmware_version, set_firmware_version: 27, 24;

    /// See [`ActiveCableVdo1::hw_version`].
    pub u8, hw_version, set_hw_version: 31, 28;
}

bitfield::bitfield! {
    /// The raw value of an [`ActiveCableVdo2`], before parsing enumerations and bitfields.
    #[derive(Copy, Clone, PartialEq, Eq)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub struct Raw2(u32);
    impl Debug;

    /// See [`ActiveCableVdo2::usb_gen`].
    pub u8, usb_gen, set_usb_gen: 0, 0;

    /// See [`ActiveCableVdo2::usb4_asymmetric_mode_supported`].
    pub bool, usb4_asymmetric_mode_supported, set_usb4_asymmetric_mode_supported: 1;

    /// See [`ActiveCableVdo2::optically_isolated_active_cable`].
    pub bool, optically_isolated_active_cable, set_optically_isolated_active_cable: 2;

    /// See [`ActiveCableVdo2::usb_lanes_supported`].
    pub u8, usb_lanes_supported, set_usb_lanes_supported: 3, 3;

    /// See [`ActiveCableVdo2::usb3p2_supported`].
    pub bool, usb3p2_supported_n, set_usb3p2_supported_n: 4;

    /// See [`ActiveCableVdo2::usb2p0_supported`].
    pub bool, usb2p0_supported_n, set_usb2p0_supported_n: 5;

    /// See [`ActiveCableVdo2::usb2p0_hub_hops_consumed`].
    pub u8, usb2p0_hub_hops_consumed, set_usb2p0_hub_hops_consumed: 7, 6;

    /// See [`ActiveCableVdo2::usb4_supported`].
    pub bool, usb4_supported_n, set_usb4_supported_n: 8;

    /// See [`ActiveCableVdo2::active_element`].
    pub u8, active_element, set_active_element: 9, 9;

    /// See [`ActiveCableVdo2::physical_connection`].
    pub u8, physical_connection, set_physical_connection: 10, 10;

    /// See [`ActiveCableVdo2::u3_to_u0_transition_mode`].
    pub u8, u3_to_u0_transition_mode, set_u3_to_u0_transition_mode: 11, 11;

    /// See [`ActiveCableVdo2::u3_cld_power`].
    pub u8, u3_cld_power, set_u3_cld_power: 14, 12;

    /// See [`ActiveCableVdo2::shutdown_temperature`].
    pub u8, shutdown_temperature, set_shutdown_temperature: 23, 16;

    /// See [`ActiveCableVdo2::maximum_operating_temperature`].
    pub u8, maximum_operating_temperature, set_maximum_operating_temperature: 31, 24;
}

/// The highest rate the cable supports.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UsbHighestSpeed {
    /// USB 2.0 only, no SuperSpeed support.
    Usb2p0,

    /// USB 3.2 Gen1
    Usb3p2Gen1,

    /// USB 3.2 and USB4 Gen2
    Usb3p2,

    /// USB4 Gen3
    Usb4Gen3,

    /// USB4 Gen4
    Usb4Gen4,
}

impl TryFrom<u8> for UsbHighestSpeed {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, ()> {
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

/// Whether the cable is capable of carrying 3A or 5A.
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

/// Whether the SBUs are passive or active (e.g., digital).
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SbuType {
    /// SBU is passive.
    Passive,

    /// SBU is active (e.g., digital).
    Active,
}

impl TryFrom<u8> for SbuType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Passive),
            1 => Ok(Self::Active),
            _ => Err(()),
        }
    }
}

/// The maximum voltage that shall be negotiated as part of an Explicit Contract.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MaximumVbusVoltage {
    /// 20V
    TwentyV,

    /// 30V
    ThirtyV,

    /// 40V
    FortyV,

    /// 50V
    FiftyV,
}

impl TryFrom<u8> for MaximumVbusVoltage {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0b00 => Ok(Self::TwentyV),
            0b01 => Ok(Self::ThirtyV),
            0b10 => Ok(Self::FortyV),
            0b11 => Ok(Self::FiftyV),
            _ => Err(()),
        }
    }
}

/// Whether the Active Cable has one or two Cable Plugs requiring power from
/// `VCONN`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CableTerminationType {
    /// One end of the cable is active.
    OneEndActive,

    /// Both ends of the cable are active.
    BothEndsActive,
}

impl TryFrom<u8> for CableTerminationType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0b10 => Ok(Self::OneEndActive),
            0b11 => Ok(Self::BothEndsActive),
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

    /// 1000ns (~100m)
    LessThan1000ns,

    /// 2000ns (~200m)
    LessThan2000ns,

    /// 3000ns (~300m)
    LessThan3000ns,
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
            0b1000 => Ok(Self::LessThan1000ns),
            0b1001 => Ok(Self::LessThan2000ns),
            0b1010 => Ok(Self::LessThan3000ns),
            _ => Err(()),
        }
    }
}

/// Indicates whether the opposite end from the USB Type-C plug is another USB
/// Type-C plug or is a Captive Cable Assembly.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UsbTypeCOrCaptive {
    /// The opposite end from the USB Type-C plug is another USB Type-C plug.
    UsbTypeC,

    /// The opposite end from the USB Type-C plug is a Captive Cable Assembly.
    Captive,
}

impl TryFrom<u8> for UsbTypeCOrCaptive {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0b10 => Ok(Self::UsbTypeC),
            0b11 => Ok(Self::Captive),
            _ => Err(()),
        }
    }
}

/// The signaling generation the cable supports.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UsbGen {
    /// Gen1
    Gen1,

    /// Gen2 or higher.
    Gen2OrHigher,
}

impl TryFrom<u8> for UsbGen {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, ()> {
        match value {
            0 => Ok(Self::Gen1),
            1 => Ok(Self::Gen2OrHigher),
            _ => Err(()),
        }
    }
}

/// The number of lanes the cable supports.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UsbLanesSupported {
    /// The cable supports one lane.
    OneLane,

    /// The cable supports two lanes.
    TwoLanes,
}

impl TryFrom<u8> for UsbLanesSupported {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, ()> {
        match value {
            0 => Ok(Self::OneLane),
            1 => Ok(Self::TwoLanes),
            _ => Err(()),
        }
    }
}

/// The cable's active element.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ActiveElement {
    /// The active element is a re-driver.
    Redriver,

    /// The active element is a re-timer.
    Retimer,
}

impl TryFrom<u8> for ActiveElement {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, ()> {
        match value {
            0 => Ok(Self::Redriver),
            1 => Ok(Self::Retimer),
            _ => Err(()),
        }
    }
}

/// The cable's construction between the active elements.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PhysicalConnection {
    /// The connection between the active elements is made of copper.
    Copper,

    /// The connection between the active elements is made of optical fiber.
    Optical,
}

impl TryFrom<u8> for PhysicalConnection {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, ()> {
        match value {
            0 => Ok(Self::Copper),
            1 => Ok(Self::Optical),
            _ => Err(()),
        }
    }
}

/// Which U3 to U0 mode the cable supports.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum U3ToU0TransitionMode {
    /// The cable supports a direct transition from U3 to U0.
    Direct,

    /// The cable supports a transition from U3 to U0 through U3S.
    ThroughU3S,
}

impl TryFrom<u8> for U3ToU0TransitionMode {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, ()> {
        match value {
            0 => Ok(Self::Direct),
            1 => Ok(Self::ThroughU3S),
            _ => Err(()),
        }
    }
}

/// The power the cable consumes while in USB 3.2 U3 or USB4 CLd.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum U3CldPower {
    /// >10mW
    GreaterThan10Milliwatts,

    /// 5-10mW
    FiveToTenMilliwatts,

    /// 1-5mW
    OneToFiveMilliwatts,

    /// 0.5-1mW
    P5To1Milliwatt,

    /// 0.2-0.5mW
    P2ToP5Milliwatt,

    /// 50-200µW
    FiftyToTwoHundredMicrowatts,

    /// <50µW
    LessThanFiftyMicrowatts,
}

impl TryFrom<u8> for U3CldPower {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, ()> {
        match value {
            0b000 => Ok(Self::GreaterThan10Milliwatts),
            0b001 => Ok(Self::FiveToTenMilliwatts),
            0b010 => Ok(Self::OneToFiveMilliwatts),
            0b011 => Ok(Self::P5To1Milliwatt),
            0b100 => Ok(Self::P2ToP5Milliwatt),
            0b101 => Ok(Self::FiftyToTwoHundredMicrowatts),
            0b110 => Ok(Self::LessThanFiftyMicrowatts),
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

    mod sbu_type {
        use super::*;

        #[test]
        fn all_valid_variants() {
            let cases: [(u8, SbuType); 2] = [(0, SbuType::Passive), (1, SbuType::Active)];
            for (raw, expected) in cases {
                assert_eq!(SbuType::try_from(raw), Ok(expected), "raw={raw}");
            }
        }

        #[test]
        fn invalid_values() {
            for v in 2..=255u8 {
                assert!(SbuType::try_from(v).is_err(), "raw={v} should be invalid");
            }
        }
    }

    mod maximum_vbus_voltage {
        use super::*;

        #[test]
        fn all_valid_variants() {
            let cases: [(u8, MaximumVbusVoltage); 4] = [
                (0b00, MaximumVbusVoltage::TwentyV),
                (0b01, MaximumVbusVoltage::ThirtyV),
                (0b10, MaximumVbusVoltage::FortyV),
                (0b11, MaximumVbusVoltage::FiftyV),
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
                (0b10, CableTerminationType::OneEndActive),
                (0b11, CableTerminationType::BothEndsActive),
            ];
            for (raw, expected) in cases {
                assert_eq!(CableTerminationType::try_from(raw), Ok(expected), "raw={raw}");
            }
        }

        #[test]
        fn invalid_values() {
            for v in [0u8, 1] {
                assert!(CableTerminationType::try_from(v).is_err(), "raw={v} should be invalid");
            }
        }
    }

    mod cable_latency {
        use super::*;

        #[test]
        fn all_valid_variants() {
            let cases: [(u8, CableLatency); 10] = [
                (0b0001, CableLatency::LessThan10ns),
                (0b0010, CableLatency::LessThan20ns),
                (0b0011, CableLatency::LessThan30ns),
                (0b0100, CableLatency::LessThan40ns),
                (0b0101, CableLatency::LessThan50ns),
                (0b0110, CableLatency::LessThan60ns),
                (0b0111, CableLatency::LessThan70ns),
                (0b1000, CableLatency::LessThan1000ns),
                (0b1001, CableLatency::LessThan2000ns),
                (0b1010, CableLatency::LessThan3000ns),
            ];
            for (raw, expected) in cases {
                assert_eq!(CableLatency::try_from(raw), Ok(expected), "raw={raw}");
            }
        }

        #[test]
        fn invalid_values() {
            assert!(CableLatency::try_from(0u8).is_err());
            for v in 11..=255u8 {
                assert!(CableLatency::try_from(v).is_err(), "raw={v} should be invalid");
            }
        }
    }

    mod usb_type_c_or_captive {
        use super::*;

        #[test]
        fn all_valid_variants() {
            let cases: [(u8, UsbTypeCOrCaptive); 2] =
                [(0b10, UsbTypeCOrCaptive::UsbTypeC), (0b11, UsbTypeCOrCaptive::Captive)];
            for (raw, expected) in cases {
                assert_eq!(UsbTypeCOrCaptive::try_from(raw), Ok(expected), "raw={raw}");
            }
        }

        #[test]
        fn invalid_values() {
            for v in [0u8, 1] {
                assert!(UsbTypeCOrCaptive::try_from(v).is_err(), "raw={v} should be invalid");
            }
        }
    }

    mod usb_gen {
        use super::*;

        #[test]
        fn all_valid_variants() {
            let cases: [(u8, UsbGen); 2] = [(0, UsbGen::Gen1), (1, UsbGen::Gen2OrHigher)];
            for (raw, expected) in cases {
                assert_eq!(UsbGen::try_from(raw), Ok(expected), "raw={raw}");
            }
        }

        #[test]
        fn invalid_values() {
            for v in 2..=255u8 {
                assert!(UsbGen::try_from(v).is_err(), "raw={v} should be invalid");
            }
        }
    }

    mod usb_lanes_supported {
        use super::*;

        #[test]
        fn all_valid_variants() {
            let cases: [(u8, UsbLanesSupported); 2] =
                [(0, UsbLanesSupported::OneLane), (1, UsbLanesSupported::TwoLanes)];
            for (raw, expected) in cases {
                assert_eq!(UsbLanesSupported::try_from(raw), Ok(expected), "raw={raw}");
            }
        }

        #[test]
        fn invalid_values() {
            for v in 2..=255u8 {
                assert!(UsbLanesSupported::try_from(v).is_err(), "raw={v} should be invalid");
            }
        }
    }

    mod active_element {
        use super::*;

        #[test]
        fn all_valid_variants() {
            let cases: [(u8, ActiveElement); 2] = [(0, ActiveElement::Redriver), (1, ActiveElement::Retimer)];
            for (raw, expected) in cases {
                assert_eq!(ActiveElement::try_from(raw), Ok(expected), "raw={raw}");
            }
        }

        #[test]
        fn invalid_values() {
            for v in 2..=255u8 {
                assert!(ActiveElement::try_from(v).is_err(), "raw={v} should be invalid");
            }
        }
    }

    mod physical_connection {
        use super::*;

        #[test]
        fn all_valid_variants() {
            let cases: [(u8, PhysicalConnection); 2] =
                [(0, PhysicalConnection::Copper), (1, PhysicalConnection::Optical)];
            for (raw, expected) in cases {
                assert_eq!(PhysicalConnection::try_from(raw), Ok(expected), "raw={raw}");
            }
        }

        #[test]
        fn invalid_values() {
            for v in 2..=255u8 {
                assert!(PhysicalConnection::try_from(v).is_err(), "raw={v} should be invalid");
            }
        }
    }

    mod u3_to_u0_transition_mode {
        use super::*;

        #[test]
        fn all_valid_variants() {
            let cases: [(u8, U3ToU0TransitionMode); 2] =
                [(0, U3ToU0TransitionMode::Direct), (1, U3ToU0TransitionMode::ThroughU3S)];
            for (raw, expected) in cases {
                assert_eq!(U3ToU0TransitionMode::try_from(raw), Ok(expected), "raw={raw}");
            }
        }

        #[test]
        fn invalid_values() {
            for v in 2..=255u8 {
                assert!(U3ToU0TransitionMode::try_from(v).is_err(), "raw={v} should be invalid");
            }
        }
    }

    mod u3_cld_power {
        use super::*;

        #[test]
        fn all_valid_variants() {
            let cases: [(u8, U3CldPower); 7] = [
                (0b000, U3CldPower::GreaterThan10Milliwatts),
                (0b001, U3CldPower::FiveToTenMilliwatts),
                (0b010, U3CldPower::OneToFiveMilliwatts),
                (0b011, U3CldPower::P5To1Milliwatt),
                (0b100, U3CldPower::P2ToP5Milliwatt),
                (0b101, U3CldPower::FiftyToTwoHundredMicrowatts),
                (0b110, U3CldPower::LessThanFiftyMicrowatts),
            ];
            for (raw, expected) in cases {
                assert_eq!(U3CldPower::try_from(raw), Ok(expected), "raw={raw}");
            }
        }

        #[test]
        fn invalid_values() {
            for v in 7..=255u8 {
                assert!(U3CldPower::try_from(v).is_err(), "raw={v} should be invalid");
            }
        }
    }
}

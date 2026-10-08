//! Types for GET_CONNECTOR_STATUS command, see UCSI spec 6.6

use bitfield::bitfield;
use bytemuck::{Pod, Zeroable};

use crate::ucsi::v1_2::lpm::ConnectorNumberRaw;
use crate::ucsi::v1_2::ppm::set_notification_enable::NotificationEnable;
use crate::ucsi::v1_2::{CommandHeaderRaw, COMMAND_LEN};
use crate::{PortId, PowerRole};

/// Data length for the GET_CONNECTOR_STATUS command response
pub const RESPONSE_DATA_LEN: usize = 11;
/// Command padding
pub const COMMAND_PADDING: usize = COMMAND_LEN - size_of::<CommandHeaderRaw>() - size_of::<ConnectorNumberRaw>();

/// Command arguments
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Args<T: PortId> {
    /// Connector number
    pub connector_number: T,
}

impl<T: PortId> Default for Args<T> {
    fn default() -> Self {
        Self {
            connector_number: T::from(0),
        }
    }
}

/// Raw wire format of [`Args`]
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Zeroable, Pod)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ArgsRaw {
    /// Connector number in bits 6:0
    pub connector: u8,
    /// Reserved bytes, filling out the remainder of the command
    _reserved: [u8; COMMAND_PADDING],
}

impl ArgsRaw {
    /// Length of the raw arguments in bytes
    pub const LEN: usize = size_of::<Self>();
}

impl<T: PortId> From<Args<T>> for ArgsRaw {
    /// Converts the arguments into their raw wire format
    fn from(args: Args<T>) -> Self {
        let mut connector = ConnectorNumberRaw::default();
        connector.set_connector_number(args.connector_number.into());
        Self {
            connector: connector.0,
            ..Default::default()
        }
    }
}

impl<T: PortId> From<ArgsRaw> for Args<T> {
    /// Reconstructs the arguments from their raw wire format
    fn from(raw: ArgsRaw) -> Self {
        Self {
            connector_number: ConnectorNumberRaw(raw.connector).connector_number().into(),
        }
    }
}

bitfield! {
    /// Connector Status Change bitmap
    #[derive(Copy, Default, Clone, PartialEq, Eq)]
    pub struct ConnectorStatusChangeRaw(u16);
    impl Debug;
    /// External supply change
    pub bool, external_supply_change, set_external_supply_change: 1;
    /// Power operation mode change
    pub bool, power_op_mode_change, set_power_op_mode_change: 2;
    /// Provider capabilities change
    pub bool, provider_caps_change, set_provider_caps_change: 5;
    /// Negotiated power level change
    pub bool, negotiated_power_level_change, set_negotiated_power_level_change: 6;
    /// PD reset complete
    pub bool, pd_reset_complete, set_pd_reset_complete: 7;
    /// Supported CAM change
    pub bool, supported_cam_change, set_supported_cam_change: 8;
    /// Battery charging status change
    pub bool, battery_charging_status_change, set_battery_charging_status_change: 9;
    /// Connector partner changed
    pub bool, connector_partner_changed, set_connector_partner_changed: 11;
    /// Power direction changed
    pub bool, power_direction_changed, set_power_direction_changed: 12;
    /// Connect/disconnect
    pub bool, connect_change, set_connect_change: 14;
    /// Error
    pub bool, error, set_error: 15;
}

#[cfg(feature = "defmt")]
impl defmt::Format for ConnectorStatusChangeRaw {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "ConnectorStatusChangeRaw {{ .0: {}, \
            external_supply_change: {}, \
            power_op_mode_change: {}, \
            provider_caps_change: {}, \
            negotiated_power_level_change: {}, \
            pd_reset_complete: {}, \
            supported_cam_change: {}, \
            battery_charging_status_change: {}, \
            connector_partner_changed: {}, \
            power_direction_changed: {}, \
            connect_change: {}, \
            error: {} }}",
            self.0,
            self.external_supply_change(),
            self.power_op_mode_change(),
            self.provider_caps_change(),
            self.negotiated_power_level_change(),
            self.pd_reset_complete(),
            self.supported_cam_change(),
            self.battery_charging_status_change(),
            self.connector_partner_changed(),
            self.power_direction_changed(),
            self.connect_change(),
            self.error()
        )
    }
}

/// Connector status change flags
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ConnectorStatusChange {
    /// External supply change
    pub external_supply_change: bool,
    /// Power operation mode change
    pub power_op_mode_change: bool,
    /// Provider capabilities change
    pub provider_caps_change: bool,
    /// Negotiated power level change
    pub negotiated_power_level_change: bool,
    /// PD reset complete
    pub pd_reset_complete: bool,
    /// Supported CAM change
    pub supported_cam_change: bool,
    /// Battery charging status change
    pub battery_charging_status_change: bool,
    /// Connector partner changed
    pub connector_partner_changed: bool,
    /// Power direction changed
    pub power_direction_changed: bool,
    /// Connect/disconnect
    pub connect_change: bool,
    /// Error
    pub error: bool,
}

impl ConnectorStatusChange {
    /// Returns true if no status change flags are set
    pub fn is_empty(&self) -> bool {
        *self == Self::default()
    }

    /// Returns true if any status change flags are set
    pub fn any(&self) -> bool {
        !self.is_empty()
    }

    /// Returns a new connector status change with all flags that match the given notification enable flags
    pub fn filter_enabled(&self, enable: NotificationEnable) -> Self {
        // These bitfields have the same layout
        let connector_raw: u16 = (*self).into();
        let enable_raw: u16 = enable.into();
        Self::from(connector_raw & enable_raw)
    }
}

impl From<ConnectorStatusChangeRaw> for ConnectorStatusChange {
    fn from(raw: ConnectorStatusChangeRaw) -> Self {
        Self {
            external_supply_change: raw.external_supply_change(),
            power_op_mode_change: raw.power_op_mode_change(),
            provider_caps_change: raw.provider_caps_change(),
            negotiated_power_level_change: raw.negotiated_power_level_change(),
            pd_reset_complete: raw.pd_reset_complete(),
            supported_cam_change: raw.supported_cam_change(),
            battery_charging_status_change: raw.battery_charging_status_change(),
            connector_partner_changed: raw.connector_partner_changed(),
            power_direction_changed: raw.power_direction_changed(),
            connect_change: raw.connect_change(),
            error: raw.error(),
        }
    }
}

impl From<ConnectorStatusChange> for ConnectorStatusChangeRaw {
    fn from(change: ConnectorStatusChange) -> Self {
        let mut raw = ConnectorStatusChangeRaw::default();
        raw.set_external_supply_change(change.external_supply_change);
        raw.set_power_op_mode_change(change.power_op_mode_change);
        raw.set_provider_caps_change(change.provider_caps_change);
        raw.set_negotiated_power_level_change(change.negotiated_power_level_change);
        raw.set_pd_reset_complete(change.pd_reset_complete);
        raw.set_supported_cam_change(change.supported_cam_change);
        raw.set_battery_charging_status_change(change.battery_charging_status_change);
        raw.set_connector_partner_changed(change.connector_partner_changed);
        raw.set_power_direction_changed(change.power_direction_changed);
        raw.set_connect_change(change.connect_change);
        raw.set_error(change.error);
        raw
    }
}

impl From<u16> for ConnectorStatusChange {
    fn from(raw: u16) -> Self {
        ConnectorStatusChangeRaw(raw).into()
    }
}

impl From<ConnectorStatusChange> for u16 {
    fn from(change: ConnectorStatusChange) -> Self {
        ConnectorStatusChangeRaw::from(change).0
    }
}

/// Power Operation Mode
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum PowerOperationMode {
    /// USB default current
    #[default]
    UsbDefault = 0x1,
    /// Battery Charging (BC) mode
    Bc = 0x2,
    /// Power Delivery (PD) mode
    Pd = 0x3,
    /// Type-C 1.5A mode
    TypeC1_5A = 0x4,
    /// Type-C 3A mode
    TypeC3A = 0x5,
    /// Type-C 5A mode
    TypeC5A = 0x6,
}

/// Invalid Power Operation Mode error, contains the raw value that failed to decode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct InvalidPowerOperationMode(pub u8);

impl TryFrom<u8> for PowerOperationMode {
    type Error = InvalidPowerOperationMode;

    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0x1 => Ok(PowerOperationMode::UsbDefault),
            0x2 => Ok(PowerOperationMode::Bc),
            0x3 => Ok(PowerOperationMode::Pd),
            0x4 => Ok(PowerOperationMode::TypeC1_5A),
            0x5 => Ok(PowerOperationMode::TypeC3A),
            0x6 => Ok(PowerOperationMode::TypeC5A),
            _ => Err(InvalidPowerOperationMode(val)),
        }
    }
}

bitfield! {
    /// Raw connector partner flags
    #[derive(Copy, Default, Clone, PartialEq, Eq)]
    pub struct ConnectorPartnerFlagsRaw(u8);
    impl Debug;

    /// USB2.x or USB3.x
    pub bool, usb, set_usb: 0;
    /// Alternate mode
    pub bool, alt_mode, set_alt_mode: 1;
}

#[cfg(feature = "defmt")]
impl defmt::Format for ConnectorPartnerFlagsRaw {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "ConnectorPartnerFlagsRaw {{ .0: {}, usb: {}, alt_mode: {} }}",
            self.0,
            self.usb(),
            self.alt_mode()
        )
    }
}

/// Connector partner flags
#[derive(Copy, Debug, Default, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ConnectorPartnerFlags {
    /// USB2.x or USB3.x
    pub usb: bool,
    /// Alternate mode
    pub alt_mode: bool,
}

impl From<ConnectorPartnerFlagsRaw> for ConnectorPartnerFlags {
    fn from(raw: ConnectorPartnerFlagsRaw) -> Self {
        Self {
            usb: raw.usb(),
            alt_mode: raw.alt_mode(),
        }
    }
}

impl From<ConnectorPartnerFlags> for ConnectorPartnerFlagsRaw {
    fn from(flags: ConnectorPartnerFlags) -> Self {
        let mut raw = ConnectorPartnerFlagsRaw::default();
        raw.set_usb(flags.usb);
        raw.set_alt_mode(flags.alt_mode);
        raw
    }
}

impl From<u8> for ConnectorPartnerFlags {
    fn from(value: u8) -> Self {
        ConnectorPartnerFlagsRaw(value).into()
    }
}

impl From<ConnectorPartnerFlags> for u8 {
    fn from(flags: ConnectorPartnerFlags) -> Self {
        ConnectorPartnerFlagsRaw::from(flags).0
    }
}

/// Connector Partner Type
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum ConnectorPartnerType {
    /// Downstream Facing Port (DFP) attached
    #[default]
    DfpAttached = 0x1,
    /// Upstream Facing Port (UFP) attached
    UfpAttached = 0x2,
    /// Powered Cable (No UFP)
    PoweredCableNoUfp = 0x3,
    /// Powered Cable (UFP)
    PoweredCableUfp = 0x4,
    /// Debug Accessory
    DebugAccessory = 0x5,
    /// Audio Adapter Accessory
    AudioAdapterAccessory = 0x6,
}

/// Invalid Connector Partner Type error, contains the raw value that failed to decode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct InvalidConnectorPartnerType(pub u8);

impl TryFrom<u8> for ConnectorPartnerType {
    type Error = InvalidConnectorPartnerType;

    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0x1 => Ok(ConnectorPartnerType::DfpAttached),
            0x2 => Ok(ConnectorPartnerType::UfpAttached),
            0x3 => Ok(ConnectorPartnerType::PoweredCableNoUfp),
            0x4 => Ok(ConnectorPartnerType::PoweredCableUfp),
            0x5 => Ok(ConnectorPartnerType::DebugAccessory),
            0x6 => Ok(ConnectorPartnerType::AudioAdapterAccessory),
            _ => Err(InvalidConnectorPartnerType(val)),
        }
    }
}

/// Battery Charging Capability Status
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum BatteryChargingCapabilityStatus {
    /// Not charging
    #[default]
    NotCharging = 0x0,
    /// Nominal charging
    Nominal = 0x1,
    /// Slow charging
    Slow = 0x2,
    /// Very slow charging
    VerySlow = 0x3,
}

/// Invalid Battery Charging Capability Status error, contains the raw value that failed to decode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct InvalidBatteryChargingCapabilityStatus(pub u8);

impl TryFrom<u8> for BatteryChargingCapabilityStatus {
    type Error = InvalidBatteryChargingCapabilityStatus;

    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0x0 => Ok(BatteryChargingCapabilityStatus::NotCharging),
            0x1 => Ok(BatteryChargingCapabilityStatus::Nominal),
            0x2 => Ok(BatteryChargingCapabilityStatus::Slow),
            0x3 => Ok(BatteryChargingCapabilityStatus::VerySlow),
            _ => Err(InvalidBatteryChargingCapabilityStatus(val)),
        }
    }
}

bitfield! {
    /// Provider Capabilities Limited Reason
    #[derive(Copy, Clone, Default, PartialEq, Eq)]
    pub struct ProviderCapsLimitedReasonRaw(u8);
    impl Debug;
    /// Power budget lowered
    pub bool, power_budget_lowered, set_power_budget_lowered: 0;
    /// Reaching power budget limit
    pub bool, reaching_power_budget_limit, set_reaching_power_budget_limit: 1;
}

#[cfg(feature = "defmt")]
impl defmt::Format for ProviderCapsLimitedReasonRaw {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "ProviderCapsLimitedReasonRaw {{ .0: {}, power_budget_lowered: {}, reaching_power_budget_limit: {} }}",
            self.0,
            self.power_budget_lowered(),
            self.reaching_power_budget_limit()
        )
    }
}

/// Reason for limited provider capabilities
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ProviderCapsLimitedReason {
    /// Power budget lowered
    pub power_budget_lowered: bool,
    /// Reaching power budget limit
    pub reaching_power_budget_limit: bool,
}

impl From<ProviderCapsLimitedReasonRaw> for ProviderCapsLimitedReason {
    fn from(raw: ProviderCapsLimitedReasonRaw) -> Self {
        Self {
            power_budget_lowered: raw.power_budget_lowered(),
            reaching_power_budget_limit: raw.reaching_power_budget_limit(),
        }
    }
}

impl From<ProviderCapsLimitedReason> for ProviderCapsLimitedReasonRaw {
    fn from(reason: ProviderCapsLimitedReason) -> Self {
        let mut raw = ProviderCapsLimitedReasonRaw::default();
        raw.set_power_budget_lowered(reason.power_budget_lowered);
        raw.set_reaching_power_budget_limit(reason.reaching_power_budget_limit);
        raw
    }
}

impl From<u8> for ProviderCapsLimitedReason {
    fn from(raw: u8) -> Self {
        ProviderCapsLimitedReasonRaw(raw).into()
    }
}

impl From<ProviderCapsLimitedReason> for u8 {
    fn from(reason: ProviderCapsLimitedReason) -> Self {
        ProviderCapsLimitedReasonRaw::from(reason).0
    }
}

bitfield! {
    /// Raw response data bitfield
    #[derive(Copy, Clone, Default, PartialEq, Eq)]
    pub struct ResponseBitsRaw([u8]);
    impl Debug;

    // Connector Status Change
    pub u16, status_change, set_status_change: 15, 0;
    // Power Operation Mode
    pub u8, power_op_mode, set_power_op_mode: 18, 16;
    // Connect Status
    pub bool, connect_status, set_connect_status: 19;
    // Power Direction
    pub bool, power_direction, set_power_direction: 20;
    // Connector Partner Flags
    pub u8, partner_flags, set_partner_flags: 28, 21;
    // Connector Partner Type
    pub u8, partner_type, set_partner_type: 31, 29;
    // Request Data Object
    pub u32, rdo, set_rdo: 63, 32;
    // Battery Charging Capability Status
    pub u8, battery_charging_status, set_battery_charging_status: 65, 64;
    // Reason for limited provider capabilities
    pub u8, provider_caps_limited, set_provider_caps_limited: 69, 66;
    // bcdPDVersion Operation Mode
    pub u16, bcd_pd_version, set_bcd_pd_version: 85, 70;
}

#[cfg(feature = "defmt")]
impl defmt::Format for ResponseBitsRaw<[u8; RESPONSE_DATA_LEN]> {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "ResponseBitsRaw {{ .0: {}\
                status_change: {}, \
                power_op_mode: {}, \
                connect_status: {}, \
                power_direction: {}, \
                partner_flags: {}, \
                partner_type: {}, \
                rdo: {}, \
                battery_charging_status: {}, \
                provider_caps_limited: {}, \
                bcd_pd_version: {} \
            }}",
            self.0,
            self.status_change(),
            self.power_op_mode(),
            self.connect_status(),
            self.power_direction(),
            self.partner_flags(),
            self.partner_type(),
            self.rdo(),
            self.battery_charging_status(),
            self.provider_caps_limited(),
            self.bcd_pd_version()
        )
    }
}

/// All fields that are only valid when connect_status is true
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ConnectedStatus {
    /// Power operation mode
    pub power_op_mode: PowerOperationMode,
    /// Power direction
    pub power_direction: PowerRole,
    /// Connector partner flags
    pub partner_flags: ConnectorPartnerFlags,
    /// Connector partner type
    pub partner_type: ConnectorPartnerType,
    /// Raw RDO, only valid when operating in PD mode
    ///
    /// An RDO does not contain its type so we can only store the raw value here.
    pub rdo: Option<u32>,
    /// Battery charging capability status, only valid when operating as a sink
    pub battery_charging_status: Option<BatteryChargingCapabilityStatus>,
    /// Reason for limited provider capability
    pub provider_caps_limited: Option<ProviderCapsLimitedReason>,
    /// BCD PD version, only valid when operating in PD mode
    pub bcd_pd_version: Option<u16>,
}

/// Main GET_CONNECTOR_STATUS response data structure
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ResponseData {
    /// Connector status change bitmap
    pub status_change: ConnectorStatusChange,
    /// True if connected
    pub connect_status: bool,
    /// Status only valid when connected
    pub status: Option<ConnectedStatus>,
}

/// Error returned when the raw response data cannot be decoded
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum InvalidResponseData {
    /// Invalid power operation mode
    InvalidPowerOperationMode(InvalidPowerOperationMode),
    /// Invalid connector partner type
    InvalidConnectorPartnerType(InvalidConnectorPartnerType),
    /// Invalid battery charging capability status
    InvalidBatteryChargingCapabilityStatus(InvalidBatteryChargingCapabilityStatus),
}

/// Raw wire format of [`ResponseData`]
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Zeroable, Pod)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ResponseDataRaw {
    /// Response data bits, see [`ResponseBitsRaw`]
    pub bits: [u8; RESPONSE_DATA_LEN],
}

impl ResponseDataRaw {
    /// Length of the raw response data in bytes
    pub const LEN: usize = size_of::<Self>();
}

impl TryFrom<ResponseDataRaw> for ResponseData {
    type Error = InvalidResponseData;

    fn try_from(data: ResponseDataRaw) -> Result<Self, Self::Error> {
        let raw = ResponseBitsRaw(data.bits);

        let status_change = ConnectorStatusChange::from(raw.status_change());
        let connect_status = raw.connect_status();

        // Get connected status if connected
        let status = if connect_status {
            let power_op_mode = PowerOperationMode::try_from(raw.power_op_mode())
                .map_err(InvalidResponseData::InvalidPowerOperationMode)?;
            let power_direction = if raw.power_direction() {
                PowerRole::Source
            } else {
                PowerRole::Sink
            };
            let partner_flags = ConnectorPartnerFlags::from(raw.partner_flags());
            let partner_type = ConnectorPartnerType::try_from(raw.partner_type())
                .map_err(InvalidResponseData::InvalidConnectorPartnerType)?;
            let rdo = if connect_status && power_op_mode == PowerOperationMode::Pd && raw.rdo() != 0 {
                Some(raw.rdo())
            } else {
                None
            };

            // Battery charging status is only valid when operating as a sink
            let battery_charging_status = if power_direction == PowerRole::Sink {
                Some(
                    BatteryChargingCapabilityStatus::try_from(raw.battery_charging_status())
                        .map_err(InvalidResponseData::InvalidBatteryChargingCapabilityStatus)?,
                )
            } else {
                None
            };

            let provider_caps_limited = if raw.provider_caps_limited() != 0 {
                Some(ProviderCapsLimitedReason::from(raw.provider_caps_limited()))
            } else {
                None
            };

            let bcd_pd_version = if connect_status && power_op_mode == PowerOperationMode::Pd {
                Some(raw.bcd_pd_version())
            } else {
                None
            };

            Some(ConnectedStatus {
                power_op_mode,
                power_direction,
                partner_flags,
                partner_type,
                rdo,
                battery_charging_status,
                provider_caps_limited,
                bcd_pd_version,
            })
        } else {
            None
        };

        Ok(ResponseData {
            status_change,
            connect_status,
            status,
        })
    }
}

impl From<ResponseData> for ResponseDataRaw {
    fn from(data: ResponseData) -> Self {
        let mut raw = ResponseBitsRaw([0; RESPONSE_DATA_LEN]);

        raw.set_status_change(data.status_change.into());
        raw.set_connect_status(data.connect_status);

        if let Some(status) = data.status {
            raw.set_power_op_mode(status.power_op_mode as u8);
            raw.set_power_direction(status.power_direction == PowerRole::Source);
            raw.set_partner_flags(status.partner_flags.into());
            raw.set_partner_type(status.partner_type as u8);

            // Note: Can be collapsed to a let chain when this crate is updated to 2024 edition
            if let Some(rdo) = status.rdo {
                if rdo != 0 {
                    raw.set_rdo(rdo)
                }
            }

            if let Some(battery_charging_status) = status.battery_charging_status {
                raw.set_battery_charging_status(battery_charging_status as u8);
            }

            if let Some(provider_caps_limited) = status.provider_caps_limited {
                raw.set_provider_caps_limited(provider_caps_limited.into());
            }

            if let Some(bcd_pd_version) = status.bcd_pd_version {
                raw.set_bcd_pd_version(bcd_pd_version);
            }
        }

        Self { bits: raw.0 }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::GlobalPortId;

    /// Create standard response data for testing
    pub fn create_response_data() -> (ResponseData, [u8; RESPONSE_DATA_LEN]) {
        let response_data = ResponseData {
            status_change: ConnectorStatusChange::from(0x8002),
            connect_status: true,
            status: Some(ConnectedStatus {
                power_op_mode: PowerOperationMode::Pd,
                power_direction: PowerRole::Sink,
                partner_flags: ConnectorPartnerFlags::from(0x03),
                partner_type: ConnectorPartnerType::DfpAttached,
                rdo: Some(0x78563412),
                battery_charging_status: Some(BatteryChargingCapabilityStatus::Nominal),
                provider_caps_limited: Some(ProviderCapsLimitedReason::from(0x01)),
                bcd_pd_version: Some(0x300),
            }),
        };

        let mut bytes = [0u8; RESPONSE_DATA_LEN];
        // Status changed flags - 2 bytes
        // Set lowest and highest non-reserved bits
        // Corresponds to external supply change + error
        bytes[0] = 0x2;
        bytes[1] = 0x80;

        // Various status flags - 1 byte
        // power operation mode = PD
        // Connect status = 1
        // Power direction = 0 (consumer)
        bytes[2] = 0x0b;

        // Connector partner flags - 1 byte
        // DFP, usb + alt_mode set (lower 2 bits of partner_flags = bits 21,22 = byte 2 bits 5,6)
        bytes[2] |= 0x60;
        bytes[3] = 0x20;

        // RDO - 4 bytes
        // Probably not a valid RDO, but we only have the raw value because an RDO needs
        // the corresponding PDO to be decoded
        bytes[4] = 0x12;
        bytes[5] = 0x34;
        bytes[6] = 0x56;
        bytes[7] = 0x78;

        // More status flags + lower 2 bits of bcdPDVersion - 1 byte
        // Battery charging status - nominal, provider power level lowered, version 3.0
        bytes[8] = 0x05;

        // Bits 2 through 10 of bcdPDVersion - 1 byte
        // PD version 3.00
        bytes[9] = 0xC0;

        (response_data, bytes)
    }

    #[test]
    fn test_raw_len() {
        assert_eq!(ArgsRaw::LEN, COMMAND_LEN - size_of::<CommandHeaderRaw>());
        assert_eq!(ResponseDataRaw::LEN, RESPONSE_DATA_LEN);
    }

    #[test]
    fn test_args_raw_roundtrip() {
        let encoded: [u8; ArgsRaw::LEN] = [0x03, 0x00, 0x00, 0x00, 0x00, 0x00];
        let args: Args<GlobalPortId> = Args {
            connector_number: GlobalPortId(3),
        };

        assert_eq!(
            bytemuck::must_cast::<_, [u8; ArgsRaw::LEN]>(ArgsRaw::from(args)),
            encoded
        );
        assert_eq!(Args::from(bytemuck::must_cast::<_, ArgsRaw>(encoded)), args);
    }

    #[test]
    fn test_response_data_roundtrip() {
        let (expected, bytes) = create_response_data();

        let raw = bytemuck::must_cast::<_, ResponseDataRaw>(bytes);
        assert_eq!(ResponseData::try_from(raw), Ok(expected));

        let encoded: [u8; RESPONSE_DATA_LEN] = bytemuck::must_cast(ResponseDataRaw::from(expected));
        assert_eq!(encoded, bytes);
    }
}

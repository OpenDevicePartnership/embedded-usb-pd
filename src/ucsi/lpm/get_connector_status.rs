//! Types for GET_CONNECTOR_STATUS command, see UCSI spec 6.6

use bincode::de::{Decode, Decoder};
use bincode::enc::{Encode, Encoder};
use bincode::error::{AllowedEnumVariants, DecodeError, EncodeError};
use bitfield::bitfield;

use crate::ucsi::ppm::set_notification_enable::NotificationEnable;
use crate::ucsi::{CommandHeaderRaw, COMMAND_LEN};
use crate::PowerRole;

/// Data length for the GET_CONNECTOR_STATUS command response
pub const RESPONSE_DATA_LEN: usize = 11;
/// Command padding, -1 for the connector number byte
pub const COMMAND_PADDING: usize = COMMAND_LEN - size_of::<CommandHeaderRaw>() - 1;

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

/// Higher-level wrapper around [`ConnectorStatusChangeRaw`]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ConnectorStatusChange(ConnectorStatusChangeRaw);

impl ConnectorStatusChange {
    /// Returns the external supply change flag
    pub fn external_supply_change(&self) -> bool {
        self.0.external_supply_change()
    }

    /// Sets the external supply change flag
    pub fn set_external_supply_change(&mut self, value: bool) {
        self.0.set_external_supply_change(value);
    }

    /// Returns the power operation mode change flag
    pub fn power_op_mode_change(&self) -> bool {
        self.0.power_op_mode_change()
    }

    /// Sets the power operation mode change flag
    pub fn set_power_op_mode_change(&mut self, value: bool) {
        self.0.set_power_op_mode_change(value);
    }

    /// Returns the provider capabilities change flag
    pub fn provider_caps_change(&self) -> bool {
        self.0.provider_caps_change()
    }

    /// Sets the provider capabilities change flag
    pub fn set_provider_caps_change(&mut self, value: bool) {
        self.0.set_provider_caps_change(value);
    }

    /// Returns the negotiated power level change flag
    pub fn negotiated_power_level_change(&self) -> bool {
        self.0.negotiated_power_level_change()
    }

    /// Sets the negotiated power level change flag
    pub fn set_negotiated_power_level_change(&mut self, value: bool) {
        self.0.set_negotiated_power_level_change(value);
    }

    /// Returns the PD reset complete flag
    pub fn pd_reset_complete(&self) -> bool {
        self.0.pd_reset_complete()
    }

    /// Sets the PD reset complete flag
    pub fn set_pd_reset_complete(&mut self, value: bool) {
        self.0.set_pd_reset_complete(value);
    }

    /// Returns the supported CAM change flag
    pub fn supported_cam_change(&self) -> bool {
        self.0.supported_cam_change()
    }

    /// Sets the supported CAM change flag
    pub fn set_supported_cam_change(&mut self, value: bool) {
        self.0.set_supported_cam_change(value);
    }

    /// Returns the battery charging status change flag
    pub fn battery_charging_status_change(&self) -> bool {
        self.0.battery_charging_status_change()
    }

    /// Sets the battery charging status change flag
    pub fn set_battery_charging_status_change(&mut self, value: bool) {
        self.0.set_battery_charging_status_change(value);
    }

    /// Returns the connector partner changed flag
    pub fn connector_partner_changed(&self) -> bool {
        self.0.connector_partner_changed()
    }

    /// Sets the connector partner changed flag
    pub fn set_connector_partner_changed(&mut self, value: bool) {
        self.0.set_connector_partner_changed(value);
    }

    /// Returns the power direction changed flag
    pub fn power_direction_changed(&self) -> bool {
        self.0.power_direction_changed()
    }

    /// Sets the power direction changed flag
    pub fn set_power_direction_changed(&mut self, value: bool) {
        self.0.set_power_direction_changed(value);
    }

    /// Returns the connect/disconnect change flag
    pub fn connect_change(&self) -> bool {
        self.0.connect_change()
    }

    /// Sets the connect/disconnect change flag
    pub fn set_connect_change(&mut self, value: bool) {
        self.0.set_connect_change(value);
    }

    /// Returns the error flag
    pub fn error(&self) -> bool {
        self.0.error()
    }

    /// Sets the error flag
    pub fn set_error(&mut self, value: bool) {
        self.0.set_error(value);
    }

    /// Returns true if no status change flags are set
    pub fn is_empty(&self) -> bool {
        self.0 .0 == 0
    }

    /// Returns true if any status change flags are set
    pub fn any(&self) -> bool {
        !self.is_empty()
    }

    /// Returns a new connector status change with all flags that match the given notification enable flags
    pub fn filter_enabled(&self, enable: NotificationEnable) -> Self {
        // These bitfields have the same layout
        let connector_raw = self.0 .0;
        let enable_raw: u16 = enable.into();
        ConnectorStatusChange(ConnectorStatusChangeRaw(connector_raw & enable_raw))
    }
}

impl From<u16> for ConnectorStatusChange {
    fn from(raw: u16) -> Self {
        ConnectorStatusChange(ConnectorStatusChangeRaw(raw))
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

impl From<InvalidPowerOperationMode> for DecodeError {
    fn from(val: InvalidPowerOperationMode) -> Self {
        DecodeError::UnexpectedVariant {
            type_name: "PowerOperationMode",
            found: val.0 as u32,
            allowed: &AllowedEnumVariants::Allowed(&[
                PowerOperationMode::UsbDefault as u32,
                PowerOperationMode::Bc as u32,
                PowerOperationMode::Pd as u32,
                PowerOperationMode::TypeC1_5A as u32,
                PowerOperationMode::TypeC3A as u32,
                PowerOperationMode::TypeC5A as u32,
            ]),
        }
    }
}

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
pub struct ConnectorPartnerFlags(ConnectorPartnerFlagsRaw);

impl ConnectorPartnerFlags {
    /// Get usb flag
    pub fn usb(&self) -> bool {
        self.0.usb()
    }

    /// Set usb flag
    pub fn set_usb(&mut self, value: bool) {
        self.0.set_usb(value);
    }

    /// Get alternate mode flag
    pub fn alt_mode(&self) -> bool {
        self.0.alt_mode()
    }

    /// Set alternate mode flag
    pub fn set_alt_mode(&mut self, value: bool) {
        self.0.set_alt_mode(value);
    }
}

impl From<u8> for ConnectorPartnerFlags {
    fn from(value: u8) -> Self {
        ConnectorPartnerFlags(ConnectorPartnerFlagsRaw(value))
    }
}

impl From<ConnectorPartnerFlags> for u8 {
    fn from(flags: ConnectorPartnerFlags) -> Self {
        flags.0 .0
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

impl From<InvalidConnectorPartnerType> for DecodeError {
    fn from(val: InvalidConnectorPartnerType) -> Self {
        DecodeError::UnexpectedVariant {
            type_name: "ConnectorPartnerType",
            found: val.0 as u32,
            allowed: &AllowedEnumVariants::Allowed(&[
                ConnectorPartnerType::DfpAttached as u32,
                ConnectorPartnerType::UfpAttached as u32,
                ConnectorPartnerType::PoweredCableNoUfp as u32,
                ConnectorPartnerType::PoweredCableUfp as u32,
                ConnectorPartnerType::DebugAccessory as u32,
                ConnectorPartnerType::AudioAdapterAccessory as u32,
            ]),
        }
    }
}

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

impl From<InvalidBatteryChargingCapabilityStatus> for DecodeError {
    fn from(val: InvalidBatteryChargingCapabilityStatus) -> Self {
        DecodeError::UnexpectedVariant {
            type_name: "BatteryChargingCapabilityStatus",
            found: val.0 as u32,
            allowed: &AllowedEnumVariants::Allowed(&[
                BatteryChargingCapabilityStatus::NotCharging as u32,
                BatteryChargingCapabilityStatus::Nominal as u32,
                BatteryChargingCapabilityStatus::Slow as u32,
                BatteryChargingCapabilityStatus::VerySlow as u32,
            ]),
        }
    }
}

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

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ProviderCapsLimitedReason(ProviderCapsLimitedReasonRaw);

impl ProviderCapsLimitedReason {
    /// Returns if power budget is lowered
    pub fn power_budget_lowered(&self) -> bool {
        self.0.power_budget_lowered()
    }

    /// Sets power budget lowered status
    pub fn set_power_budget_lowered(&mut self, lowered: bool) -> &mut Self {
        self.0.set_power_budget_lowered(lowered);
        self
    }

    /// Returns if reaching power budget limit
    pub fn reaching_power_budget_limit(&self) -> bool {
        self.0.reaching_power_budget_limit()
    }

    /// Sets reaching power budget limit status
    pub fn set_reaching_power_budget_limit(&mut self, limit: bool) -> &mut Self {
        self.0.set_reaching_power_budget_limit(limit);
        self
    }
}

impl From<u8> for ProviderCapsLimitedReason {
    fn from(raw: u8) -> Self {
        ProviderCapsLimitedReason(ProviderCapsLimitedReasonRaw(raw))
    }
}

bitfield! {
    /// Raw response data bitfield
    #[derive(Copy, Clone, Default)]
    pub struct ResponseDataRaw([u8]);
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
impl defmt::Format for ResponseDataRaw<[u8; RESPONSE_DATA_LEN]> {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "ResponseDataRaw {{ .0: {}\
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

pub enum InvalidResponseData {
    /// Invalid power operation mode
    InvalidPowerOperationMode(InvalidPowerOperationMode),
    /// Invalid connector partner type
    InvalidConnectorPartnerType(InvalidConnectorPartnerType),
    /// Invalid battery charging capability status
    InvalidBatteryChargingCapabilityStatus(InvalidBatteryChargingCapabilityStatus),
}

impl From<InvalidResponseData> for DecodeError {
    fn from(err: InvalidResponseData) -> Self {
        match err {
            InvalidResponseData::InvalidPowerOperationMode(e) => e.into(),
            InvalidResponseData::InvalidConnectorPartnerType(e) => e.into(),
            InvalidResponseData::InvalidBatteryChargingCapabilityStatus(e) => e.into(),
        }
    }
}

impl TryFrom<[u8; RESPONSE_DATA_LEN]> for ResponseData {
    type Error = InvalidResponseData;

    fn try_from(data: [u8; RESPONSE_DATA_LEN]) -> Result<Self, Self::Error> {
        let raw = ResponseDataRaw(data);

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

impl From<ResponseData> for [u8; RESPONSE_DATA_LEN] {
    fn from(data: ResponseData) -> Self {
        let mut raw = ResponseDataRaw([0; RESPONSE_DATA_LEN]);

        raw.set_status_change(data.status_change.0 .0);
        raw.set_connect_status(data.connect_status);

        if let Some(status) = data.status {
            raw.set_power_op_mode(status.power_op_mode as u8);
            raw.set_power_direction(status.power_direction == PowerRole::Source);
            raw.set_partner_flags(status.partner_flags.into());
            raw.set_partner_type(status.partner_type as u8);

            if status.rdo.is_some_and(|rdo| rdo != 0) {
                raw.set_rdo(status.rdo.unwrap());
            }

            if let Some(battery_charging_status) = status.battery_charging_status {
                raw.set_battery_charging_status(battery_charging_status as u8);
            }

            if let Some(provider_caps_limited) = status.provider_caps_limited {
                raw.set_provider_caps_limited(provider_caps_limited.0 .0);
            }

            if let Some(bcd_pd_version) = status.bcd_pd_version {
                raw.set_bcd_pd_version(bcd_pd_version);
            }
        }
        raw.0
    }
}

impl Encode for ResponseData {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        <[u8; RESPONSE_DATA_LEN]>::from(*self).encode(encoder)
    }
}

impl<Context> Decode<Context> for ResponseData {
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, DecodeError> {
        let raw = <[u8; RESPONSE_DATA_LEN]>::decode(decoder)?;
        let data = ResponseData::try_from(raw)?;
        Ok(data)
    }
}
/// GET_CONNECTOR_STATUS command arguments
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Args;

impl Encode for Args {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        // Padding to fill the command length
        [0u8; COMMAND_PADDING].encode(encoder)
    }
}

impl<Context> Decode<Context> for Args {
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, DecodeError> {
        // Read padding
        let _padding: [u8; COMMAND_PADDING] = Decode::decode(decoder)?;
        Ok(Self)
    }
}

#[cfg(test)]
pub mod test {
    use bincode::config::standard;
    use bincode::{decode_from_slice, encode_into_slice};

    use super::*;

    /// Create standard response data for testing
    pub fn create_response_data() -> (ResponseData, [u8; RESPONSE_DATA_LEN]) {
        let response_data = ResponseData {
            status_change: ConnectorStatusChange::from(0x8001),
            connect_status: true,
            status: Some(ConnectedStatus {
                power_op_mode: PowerOperationMode::Pd,
                power_direction: PowerRole::Sink,
                partner_flags: ConnectorPartnerFlags::from(0),
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
        bytes[0] = 0x1;
        bytes[1] = 0x80;

        // Various status flags - 1 byte
        // power operation mode = PD
        // Connect status = 1
        // Power direction = 0 (consumer)
        bytes[2] = 0x0b;

        // Connector partner flags - 1 byte
        // DFP
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
    fn test_decode_response_data() {
        let (expected, bytes) = create_response_data();

        let (response_data, consumed): (ResponseData, usize) =
            decode_from_slice(&bytes, standard().with_fixed_int_encoding()).unwrap();

        assert_eq!(consumed, bytes.len());
        assert_eq!(response_data, expected);

        let mut encoded_bytes = [0u8; RESPONSE_DATA_LEN];
        let len = encode_into_slice(expected, &mut encoded_bytes, standard().with_fixed_int_encoding()).unwrap();

        assert_eq!(len, RESPONSE_DATA_LEN);
        assert_eq!(encoded_bytes, bytes);
    }
}

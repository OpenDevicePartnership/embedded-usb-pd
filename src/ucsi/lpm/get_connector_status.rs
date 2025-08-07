//! Types for GET_CONNECTOR_STATUS command, see UCSI spec 6.6

use bincode::de::{Decode, Decoder};
use bincode::enc::{Encode, Encoder};
use bincode::error::{AllowedEnumVariants, DecodeError, EncodeError};
use bitfield::bitfield;

use crate::pdo::{MA5_UNIT, MV5_UNIT};
use crate::{PlugOrientation, PowerRole};

/// Data length for the GET_CONNECTOR_STATUS command response
pub const RESPONSE_DATA_LEN: usize = 19;

bitfield! {
    /// Connector Status Change bitmap
    #[derive(Copy, Default, Clone)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub struct ConnectorStatusChangeRaw(u16);
    impl Debug;
    /// External supply change
    pub bool, external_supply_change, set_external_supply_change: 1;
    /// Power operation mode change
    pub bool, power_op_mode_change, set_power_op_mode_change: 2;
    /// Attention received from port partner
    pub bool, attention, set_attention: 3;
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
    /// Sink path status change
    pub bool, sink_path_status_change, set_sink_path_status_change: 13;
    /// Connect/disconnect
    pub bool, connect_change, set_connect_change: 14;
    /// Error
    pub bool, error, set_error: 15;
}

/// Higher-level wrapper around [`ConnectorStatusChangeRaw`]
#[derive(Copy, Clone, Debug, Default)]
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

    /// Returns the attention flag
    pub fn attention(&self) -> bool {
        self.0.attention()
    }

    /// Sets the attention flag
    pub fn set_attention(&mut self, value: bool) {
        self.0.set_attention(value);
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

    /// Returns the sink path status change flag
    pub fn sink_path_status_change(&self) -> bool {
        self.0.sink_path_status_change()
    }

    /// Sets the sink path status change flag
    pub fn set_sink_path_status_change(&mut self, value: bool) {
        self.0.set_sink_path_status_change(value);
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

/// Connector Partner Flags
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ConnectorPartnerFlags {
    /// USB2.x or USB3.x
    #[default]
    Usb = 0x0,
    /// Alternate mode
    AltMode = 0x1,
    /// USB4 Gen3
    Usb4Gen3 = 0x2,
    /// USB4 Gen4
    Usb4Gen4 = 0x3,
}

/// Invalid Connector Partner Flags error, contains the raw value that failed to decode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct InvalidConnectorPartnerFlags(pub u8);

impl From<InvalidConnectorPartnerFlags> for DecodeError {
    fn from(val: InvalidConnectorPartnerFlags) -> Self {
        DecodeError::UnexpectedVariant {
            type_name: "ConnectorPartnerFlags",
            found: val.0 as u32,
            allowed: &AllowedEnumVariants::Allowed(&[
                ConnectorPartnerFlags::Usb as u32,
                ConnectorPartnerFlags::AltMode as u32,
                ConnectorPartnerFlags::Usb4Gen3 as u32,
                ConnectorPartnerFlags::Usb4Gen4 as u32,
            ]),
        }
    }
}

impl TryFrom<u8> for ConnectorPartnerFlags {
    type Error = InvalidConnectorPartnerFlags;

    fn try_from(raw: u8) -> Result<Self, Self::Error> {
        match raw {
            0x0 => Ok(ConnectorPartnerFlags::Usb),
            0x1 => Ok(ConnectorPartnerFlags::AltMode),
            0x2 => Ok(ConnectorPartnerFlags::Usb4Gen3),
            0x3 => Ok(ConnectorPartnerFlags::Usb4Gen4),
            _ => Err(InvalidConnectorPartnerFlags(raw)),
        }
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
    #[derive(Copy, Clone, Default)]
    pub struct ProviderCapsLimitedReasonRaw(u8);
    impl Debug;
    /// Power budget lowered
    pub bool, power_budget_lowered, set_power_budget_lowered: 0;
    /// Reaching power budget limit
    pub bool, reaching_power_budget_limit, set_reaching_power_budget_limit: 1;
}

#[derive(Copy, Clone, Debug, Default)]
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
    // Provider Capabilities Limited Reason
    pub u8, provider_caps_limited_reason, set_provider_caps_limited_reason: 69, 66;
    // bcdPDVersion Operation Mode
    pub u16, bcd_pd_version, set_bcd_pd_version: 85, 70;
    // Orientation
    pub bool, orientation, set_orientation: 86;
    // Sink Path Status
    pub bool, sink_path_status, set_sink_path_status: 87;
    // Reverse Current Protection Status
    pub bool, reverse_current_protection_status, set_reverse_current_protection_status: 88;
    // Power Reading Ready
    pub bool, power_reading_ready, set_power_reading_ready: 89;
    // Current Scale
    pub u8, current_scale, set_current_scale: 92, 90;
    // Peak Current
    pub u16, peak_current, set_peak_current: 108, 93;
    // Average Current
    pub u16, avg_current, set_avg_current: 124, 109;
    // Voltage Scale
    pub u8, voltage_scale, set_voltage_scale: 128, 125;
    // Voltage Reading
    pub u16, voltage_reading, set_voltage_reading: 144, 129;
}

/// All fields that are only valid when connect_status is true
#[derive(Copy, Clone, Debug, Default)]
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
    /// BCD PD version, only valid when operating in PD mode
    pub bcd_pd_version: Option<u16>,
    /// Orientation
    pub orientation: PlugOrientation,
    /// Sink path status
    pub sink_path_status: bool,
}

/// Power reading result
#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct PowerReading {
    /// Current scale
    pub scale_ma: u16,
    /// Peak current
    pub peak_current_ma: u16,
    /// Average current
    pub avg_current_ma: u16,
    /// Voltage scale
    pub scale_mv: u16,
    /// Voltage reading
    pub voltage_reading_mv: u16,
}

/// Main GET_CONNECTOR_STATUS response data structure
#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ResponseData {
    /// Connector status change bitmap
    pub status_change: ConnectorStatusChange,
    /// True if connected
    pub connect_status: bool,
    /// Status only valid when connected
    pub status: Option<ConnectedStatus>,
    /// Debug info, LPM will set this if this feature is supported
    pub reverse_current_protection_status: bool,
    /// Power reading
    pub power_reading: Option<PowerReading>,
}

pub enum InvalidResponseData {
    /// Invalid power operation mode
    InvalidPowerOperationMode(InvalidPowerOperationMode),
    /// Invalid connector partner flags
    InvalidConnectorPartnerFlags(InvalidConnectorPartnerFlags),
    /// Invalid connector partner type
    InvalidConnectorPartnerType(InvalidConnectorPartnerType),
    /// Invalid battery charging capability status
    InvalidBatteryChargingCapabilityStatus(InvalidBatteryChargingCapabilityStatus),
}

impl From<InvalidResponseData> for DecodeError {
    fn from(err: InvalidResponseData) -> Self {
        match err {
            InvalidResponseData::InvalidPowerOperationMode(e) => e.into(),
            InvalidResponseData::InvalidConnectorPartnerFlags(e) => e.into(),
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
            let partner_flags = ConnectorPartnerFlags::try_from(raw.partner_flags())
                .map_err(InvalidResponseData::InvalidConnectorPartnerFlags)?;
            let partner_type = ConnectorPartnerType::try_from(raw.partner_type())
                .map_err(InvalidResponseData::InvalidConnectorPartnerType)?;
            let rdo = if connect_status && power_op_mode == PowerOperationMode::Pd {
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

            let bcd_pd_version = if connect_status && power_op_mode == PowerOperationMode::Pd {
                Some(raw.bcd_pd_version())
            } else {
                None
            };

            let orientation = if raw.orientation() {
                PlugOrientation::CC2
            } else {
                PlugOrientation::CC1
            };

            let sink_path_status = raw.sink_path_status();

            Some(ConnectedStatus {
                power_op_mode,
                power_direction,
                partner_flags,
                partner_type,
                rdo,
                battery_charging_status,
                bcd_pd_version,
                orientation,
                sink_path_status,
            })
        } else {
            None
        };

        let reverse_current_protection_status = raw.reverse_current_protection_status();

        // Get power reading if available
        let power_reading = if raw.power_reading_ready() {
            let current_scale = raw.current_scale() as u16 * MA5_UNIT;
            let voltage_scale = raw.voltage_scale() as u16 * MV5_UNIT;

            Some(PowerReading {
                scale_ma: current_scale,
                peak_current_ma: raw.peak_current() * current_scale,
                avg_current_ma: raw.avg_current() * current_scale,
                scale_mv: voltage_scale,
                voltage_reading_mv: raw.voltage_reading() * voltage_scale,
            })
        } else {
            None
        };

        Ok(ResponseData {
            status_change,
            connect_status,
            status,
            reverse_current_protection_status,
            power_reading,
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
            raw.set_partner_flags(status.partner_flags as u8);
            raw.set_partner_type(status.partner_type as u8);
            if let Some(rdo) = status.rdo {
                raw.set_rdo(rdo);
            }
            if let Some(battery_charging_status) = status.battery_charging_status {
                raw.set_battery_charging_status(battery_charging_status as u8);
            }
            if let Some(bcd_pd_version) = status.bcd_pd_version {
                raw.set_bcd_pd_version(bcd_pd_version);
            }
            raw.set_orientation(status.orientation == PlugOrientation::CC2);
            raw.set_sink_path_status(status.sink_path_status);
        }

        raw.set_reverse_current_protection_status(data.reverse_current_protection_status);

        if let Some(power_reading) = data.power_reading {
            raw.set_power_reading_ready(true);
            raw.set_current_scale((power_reading.scale_ma / MA5_UNIT) as u8);
            raw.set_peak_current(power_reading.peak_current_ma / power_reading.scale_ma);
            raw.set_avg_current(power_reading.avg_current_ma / power_reading.scale_ma);
            raw.set_voltage_scale((power_reading.scale_mv / MV5_UNIT) as u8);
            raw.set_voltage_reading(power_reading.voltage_reading_mv / power_reading.scale_mv);
        } else {
            raw.set_power_reading_ready(false);
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

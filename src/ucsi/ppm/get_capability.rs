//! Types for the `GetCapability` command, see USCI spec 6.5

use bincode::de::{Decode, Decoder};
use bincode::enc::{Encode, Encoder};
use bincode::error::{DecodeError, EncodeError};
use bitfield::bitfield;

use crate::ucsi::{CommandHeaderRaw, COMMAND_LEN};

/// Data length for the GET_CAPABILITY command response
pub const RESPONSE_DATA_LEN: usize = 16;
/// Command padding
pub const COMMAND_PADDING: usize = COMMAND_LEN - size_of::<CommandHeaderRaw>();

/// GetCapability command
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

bitfield! {
    /// Optional features bitmap for GetCapability command
    #[derive(Copy, Clone, PartialEq, Eq)]
    struct OptionalFeaturesRaw(u32);
    impl Debug;

    /// Supports SET_CCOM
    pub bool, set_ccom_supported, set_set_ccom_supported: 0;
    /// Supports SET_POWER_LEVEL
    pub bool, set_power_level_supported, set_set_power_level_supported: 1;
    /// Supports alternate mode details
    pub bool, altmode_details_supported, set_altmode_details_supported: 2;
    /// Supports alternate mode override
    pub bool, altmode_override_supported, set_altmode_override_supported: 3;
    /// Supports power data object details
    pub bool, pdo_details_supported, set_pdo_details_supported: 4;
    /// Supports cable details
    pub bool, cable_details_supported, set_cable_details_supported: 5;
    /// Supports external supply notification
    pub bool, external_supply_notif_supported, set_external_supply_notif_supported: 6;
    /// Supports PD reset notification
    pub bool, pd_reset_notif_supported, set_pd_reset_notif_supported: 7;
    /// Supports GET_PD_MESSAGE
    pub bool, get_pd_msg_supported, set_get_pd_msg_supported: 8;
}

#[cfg(feature = "defmt")]
impl defmt::Format for OptionalFeaturesRaw {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "OptionalFeaturesRaw {{ .0: {}, set_ccom_supported: {}, set_power_level_supported: {}, altmode_details_supported: {}, altmode_override_supported: {}, pdo_details_supported: {}, cable_details_supported: {}, external_supply_notif_supported: {}, pd_reset_notif_supported: {}, get_pd_msg_supported: {} }}",
            self.0,
            self.set_ccom_supported(),
            self.set_power_level_supported(),
            self.altmode_details_supported(),
            self.altmode_override_supported(),
            self.pdo_details_supported(),
            self.cable_details_supported(),
            self.external_supply_notif_supported(),
            self.pd_reset_notif_supported(),
            self.get_pd_msg_supported()
        )
    }
}

/// Higher-level wrapper around [`OptionalFeaturesRaw`]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct OptionalFeatures(OptionalFeaturesRaw);

impl OptionalFeatures {
    /// Returns whether SET_CCOM is supported
    pub fn set_ccom_supported(&self) -> bool {
        self.0.set_ccom_supported()
    }

    /// Sets whether SET_CCOM is supported
    pub fn set_set_ccom_supported(&mut self, value: bool) -> &mut Self {
        self.0.set_set_ccom_supported(value);
        self
    }

    /// Returns whether SET_POWER_LEVEL is supported
    pub fn set_power_level_supported(&self) -> bool {
        self.0.set_power_level_supported()
    }

    /// Sets whether SET_POWER_LEVEL is supported
    pub fn set_set_power_level_supported(&mut self, value: bool) -> &mut Self {
        self.0.set_set_power_level_supported(value);
        self
    }

    /// Returns whether alternate mode details are supported
    pub fn altmode_details_supported(&self) -> bool {
        self.0.altmode_details_supported()
    }

    /// Sets whether alternate mode details are supported
    pub fn set_altmode_details_supported(&mut self, value: bool) -> &mut Self {
        self.0.set_altmode_details_supported(value);
        self
    }

    /// Returns whether alternate mode override is supported
    pub fn altmode_override_supported(&self) -> bool {
        self.0.altmode_override_supported()
    }

    /// Sets whether alternate mode override is supported
    pub fn set_altmode_override_supported(&mut self, value: bool) -> &mut Self {
        self.0.set_altmode_override_supported(value);
        self
    }

    /// Returns whether PDO details are supported
    pub fn pdo_details_supported(&self) -> bool {
        self.0.pdo_details_supported()
    }

    /// Sets whether PDO details are supported
    pub fn set_pdo_details_supported(&mut self, value: bool) -> &mut Self {
        self.0.set_pdo_details_supported(value);
        self
    }

    /// Returns whether cable details are supported
    pub fn cable_details_supported(&self) -> bool {
        self.0.cable_details_supported()
    }

    /// Sets whether cable details are supported
    pub fn set_cable_details_supported(&mut self, value: bool) -> &mut Self {
        self.0.set_cable_details_supported(value);
        self
    }

    /// Returns whether external supply notification is supported
    pub fn external_supply_notif_supported(&self) -> bool {
        self.0.external_supply_notif_supported()
    }

    /// Sets whether external supply notification is supported
    pub fn set_external_supply_notif_supported(&mut self, value: bool) -> &mut Self {
        self.0.set_external_supply_notif_supported(value);
        self
    }

    /// Returns whether PD reset notification is supported
    pub fn pd_reset_notif_supported(&self) -> bool {
        self.0.pd_reset_notif_supported()
    }

    /// Sets whether PD reset notification is supported
    pub fn set_pd_reset_notif_supported(&mut self, value: bool) -> &mut Self {
        self.0.set_pd_reset_notif_supported(value);
        self
    }

    /// Returns whether GET_PD_MESSAGE is supported
    pub fn get_pd_msg_supported(&self) -> bool {
        self.0.get_pd_msg_supported()
    }

    /// Sets whether GET_PD_MESSAGE is supported
    pub fn set_get_pd_msg_supported(&mut self, value: bool) -> &mut Self {
        self.0.set_get_pd_msg_supported(value);
        self
    }
}

impl From<u32> for OptionalFeatures {
    fn from(raw: u32) -> Self {
        OptionalFeatures(OptionalFeaturesRaw(raw))
    }
}

impl Default for OptionalFeatures {
    fn default() -> Self {
        OptionalFeatures(OptionalFeaturesRaw(0))
    }
}

impl Encode for OptionalFeatures {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        let raw = self.0 .0;
        let lower = (raw & 0xFFFF) as u16;
        let upper = (raw >> 16) as u8;
        lower.encode(encoder)?;
        upper.encode(encoder)?;
        Ok(())
    }
}

impl<Context> Decode<Context> for OptionalFeatures {
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, DecodeError> {
        let lower = u16::decode(decoder)?;
        let upper = u8::decode(decoder)?;
        let raw = ((upper as u32) << 16) | (lower as u32);
        Ok(OptionalFeatures::from(raw))
    }
}

bitfield! {
    /// Raw power source data for GetCapability command
    #[derive(Copy, Clone)]
    struct PowerSourceRaw(u8);
    impl Debug;

    /// AC supply supported
    pub bool, ac_supply, set_ac_supply: 0;
    /// Other supply supported
    pub bool, other, set_other: 2;
    /// Uses VBUS
    pub bool, use_vbus, set_uses_vbus: 6;
}

#[cfg(feature = "defmt")]
impl defmt::Format for PowerSourceRaw {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "PowerSourceRaw {{ .0: {}, ac_supply: {}, other: {}, use_vbus: {} }}",
            self.0,
            self.ac_supply(),
            self.other(),
            self.use_vbus()
        )
    }
}

/// Higher-level wrapper around [`PowerSourceRaw`]
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct PowerSource(PowerSourceRaw);

impl PowerSource {
    /// Returns whether AC supply is supported
    pub fn ac_supply(&self) -> bool {
        self.0.ac_supply()
    }

    /// Set whether AC supply is supported
    pub fn set_ac_supply(&mut self, ac_supply: bool) -> &mut Self {
        self.0.set_ac_supply(ac_supply);
        self
    }

    /// Returns whether other supply is supported
    pub fn other(&self) -> bool {
        self.0.other()
    }

    /// Set whether other supply is supported
    pub fn set_other(&mut self, other: bool) -> &mut Self {
        self.0.set_other(other);
        self
    }

    /// Returns whether VBUS is used
    pub fn use_vbus(&self) -> bool {
        self.0.use_vbus()
    }

    /// Set whether VBUS is used
    pub fn set_use_vbus(&mut self, use_vbus: bool) -> &mut Self {
        self.0.set_uses_vbus(use_vbus);
        self
    }
}

impl From<u8> for PowerSource {
    fn from(raw: u8) -> Self {
        PowerSource(PowerSourceRaw(raw))
    }
}

impl Default for PowerSource {
    fn default() -> Self {
        PowerSource(PowerSourceRaw(0))
    }
}

impl Encode for PowerSource {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.0 .0.encode(encoder)
    }
}

impl<Context> Decode<Context> for PowerSource {
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, DecodeError> {
        let raw = u8::decode(decoder)?;
        Ok(PowerSource::from(raw))
    }
}

bitfield! {
    /// Raw attribute data for GetCapability command
    #[derive(Copy, Clone, PartialEq, Eq)]
    struct AttributesRaw(u32);
    impl Debug;

    /// Supports disabled state as defined in Type-C spec
    pub bool, disabled_state_support, set_disabled_state_support: 0;
    /// PPM supports battery charging spec with version given in [`GetCapabilityDataRaw::bcd_battery_charging_spec`]
    pub bool, battery_charging, set_battery_charging: 1;
    /// PPM supports USB PD spec with version given in [`GetCapabilityDataRaw::bcd_usb_pd_spec`]
    pub bool, usb_power_delivery, set_usb_power_delivery: 2;
    /// PPM supports USB Type-C spec with version given in [`GetCapabilityDataRaw::bcd_type_c_spec`]
    pub bool, usb_type_c_current, set_usb_type_c_current: 6;
    /// Supported power sources bitmap
    pub u8, bm_power_source, set_bm_power_source: 15, 8;
}

#[cfg(feature = "defmt")]
impl defmt::Format for AttributesRaw {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "AttributesRaw {{ .0: {}, \
            disabled_state_support: {}, \
            battery_charging: {}, \
            usb_power_delivery: {}, \
            usb_type_c_current: {}, \
            bm_power_source: {} }}",
            self.0,
            self.disabled_state_support(),
            self.battery_charging(),
            self.usb_power_delivery(),
            self.usb_type_c_current(),
            self.bm_power_source()
        )
    }
}

/// Higher-level wrapper around [`AttributesRaw`]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Attributes(AttributesRaw);

impl Attributes {
    /// Returns whether the disabled state is supported
    pub fn disabled_state_support(&self) -> bool {
        self.0.disabled_state_support()
    }

    /// Sets whether the disabled state is supported
    pub fn set_disabled_state_support(&mut self, value: bool) -> &mut Self {
        self.0.set_disabled_state_support(value);
        self
    }

    /// Returns whether battery charging is supported
    pub fn battery_charging(&self) -> bool {
        self.0.battery_charging()
    }

    /// Sets whether battery charging is supported
    pub fn set_battery_charging(&mut self, value: bool) -> &mut Self {
        self.0.set_battery_charging(value);
        self
    }

    /// Returns whether USB PD is supported
    pub fn usb_power_delivery(&self) -> bool {
        self.0.usb_power_delivery()
    }

    /// Sets whether USB PD is supported
    pub fn set_usb_power_delivery(&mut self, value: bool) -> &mut Self {
        self.0.set_usb_power_delivery(value);
        self
    }

    /// Returns whether USB Type-C current is supported
    pub fn usb_type_c_current(&self) -> bool {
        self.0.usb_type_c_current()
    }

    /// Sets whether USB Type-C current is supported
    pub fn set_usb_type_c_current(&mut self, value: bool) -> &mut Self {
        self.0.set_usb_type_c_current(value);
        self
    }

    /// Returns the power source bitmap
    pub fn power_source(&self) -> PowerSource {
        self.0.bm_power_source().into()
    }

    /// Sets the power source bitmap
    pub fn set_power_source(&mut self, value: PowerSource) -> &mut Self {
        self.0.set_bm_power_source(value.0 .0);
        self
    }
}

impl From<u32> for Attributes {
    fn from(raw: u32) -> Self {
        Attributes(AttributesRaw(raw))
    }
}

impl Default for Attributes {
    fn default() -> Self {
        Attributes(AttributesRaw(0))
    }
}

impl Encode for Attributes {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.0 .0.encode(encoder)
    }
}

impl<Context> Decode<Context> for Attributes {
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, DecodeError> {
        let raw = u32::decode(decoder)?;
        Ok(Attributes::from(raw))
    }
}

/// Get capability response data
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ResponseData {
    /// Attributes
    pub attributes: Attributes,
    /// Number of connectors
    pub num_connectors: u8,
    /// Optional features
    pub optional_features: OptionalFeatures,
    /// Number of supported alternate modes
    pub num_alt_modes: u8,
    /// BCD coded battery charging spec version
    pub bcd_battery_charging_spec: u16,
    /// BCD coded USB PD spec version
    pub bcd_usb_pd_spec: u16,
    /// BCD coded Type-C spec version
    pub bcd_type_c_spec: u16,
}

impl Encode for ResponseData {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        Encode::encode(&self.attributes, encoder)?;
        Encode::encode(&self.num_connectors, encoder)?;
        Encode::encode(&self.optional_features, encoder)?;
        Encode::encode(&self.num_alt_modes, encoder)?;
        Encode::encode(&0u8, encoder)?; // Reserved byte
        Encode::encode(&self.bcd_battery_charging_spec, encoder)?;
        Encode::encode(&self.bcd_usb_pd_spec, encoder)?;
        Encode::encode(&self.bcd_type_c_spec, encoder)?;
        Ok(())
    }
}

impl<Context> Decode<Context> for ResponseData {
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, DecodeError> {
        let attributes = Attributes::decode(decoder)?;
        let num_connectors = u8::decode(decoder)?;
        let optional_features = OptionalFeatures::decode(decoder)?;
        let num_alt_modes = u8::decode(decoder)?;
        let _reserved = u8::decode(decoder)?; // Reserved byte
        let bcd_battery_charging_spec = u16::decode(decoder)?;
        let bcd_usb_pd_spec = u16::decode(decoder)?;
        let bcd_type_c_spec = u16::decode(decoder)?;

        Ok(ResponseData {
            attributes,
            num_connectors,
            optional_features,
            num_alt_modes,
            bcd_battery_charging_spec,
            bcd_usb_pd_spec,
            bcd_type_c_spec,
        })
    }
}

#[cfg(test)]
pub mod test {
    use bincode::config::standard;
    use bincode::{decode_from_slice, encode_into_slice};

    use super::*;

    /// Create a standard response data value for testing
    pub fn create_response_data() -> (ResponseData, [u8; RESPONSE_DATA_LEN]) {
        let response_data = ResponseData {
            attributes: Attributes::from(0x43),
            num_connectors: 2,
            optional_features: OptionalFeatures::from(0xFF),
            num_alt_modes: 3,
            bcd_battery_charging_spec: 0x0120,
            bcd_usb_pd_spec: 0x0300,
            bcd_type_c_spec: 0x0200,
        };

        let mut bytes = [0u8; RESPONSE_DATA_LEN];

        // Attributes - 4 bytes
        // Disable state support + Battery charging + USB PD + USB Type-C
        bytes[0] = 0x43;

        // Num connectors - 1 byte
        bytes[4] = 2;

        // Optional features - 3 bytes
        bytes[5] = 0xFF; // Support everything

        // Number of support alt modes
        bytes[8] = 3; // Let's just say 3

        // bcdBCVersion - 2 bytes
        // 1.20
        bytes[10] = 0x20;
        bytes[11] = 0x01;

        // bcdPDVersion - 2 bytes
        // 3.00
        bytes[12] = 0x00;
        bytes[13] = 0x03;

        // bcdUSBTypeCVersion - 2 bytes
        // 2.00
        bytes[14] = 0x00;
        bytes[15] = 0x02;

        (response_data, bytes)
    }

    #[test]
    fn test_encode_response_data() {
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

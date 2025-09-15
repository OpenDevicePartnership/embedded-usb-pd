//! Types for GET_CABLE_PROPERTY command, see UCSI spec 6.5.16
use bincode::de::Decoder;
use bincode::enc::Encoder;
use bincode::error::{DecodeError, EncodeError};
use bincode::{Decode, Encode};
use bitfield::bitfield;

use crate::pdo::MA50_UNIT;
use crate::ucsi::{CommandHeaderRaw, COMMAND_LEN};

/// Data length for the GET_CABLE_PROPERTY command response
pub const RESPONSE_DATA_LEN: usize = 5;
/// Command padding
// -1 for the connector number byte
pub const COMMAND_PADDING: usize = COMMAND_LEN - size_of::<CommandHeaderRaw>() - 1;

/// Command arguments
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
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
    /// Raw speed supported type
    #[derive(Copy, Clone, PartialEq, Eq)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    struct SpeedSupportedRaw(u16);
    impl Debug;

    /// Connector number
    pub u8, units, set_units: 1, 0;
    /// Value
    pub u16, value, set_value: 15, 2;
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SpeedSupported {
    /// Bits per second
    Bps(u16),
    /// Kilobits per second
    Kbps(u16),
    /// Megabits per second
    Mbps(u16),
    /// Gigabits per second
    Gbps(u16),
}

impl From<u16> for SpeedSupported {
    fn from(value: u16) -> Self {
        let raw = SpeedSupportedRaw(value);
        match raw.units() {
            0x0 => SpeedSupported::Bps(raw.value()),
            0x1 => SpeedSupported::Kbps(raw.value()),
            0x2 => SpeedSupported::Mbps(raw.value()),
            0x3 => SpeedSupported::Gbps(raw.value()),
            _ => unreachable!(),
        }
    }
}

impl From<SpeedSupported> for u16 {
    fn from(value: SpeedSupported) -> Self {
        let mut raw = SpeedSupportedRaw(0);
        match value {
            SpeedSupported::Bps(v) => {
                raw.set_units(0x0);
                raw.set_value(v);
            }
            SpeedSupported::Kbps(v) => {
                raw.set_units(0x1);
                raw.set_value(v);
            }
            SpeedSupported::Mbps(v) => {
                raw.set_units(0x2);
                raw.set_value(v);
            }
            SpeedSupported::Gbps(v) => {
                raw.set_units(0x3);
                raw.set_value(v);
            }
        }
        raw.0
    }
}

impl Default for SpeedSupported {
    fn default() -> Self {
        SpeedSupported::Bps(0)
    }
}

/// Cable plug end type
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PlugEndType {
    /// Type-A plug
    #[default]
    TypeA,
    /// Type-B plug
    TypeB,
    /// Type-C plug
    TypeC,
    /// Not USB
    Other,
}

impl From<u8> for PlugEndType {
    fn from(value: u8) -> Self {
        match value & 0x3 {
            0x0 => PlugEndType::TypeA,
            0x1 => PlugEndType::TypeB,
            0x2 => PlugEndType::TypeC,
            0x3 => PlugEndType::Other,
            _ => unreachable!(),
        }
    }
}

impl From<PlugEndType> for u8 {
    fn from(value: PlugEndType) -> Self {
        match value {
            PlugEndType::TypeA => 0x0,
            PlugEndType::TypeB => 0x1,
            PlugEndType::TypeC => 0x2,
            PlugEndType::Other => 0x3,
        }
    }
}

bitfield! {
    /// Raw response
    #[derive(Copy, Clone, PartialEq, Eq)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    struct ResponseRaw([u8]);
    impl Debug;

    /// Speed supported
    pub u16, speed_supported, set_speed_supported: 15, 0;
    /// Current capability in 50mA units
    pub u8, current_capability, set_current_capability: 23, 16;
    /// True the cable has an end-to-end vbus connection
    pub bool, vbus_in_cable, set_vbus_in_cable: 24;
    /// True if the cable is an acive cable
    pub bool, active_cable, set_active_cable: 25;
    /// True if lane directionality is configurable
    pub bool, directionality_configurable, set_directionality_configurable: 26;
    /// Plug end type
    pub u8, plug_end_type, set_plug_end_type: 28, 27;
    /// True if the cable supports alternate modes
    pub bool, alt_mode_supported, set_alt_mode_supported: 29;
    /// Cable PD major version
    pub u8, cable_pd_major, set_cable_pd_major: 31, 30;
    /// Latency
    pub u8, latency, set_latency: 35, 32;
}

/// GET_CABLE_PROPERTY response data
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ResponseData {
    /// Speed supported
    pub speed_supported: SpeedSupported,
    /// Current capability in mA
    pub current_capability: u16,
    /// True the cable has an end-to-end vbus connection
    pub vbus_in_cable: bool,
    /// True if the cable is an active cable
    pub active_cable: bool,
    /// True if lane directionality is configurable
    pub directionality_configurable: bool,
    /// Plug end type
    pub plug_end_type: PlugEndType,
    /// True if the cable supports alternate modes
    pub alt_mode_supported: bool,
    /// Cable PD major version
    pub cable_pd_major: u8,
    /// Latency
    pub latency: u8,
}

impl From<[u8; RESPONSE_DATA_LEN]> for ResponseData {
    fn from(value: [u8; RESPONSE_DATA_LEN]) -> Self {
        let raw = ResponseRaw(value);
        ResponseData {
            speed_supported: SpeedSupported::from(raw.speed_supported()),
            current_capability: (raw.current_capability() as u16) * MA50_UNIT,
            vbus_in_cable: raw.vbus_in_cable(),
            active_cable: raw.active_cable(),
            directionality_configurable: raw.directionality_configurable(),
            plug_end_type: raw.plug_end_type().into(),
            alt_mode_supported: raw.alt_mode_supported(),
            cable_pd_major: raw.cable_pd_major(),
            latency: raw.latency(),
        }
    }
}

impl From<ResponseData> for [u8; RESPONSE_DATA_LEN] {
    fn from(value: ResponseData) -> [u8; RESPONSE_DATA_LEN] {
        let mut raw = ResponseRaw([0u8; RESPONSE_DATA_LEN]);
        let speed: u16 = value.speed_supported.into();
        raw.set_speed_supported(speed);
        raw.set_current_capability((value.current_capability / MA50_UNIT) as u8);
        raw.set_vbus_in_cable(value.vbus_in_cable);
        raw.set_active_cable(value.active_cable);
        raw.set_directionality_configurable(value.directionality_configurable);
        raw.set_plug_end_type(value.plug_end_type.into());
        raw.set_alt_mode_supported(value.alt_mode_supported);
        raw.set_cable_pd_major(value.cable_pd_major);
        raw.set_latency(value.latency);
        raw.0
    }
}

impl Encode for ResponseData {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        let raw: [u8; RESPONSE_DATA_LEN] = (*self).into();
        raw.encode(encoder)
    }
}

impl<Context> Decode<Context> for ResponseData {
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, DecodeError> {
        let raw: [u8; RESPONSE_DATA_LEN] = Decode::decode(decoder)?;
        Ok(raw.into())
    }
}

#[cfg(test)]
mod test {
    use bincode::config::standard;
    use bincode::decode_from_slice;

    use super::*;

    #[test]
    fn test_encode_response_data() {
        let bytes: [u8; RESPONSE_DATA_LEN] = [0x05, 0x00, 0x02, 0xF7, 0x0A];
        let expected = ResponseData {
            speed_supported: SpeedSupported::Kbps(1),
            current_capability: 100,
            vbus_in_cable: true,
            active_cable: true,
            directionality_configurable: true,
            plug_end_type: PlugEndType::TypeC,
            alt_mode_supported: true,
            cable_pd_major: 3,
            latency: 10,
        };
        let (data, len): (ResponseData, _) =
            decode_from_slice(&bytes, standard().with_fixed_int_encoding()).expect("Decoding failed");
        assert_eq!(data, expected);
        assert_eq!(len, RESPONSE_DATA_LEN);
    }
}

//! Types for GET_CONNECTOR_STATUS command, see UCSI spec 6.5.7

use bincode::de::Decoder;
use bincode::enc::Encoder;
use bincode::error::{DecodeError, EncodeError};
use bincode::{Decode, Encode};
use bitfield::bitfield;

use crate::ucsi::{CommandHeaderRaw, COMMAND_LEN};

/// Data length for the GET_CONNECTOR_CAPABILITY command response
pub const RESPONSE_DATA_LEN: usize = 2;
/// Command padding, -1 for the connector number byte
pub const COMMAND_PADDING: usize = COMMAND_LEN - size_of::<CommandHeaderRaw>() - 1;

bitfield! {
    /// Operation mode raw flags
    #[derive(Copy, Default, Clone, PartialEq, Eq)]
    pub struct OperationModeFlagsRaw(u8);
    impl Debug;
    pub bool, rp_only, set_rp_only: 0;
    pub bool, rd_only, set_rd_only: 1;
    pub bool, drp, set_drp: 2;
    pub bool, analog_audio, set_analog_audio: 3;
    pub bool, debug_accessory, set_debug_accessory: 4;
    pub bool, usb2, set_usb2: 5;
    pub bool, usb3, set_usb3: 6;
    pub bool, alternate_mode, set_alternate_mode: 7;
}

#[cfg(feature = "defmt")]
impl defmt::Format for OperationModeFlagsRaw {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "OperationModeFlagsRaw {{ .0: {}, \
            rp_only: {}, \
            rd_only: {}, \
            drp: {}, \
            analog_audio: {}, \
            debug_accessory: {}, \
            usb2: {}, \
            usb3: {}, \
            alternate_mode: {} }}",
            self.0,
            self.rp_only(),
            self.rd_only(),
            self.drp(),
            self.analog_audio(),
            self.debug_accessory(),
            self.usb2(),
            self.usb3(),
            self.alternate_mode(),
        )
    }
}

/// Operation mode flags
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct OperationModeFlags(OperationModeFlagsRaw);

impl OperationModeFlags {
    pub fn rp_only(&self) -> bool {
        self.0.rp_only()
    }

    pub fn set_rp_only(&mut self, value: bool) -> &mut Self {
        self.0.set_rp_only(value);
        self
    }

    pub fn rd_only(&self) -> bool {
        self.0.rd_only()
    }

    pub fn set_rd_only(&mut self, value: bool) -> &mut Self {
        self.0.set_rd_only(value);
        self
    }

    pub fn drp(&self) -> bool {
        self.0.drp()
    }

    pub fn set_drp(&mut self, value: bool) -> &mut Self {
        self.0.set_drp(value);
        self
    }

    pub fn analog_audio(&self) -> bool {
        self.0.analog_audio()
    }

    pub fn set_analog_audio(&mut self, value: bool) -> &mut Self {
        self.0.set_analog_audio(value);
        self
    }

    pub fn debug_accessory(&self) -> bool {
        self.0.debug_accessory()
    }

    pub fn set_debug_accessory(&mut self, value: bool) -> &mut Self {
        self.0.set_debug_accessory(value);
        self
    }

    pub fn usb2(&self) -> bool {
        self.0.usb2()
    }

    pub fn set_usb2(&mut self, value: bool) -> &mut Self {
        self.0.set_usb2(value);
        self
    }

    pub fn usb3(&self) -> bool {
        self.0.usb3()
    }

    pub fn set_usb3(&mut self, value: bool) -> &mut Self {
        self.0.set_usb3(value);
        self
    }

    pub fn alternate_mode(&self) -> bool {
        self.0.alternate_mode()
    }

    pub fn set_alternate_mode(&mut self, value: bool) -> &mut Self {
        self.0.set_alternate_mode(value);
        self
    }
}

impl Default for OperationModeFlags {
    fn default() -> Self {
        Self(OperationModeFlagsRaw(0))
    }
}

impl From<u8> for OperationModeFlags {
    fn from(raw: u8) -> Self {
        Self(OperationModeFlagsRaw(raw))
    }
}

impl From<OperationModeFlags> for u8 {
    fn from(raw: OperationModeFlags) -> Self {
        raw.0 .0
    }
}

bitfield! {
    /// Raw GET_CONNECTOR_CAPABILITY response bitfield
    #[derive(Copy, Clone, Default, PartialEq, Eq)]
    pub struct ResponseDataRaw(u16);
    impl Debug;

    pub u8, operation_mode, set_operation_mode: 7, 0;
    pub bool, provider, set_provider: 8;
    pub bool, consumer, set_consumer: 9;
    pub bool, swap_to_dfp, set_swap_to_dfp: 10;
    pub bool, swap_to_ufp, set_swap_to_ufp: 11;
    pub bool, swap_to_src, set_swap_to_src: 12;
    pub bool, swap_to_snk, set_swap_to_snk: 13;
}

#[cfg(feature = "defmt")]
impl defmt::Format for ResponseDataRaw {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "ResponseDataRaw {{ .0: {}, \
            operation_mode: {}, \
            provider: {}, \
            consumer: {}, \
            swap_to_dfp: {}, \
            swap_to_ufp: {}, \
            swap_to_src: {}, \
            swap_to_snk: {} }}",
            self.0,
            self.operation_mode(),
            self.provider(),
            self.consumer(),
            self.swap_to_dfp(),
            self.swap_to_ufp(),
            self.swap_to_src(),
            self.swap_to_snk()
        )
    }
}

/// Response data
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ResponseData(ResponseDataRaw);
impl ResponseData {
    pub fn operation_mode(&self) -> OperationModeFlags {
        OperationModeFlags::from(self.0.operation_mode())
    }

    pub fn set_operation_mode(&mut self, value: OperationModeFlags) -> &mut Self {
        self.0.set_operation_mode(u8::from(value));
        self
    }

    pub fn provider(&self) -> bool {
        self.0.provider()
    }

    pub fn set_provider(&mut self, value: bool) -> &mut Self {
        self.0.set_provider(value);
        self
    }

    pub fn consumer(&self) -> bool {
        self.0.consumer()
    }

    pub fn set_consumer(&mut self, value: bool) -> &mut Self {
        self.0.set_consumer(value);
        self
    }

    pub fn swap_to_dfp(&self) -> bool {
        self.0.swap_to_dfp()
    }

    pub fn set_swap_to_dfp(&mut self, value: bool) -> &mut Self {
        self.0.set_swap_to_dfp(value);
        self
    }

    pub fn swap_to_ufp(&self) -> bool {
        self.0.swap_to_ufp()
    }

    pub fn set_swap_to_ufp(&mut self, value: bool) -> &mut Self {
        self.0.set_swap_to_ufp(value);
        self
    }

    pub fn swap_to_src(&self) -> bool {
        self.0.swap_to_src()
    }

    pub fn set_swap_to_src(&mut self, value: bool) -> &mut Self {
        self.0.set_swap_to_src(value);
        self
    }

    pub fn swap_to_snk(&self) -> bool {
        self.0.swap_to_snk()
    }

    pub fn set_swap_to_snk(&mut self, value: bool) -> &mut Self {
        self.0.set_swap_to_snk(value);
        self
    }
}

impl Default for ResponseData {
    fn default() -> Self {
        Self(ResponseDataRaw(0))
    }
}

impl From<u16> for ResponseData {
    fn from(raw: u16) -> Self {
        Self(ResponseDataRaw(raw))
    }
}

impl From<ResponseData> for u16 {
    fn from(data: ResponseData) -> Self {
        data.0 .0
    }
}

impl Encode for ResponseData {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.0 .0.encode(encoder)
    }
}

impl<Context> Decode<Context> for ResponseData {
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, DecodeError> {
        Ok(ResponseData::from(u16::decode(decoder)?))
    }
}

/// GET_CONNECTOR_CAPABILITY command arguments
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
mod test {
    use bincode::config::standard;
    use bincode::{decode_from_slice, encode_into_slice};

    use super::*;

    #[test]
    fn test_decode_response_data() {
        // Byte 0
        // Operation mode - Rp only + USB2 + Alt mode
        // Byte 1
        // Bits 8-13 all set
        let bytes: [u8; RESPONSE_DATA_LEN] = [0xA1, 0x3F];

        let expected = *ResponseData::default()
            .set_operation_mode(
                *OperationModeFlags::default()
                    .set_rp_only(true)
                    .set_usb2(true)
                    .set_alternate_mode(true),
            )
            .set_consumer(true)
            .set_provider(true)
            .set_swap_to_dfp(true)
            .set_swap_to_ufp(true)
            .set_swap_to_src(true)
            .set_swap_to_snk(true);

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

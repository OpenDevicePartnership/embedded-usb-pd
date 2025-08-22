//! Types for GET_CONNECTOR_STATUS command, see UCSI spec 6.5.7

use bincode::de::Decoder;
use bincode::enc::Encoder;
use bincode::error::{DecodeError, EncodeError};
use bincode::{Decode, Encode};
use bitfield::bitfield;

use crate::ucsi::{CommandHeaderRaw, COMMAND_LEN};

/// Data length for the GET_CONNECTOR_CAPABILITY command response
pub const RESPONSE_DATA_LEN: usize = 4;
/// Command padding, -1 for the connector number byte
pub const COMMAND_PADDING: usize = COMMAND_LEN - size_of::<CommandHeaderRaw>() - 1;

bitfield! {
    /// Operation mode raw flags
    #[derive(Copy, Default, Clone, PartialEq, Eq)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
    /// Extended operation mode raw flags
    #[derive(Copy, Default, Clone, PartialEq, Eq)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub struct ExtendedOperationModeFlagsRaw(u8);
    impl Debug;
    pub bool, usb4_gen2, set_usb4_gen2: 0;
    pub bool, epr_source, set_epr_source: 1;
    pub bool, epr_sink, set_epr_sink: 2;
    pub bool, usb4_gen3, set_usb4_gen3: 3;
    pub bool, usb4_gen4, set_usb4_gen4: 4;
}

/// Extended operation mode flags
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ExtendedOperationModeFlags(ExtendedOperationModeFlagsRaw);

impl ExtendedOperationModeFlags {
    pub fn usb4_gen2(&self) -> bool {
        self.0.usb4_gen2()
    }

    pub fn set_usb4_gen2(&mut self, value: bool) -> &mut Self {
        self.0.set_usb4_gen2(value);
        self
    }

    pub fn epr_source(&self) -> bool {
        self.0.epr_source()
    }

    pub fn set_epr_source(&mut self, value: bool) -> &mut Self {
        self.0.set_epr_source(value);
        self
    }

    pub fn epr_sink(&self) -> bool {
        self.0.epr_sink()
    }

    pub fn set_epr_sink(&mut self, value: bool) -> &mut Self {
        self.0.set_epr_sink(value);
        self
    }

    pub fn usb4_gen3(&self) -> bool {
        self.0.usb4_gen3()
    }

    pub fn set_usb4_gen3(&mut self, value: bool) -> &mut Self {
        self.0.set_usb4_gen3(value);
        self
    }

    pub fn usb4_gen4(&self) -> bool {
        self.0.usb4_gen4()
    }

    pub fn set_usb4_gen4(&mut self, value: bool) -> &mut Self {
        self.0.set_usb4_gen4(value);
        self
    }
}

impl Default for ExtendedOperationModeFlags {
    fn default() -> Self {
        Self(ExtendedOperationModeFlagsRaw(0))
    }
}

impl From<u8> for ExtendedOperationModeFlags {
    fn from(raw: u8) -> Self {
        ExtendedOperationModeFlags(ExtendedOperationModeFlagsRaw(raw))
    }
}

impl From<ExtendedOperationModeFlags> for u8 {
    fn from(raw: ExtendedOperationModeFlags) -> Self {
        raw.0 .0
    }
}

bitfield! {
    /// Miscellaneous capabilities raw flags
    #[derive(Copy, Default, Clone, PartialEq, Eq)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub struct MiscCapabilitiesRaw(u8);
    impl Debug;
    pub bool, fw_update, set_fw_update: 0;
    pub bool, security, set_security: 1;
}

/// Miscellaneous capabilities
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct MiscCapabilities(MiscCapabilitiesRaw);

impl MiscCapabilities {
    pub fn fw_update(&self) -> bool {
        self.0.fw_update()
    }

    pub fn set_fw_update(&mut self, value: bool) -> &mut Self {
        self.0.set_fw_update(value);
        self
    }

    pub fn security(&self) -> bool {
        self.0.security()
    }

    pub fn set_security(&mut self, value: bool) -> &mut Self {
        self.0.set_security(value);
        self
    }
}

impl Default for MiscCapabilities {
    fn default() -> Self {
        Self(MiscCapabilitiesRaw(0))
    }
}

impl From<u8> for MiscCapabilities {
    fn from(raw: u8) -> Self {
        Self(MiscCapabilitiesRaw(raw))
    }
}

impl From<MiscCapabilities> for u8 {
    fn from(raw: MiscCapabilities) -> Self {
        raw.0 .0
    }
}

bitfield! {
    /// Raw GET_CONNECTOR_CAPABILITY response bitfield
    #[derive(Copy, Clone, Default, PartialEq, Eq)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub struct ResponseDataRaw(u32);
    impl Debug;

    pub u8, operation_mode, set_operation_mode: 7, 0;
    pub bool, provider, set_provider: 8;
    pub bool, consumer, set_consumer: 9;
    pub bool, swap_to_dfp, set_swap_to_dfp: 10;
    pub bool, swap_to_ufp, set_swap_to_ufp: 11;
    pub bool, swap_to_src, set_swap_to_src: 12;
    pub bool, swap_to_snk, set_swap_to_snk: 13;
    pub u8, extended_operation_mode, set_extended_operation_mode: 21, 14;
    pub u8, misc_capabilities, set_misc_capabilities: 25, 22;
    pub bool, reverse_current_protection, set_reverse_current_protection: 26;
    pub u8, partner_pd_revision, set_partner_pd_revision: 28, 27;
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

    pub fn extended_operation_mode(&self) -> ExtendedOperationModeFlags {
        ExtendedOperationModeFlags::from(self.0.extended_operation_mode())
    }

    pub fn set_extended_operation_mode(&mut self, value: ExtendedOperationModeFlags) -> &mut Self {
        self.0.set_extended_operation_mode(u8::from(value));
        self
    }

    pub fn misc_capabilities(&self) -> MiscCapabilities {
        MiscCapabilities::from(self.0.misc_capabilities())
    }

    pub fn set_misc_capabilities(&mut self, value: MiscCapabilities) -> &mut Self {
        self.0.set_misc_capabilities(u8::from(value));
        self
    }

    pub fn reverse_current_protection(&self) -> bool {
        self.0.reverse_current_protection()
    }

    pub fn set_reverse_current_protection(&mut self, value: bool) -> &mut Self {
        self.0.set_reverse_current_protection(value);
        self
    }

    pub fn partner_pd_revision(&self) -> u8 {
        self.0.partner_pd_revision()
    }

    pub fn set_partner_pd_revision(&mut self, value: u8) -> &mut Self {
        self.0.set_partner_pd_revision(value);
        self
    }
}

impl Default for ResponseData {
    fn default() -> Self {
        Self(ResponseDataRaw(0))
    }
}

impl From<u32> for ResponseData {
    fn from(raw: u32) -> Self {
        Self(ResponseDataRaw(raw))
    }
}

impl From<ResponseData> for u32 {
    fn from(data: ResponseData) -> Self {
        data.0 .0
    }
}

impl Encode for ResponseData {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        u32::from(*self).encode(encoder)
    }
}

impl<Context> Decode<Context> for ResponseData {
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, DecodeError> {
        Ok(ResponseData::from(u32::decode(decoder)?))
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
        // Extended operation mode usb4_gen2 + epr_source
        // Byte 2
        // Both misc capabilities set
        // Byte 3
        // Reverse current protection
        // Partner PD revision 1
        let bytes = [0xA1, 0xFF, 0xC0, 0x0C];

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
            .set_swap_to_snk(true)
            .set_extended_operation_mode(
                *ExtendedOperationModeFlags::default()
                    .set_usb4_gen2(true)
                    .set_epr_source(true),
            )
            .set_misc_capabilities(*MiscCapabilities::default().set_fw_update(true).set_security(true))
            .set_reverse_current_protection(true)
            .set_partner_pd_revision(1);

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

use bincode::de::{Decode, Decoder};
use bincode::enc::{Encode, Encoder};
use bincode::error::{DecodeError, EncodeError};
use bitfield::bitfield;

use crate::ucsi::{CommandHeaderRaw, COMMAND_LEN};

/// Data length for the GET_CONNECTOR_STATUS command response
pub const RESPONSE_DATA_LEN: usize = MAX_VENDOR_DATA_LEN + size_of::<InformationRaw>();
/// Command padding, -1 for the connector number byte
pub const COMMAND_PADDING: usize = COMMAND_LEN - size_of::<CommandHeaderRaw>() - 1;

/// Maximum support vendor-data length
pub const MAX_VENDOR_DATA_LEN: usize = 14;

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
    /// Raw error bitfield
    #[derive(Copy, Clone, PartialEq, Eq)]
    struct InformationRaw(u16);
    impl Debug;

    /// Unrecognized command
    pub bool, unrecognized_command, set_unrecognized_command: 0;
    /// Invalid connector number
    pub bool, invalid_connector, set_invalid_connector: 1;
    /// Invalid command arguments
    pub bool, invalid_command_args, set_invalid_command_args: 2;
    /// Incompatible partner
    pub bool, incompatible_partner, set_incompatible_partner: 3;
    /// CC communication error
    pub bool, cc_comm, set_cc_com: 4;
    /// Failed due to dead battery
    pub bool, dead_battery, set_dead_battery: 5;
    /// Contract negociation failure
    pub bool, contract_failure, set_contract_failure: 6;
    /// Overcurrent
    pub bool, overcurrent, set_overcurrent: 7;
    /// Undefined
    pub bool, undefined, set_undefined: 8;
    /// Swap rejected by port partner
    pub bool, port_partner_rejected_swap, set_port_partner_rejected_swap: 9;
    /// Hard reset
    pub bool, hard_reset, set_hard_reset: 10;
    /// PPM policy conflict
    pub bool, ppm_policy_conflict, set_ppm_policy_conflict: 11;
    /// Swap rejected
    pub bool, swap_rejected, set_swap_rejected: 12;
    /// Reverse current protection
    pub bool, reverse_current_protection, set_reverse_current_protection: 13;
    /// Set sink path rejected
    pub bool, sink_path_rejected, set_sink_path_rejected: 14;
}

#[cfg(feature = "defmt")]
impl defmt::Format for InformationRaw {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "InformationRaw {{ .0: {}, \
            unrecognized_command: {}, \
            invalid_connector: {}, \
            invalid_command_args: {}, \
            incompatible_partner: {}, \
            cc_comm: {}, \
            dead_battery: {}, \
            contract_failure: {}, \
            overcurrent: {}, \
            undefined: {}, \
            port_partner_rejected_swap: {}, \
            hard_reset: {}, \
            ppm_policy_conflict: {}, \
            swap_rejected: {}, \
            reverse_current_protection: {}, \
            sink_path_rejected: {} }}",
            self.0,
            self.unrecognized_command(),
            self.invalid_connector(),
            self.invalid_command_args(),
            self.incompatible_partner(),
            self.cc_comm(),
            self.dead_battery(),
            self.contract_failure(),
            self.overcurrent(),
            self.undefined(),
            self.port_partner_rejected_swap(),
            self.hard_reset(),
            self.ppm_policy_conflict(),
            self.swap_rejected(),
            self.reverse_current_protection(),
            self.sink_path_rejected()
        )
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Information(InformationRaw);

impl Information {
    pub fn unrecognized_command(&self) -> bool {
        self.0.unrecognized_command()
    }

    pub fn set_unrecognized_command(&mut self, value: bool) -> &mut Self {
        self.0.set_unrecognized_command(value);
        self
    }

    pub fn invalid_connector(&self) -> bool {
        self.0.invalid_connector()
    }

    pub fn set_invalid_connector(&mut self, value: bool) -> &mut Self {
        self.0.set_invalid_connector(value);
        self
    }

    pub fn invalid_command_args(&self) -> bool {
        self.0.invalid_command_args()
    }

    pub fn set_invalid_command_args(&mut self, value: bool) -> &mut Self {
        self.0.set_invalid_command_args(value);
        self
    }

    pub fn incompatible_partner(&self) -> bool {
        self.0.incompatible_partner()
    }

    pub fn set_incompatible_partner(&mut self, value: bool) -> &mut Self {
        self.0.set_incompatible_partner(value);
        self
    }

    pub fn cc_comm(&self) -> bool {
        self.0.cc_comm()
    }

    pub fn set_cc_comm(&mut self, value: bool) -> &mut Self {
        self.0.set_cc_com(value);
        self
    }

    pub fn dead_battery(&self) -> bool {
        self.0.dead_battery()
    }

    pub fn set_dead_battery(&mut self, value: bool) -> &mut Self {
        self.0.set_dead_battery(value);
        self
    }

    pub fn contract_failure(&self) -> bool {
        self.0.contract_failure()
    }

    pub fn set_contract_failure(&mut self, value: bool) -> &mut Self {
        self.0.set_contract_failure(value);
        self
    }

    pub fn overcurrent(&self) -> bool {
        self.0.overcurrent()
    }

    pub fn set_overcurrent(&mut self, value: bool) -> &mut Self {
        self.0.set_overcurrent(value);
        self
    }

    pub fn undefined(&self) -> bool {
        self.0.undefined()
    }

    pub fn set_undefined(&mut self, value: bool) -> &mut Self {
        self.0.set_undefined(value);
        self
    }

    pub fn port_partner_rejected_swap(&self) -> bool {
        self.0.port_partner_rejected_swap()
    }

    pub fn set_port_partner_rejected_swap(&mut self, value: bool) -> &mut Self {
        self.0.set_port_partner_rejected_swap(value);
        self
    }

    pub fn hard_reset(&self) -> bool {
        self.0.hard_reset()
    }

    pub fn set_hard_reset(&mut self, value: bool) -> &mut Self {
        self.0.set_hard_reset(value);
        self
    }

    pub fn ppm_policy_conflict(&self) -> bool {
        self.0.ppm_policy_conflict()
    }

    pub fn set_ppm_policy_conflict(&mut self, value: bool) -> &mut Self {
        self.0.set_ppm_policy_conflict(value);
        self
    }

    pub fn swap_rejected(&self) -> bool {
        self.0.swap_rejected()
    }

    pub fn set_swap_rejected(&mut self, value: bool) -> &mut Self {
        self.0.set_swap_rejected(value);
        self
    }

    pub fn reverse_current_protection(&self) -> bool {
        self.0.reverse_current_protection()
    }

    pub fn set_reverse_current_protection(&mut self, value: bool) -> &mut Self {
        self.0.set_reverse_current_protection(value);
        self
    }

    pub fn sink_path_rejected(&self) -> bool {
        self.0.sink_path_rejected()
    }

    pub fn set_sink_path_rejected(&mut self, value: bool) -> &mut Self {
        self.0.set_sink_path_rejected(value);
        self
    }
}

impl Default for Information {
    fn default() -> Self {
        Self(InformationRaw(0))
    }
}

impl From<u16> for Information {
    fn from(value: u16) -> Self {
        Self(InformationRaw(value))
    }
}

impl From<Information> for u16 {
    fn from(info: Information) -> Self {
        info.0 .0
    }
}

/// Response data
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ResponseData {
    /// Error information
    pub information: Information,
    /// Vendor-specific error information
    pub vendor: [u8; MAX_VENDOR_DATA_LEN],
}

impl Encode for ResponseData {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        u16::encode(&self.information.into(), encoder)?;
        self.vendor.encode(encoder)?;
        Ok(())
    }
}

impl<Context> Decode<Context> for ResponseData {
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, DecodeError> {
        let information = Information::from(u16::decode(decoder)?);
        let vendor = <[u8; MAX_VENDOR_DATA_LEN]>::decode(decoder)?;
        Ok(ResponseData { information, vendor })
    }
}

#[cfg(test)]
mod test {
    use bincode::config::standard;
    use bincode::{decode_from_slice, encode_into_slice};

    use super::*;

    #[test]
    fn test_encode_response_data() {
        let mut bytes = [0u8; RESPONSE_DATA_LEN];
        let expected = ResponseData {
            information: *Information::default()
                .set_cc_comm(true)
                .set_dead_battery(true)
                .set_contract_failure(true)
                .set_overcurrent(true),
            vendor: [
                0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            ],
        };

        bytes[0] = 0xf0;
        bytes[2] = 0x01;

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

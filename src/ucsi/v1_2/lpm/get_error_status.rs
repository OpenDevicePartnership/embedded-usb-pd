//! Types for GET_ERROR_STATUS command, see UCSI spec 6.5.17

use bitfield::bitfield;
use bytemuck::{Pod, Zeroable};
use pack1::U16LE;

use crate::ucsi::v1_2::lpm::ConnectorNumberRaw;
use crate::ucsi::v1_2::{CommandHeaderRaw, COMMAND_LEN};
use crate::PortId;

/// Data length for the GET_CONNECTOR_STATUS command response
pub const RESPONSE_DATA_LEN: usize = MAX_VENDOR_DATA_LEN + size_of::<InformationRaw>();
/// Command padding
pub const COMMAND_PADDING: usize = COMMAND_LEN - size_of::<CommandHeaderRaw>() - size_of::<ConnectorNumberRaw>();

/// Maximum support vendor-data length
pub const MAX_VENDOR_DATA_LEN: usize = 14;

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
    /// Raw error bitfield
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct InformationRaw(u16);
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
    /// Contract negotiation failure
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

/// Error information flags
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Information {
    /// Unrecognized command
    pub unrecognized_command: bool,
    /// Invalid connector number
    pub invalid_connector: bool,
    /// Invalid command arguments
    pub invalid_command_args: bool,
    /// Incompatible partner
    pub incompatible_partner: bool,
    /// CC communication error
    pub cc_comm: bool,
    /// Failed due to dead battery
    pub dead_battery: bool,
    /// Contract negociation failure
    pub contract_failure: bool,
    /// Overcurrent
    pub overcurrent: bool,
    /// Undefined
    pub undefined: bool,
    /// Swap rejected by port partner
    pub port_partner_rejected_swap: bool,
    /// Hard reset
    pub hard_reset: bool,
    /// PPM policy conflict
    pub ppm_policy_conflict: bool,
    /// Swap rejected
    pub swap_rejected: bool,
    /// Reverse current protection
    pub reverse_current_protection: bool,
    /// Set sink path rejected
    pub sink_path_rejected: bool,
}

impl From<InformationRaw> for Information {
    fn from(raw: InformationRaw) -> Self {
        Self {
            unrecognized_command: raw.unrecognized_command(),
            invalid_connector: raw.invalid_connector(),
            invalid_command_args: raw.invalid_command_args(),
            incompatible_partner: raw.incompatible_partner(),
            cc_comm: raw.cc_comm(),
            dead_battery: raw.dead_battery(),
            contract_failure: raw.contract_failure(),
            overcurrent: raw.overcurrent(),
            undefined: raw.undefined(),
            port_partner_rejected_swap: raw.port_partner_rejected_swap(),
            hard_reset: raw.hard_reset(),
            ppm_policy_conflict: raw.ppm_policy_conflict(),
            swap_rejected: raw.swap_rejected(),
            reverse_current_protection: raw.reverse_current_protection(),
            sink_path_rejected: raw.sink_path_rejected(),
        }
    }
}

impl From<Information> for InformationRaw {
    fn from(info: Information) -> Self {
        let mut raw = InformationRaw(0);
        raw.set_unrecognized_command(info.unrecognized_command);
        raw.set_invalid_connector(info.invalid_connector);
        raw.set_invalid_command_args(info.invalid_command_args);
        raw.set_incompatible_partner(info.incompatible_partner);
        raw.set_cc_com(info.cc_comm);
        raw.set_dead_battery(info.dead_battery);
        raw.set_contract_failure(info.contract_failure);
        raw.set_overcurrent(info.overcurrent);
        raw.set_undefined(info.undefined);
        raw.set_port_partner_rejected_swap(info.port_partner_rejected_swap);
        raw.set_hard_reset(info.hard_reset);
        raw.set_ppm_policy_conflict(info.ppm_policy_conflict);
        raw.set_swap_rejected(info.swap_rejected);
        raw.set_reverse_current_protection(info.reverse_current_protection);
        raw.set_sink_path_rejected(info.sink_path_rejected);
        raw
    }
}

impl From<u16> for Information {
    fn from(value: u16) -> Self {
        InformationRaw(value).into()
    }
}

impl From<Information> for u16 {
    fn from(info: Information) -> Self {
        InformationRaw::from(info).0
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

/// Raw wire format of [`ResponseData`]
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Zeroable, Pod)]
pub struct ResponseDataRaw {
    /// Error information bits, see [`InformationRaw`]
    pub information: U16LE,
    /// Vendor-specific error information
    pub vendor: [u8; MAX_VENDOR_DATA_LEN],
}

impl ResponseDataRaw {
    /// Length of the raw response data in bytes
    pub const LEN: usize = size_of::<Self>();
}

#[cfg(feature = "defmt")]
impl defmt::Format for ResponseDataRaw {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "ResponseDataRaw {{ information: {}, vendor: {} }}",
            InformationRaw(self.information.get()),
            self.vendor
        )
    }
}

impl From<ResponseDataRaw> for ResponseData {
    fn from(raw: ResponseDataRaw) -> Self {
        Self {
            information: raw.information.get().into(),
            vendor: raw.vendor,
        }
    }
}

impl From<ResponseData> for ResponseDataRaw {
    fn from(data: ResponseData) -> Self {
        Self {
            information: U16LE::new(data.information.into()),
            vendor: data.vendor,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::GlobalPortId;

    /// Mask of all bits defined by [`InformationRaw`]
    const DEFINED_BITS: u16 = 0x7FFF;

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
    fn test_information_roundtrip() {
        for bit in 0..u16::BITS {
            let raw = 1u16 << bit;
            // Undefined bits are dropped by the roundtrip
            assert_eq!(u16::from(Information::from(raw)), raw & DEFINED_BITS);
        }
    }

    #[test]
    fn test_response_data_roundtrip() {
        let bytes: [u8; RESPONSE_DATA_LEN] = [
            0xF0, 0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE,
        ];
        let expected = ResponseData {
            information: Information {
                cc_comm: true,
                dead_battery: true,
                contract_failure: true,
                overcurrent: true,
                ..Default::default()
            },
            vendor: [
                0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE,
            ],
        };

        assert_eq!(
            ResponseData::from(bytemuck::must_cast::<_, ResponseDataRaw>(bytes)),
            expected
        );
        let encoded: [u8; RESPONSE_DATA_LEN] = bytemuck::must_cast(ResponseDataRaw::from(expected));
        assert_eq!(encoded, bytes);
    }
}

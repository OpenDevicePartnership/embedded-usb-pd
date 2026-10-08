//! Types for GET_CONNECTOR_CAPABILITY command, see UCSI spec 6.5.6

use bitfield::bitfield;
use bytemuck::{Pod, Zeroable};
use pack1::U16LE;

use crate::ucsi::v1_2::lpm::ConnectorNumberRaw;
use crate::ucsi::v1_2::{CommandHeaderRaw, COMMAND_LEN};
use crate::PortId;

/// Data length for the GET_CONNECTOR_CAPABILITY command response
pub const RESPONSE_DATA_LEN: usize = 2;
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
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct OperationModeFlags {
    /// Rp only
    pub rp_only: bool,
    /// Rd only
    pub rd_only: bool,
    /// DRP
    pub drp: bool,
    /// Analog audio accessory
    pub analog_audio: bool,
    /// Debug accessory
    pub debug_accessory: bool,
    /// USB2
    pub usb2: bool,
    /// USB3
    pub usb3: bool,
    /// Alternate mode
    pub alternate_mode: bool,
}

impl From<OperationModeFlagsRaw> for OperationModeFlags {
    fn from(raw: OperationModeFlagsRaw) -> Self {
        Self {
            rp_only: raw.rp_only(),
            rd_only: raw.rd_only(),
            drp: raw.drp(),
            analog_audio: raw.analog_audio(),
            debug_accessory: raw.debug_accessory(),
            usb2: raw.usb2(),
            usb3: raw.usb3(),
            alternate_mode: raw.alternate_mode(),
        }
    }
}

impl From<OperationModeFlags> for OperationModeFlagsRaw {
    fn from(flags: OperationModeFlags) -> Self {
        let mut raw = OperationModeFlagsRaw(0);
        raw.set_rp_only(flags.rp_only);
        raw.set_rd_only(flags.rd_only);
        raw.set_drp(flags.drp);
        raw.set_analog_audio(flags.analog_audio);
        raw.set_debug_accessory(flags.debug_accessory);
        raw.set_usb2(flags.usb2);
        raw.set_usb3(flags.usb3);
        raw.set_alternate_mode(flags.alternate_mode);
        raw
    }
}

impl From<u8> for OperationModeFlags {
    fn from(raw: u8) -> Self {
        OperationModeFlagsRaw(raw).into()
    }
}

impl From<OperationModeFlags> for u8 {
    fn from(flags: OperationModeFlags) -> Self {
        OperationModeFlagsRaw::from(flags).0
    }
}

bitfield! {
    /// Raw GET_CONNECTOR_CAPABILITY response bitfield
    #[derive(Copy, Clone, Default, PartialEq, Eq)]
    pub struct ResponseBitsRaw(u16);
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
impl defmt::Format for ResponseBitsRaw {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "ResponseBitsRaw {{ .0: {}, \
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
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ResponseData {
    /// Supported operation modes
    pub operation_mode: OperationModeFlags,
    /// Capable of providing power
    pub provider: bool,
    /// Capable of consuming power
    pub consumer: bool,
    /// Supports swapping to DFP
    pub swap_to_dfp: bool,
    /// Supports swapping to UFP
    pub swap_to_ufp: bool,
    /// Supports swapping to source
    pub swap_to_src: bool,
    /// Supports swapping to sink
    pub swap_to_snk: bool,
}

impl From<ResponseBitsRaw> for ResponseData {
    fn from(raw: ResponseBitsRaw) -> Self {
        Self {
            operation_mode: raw.operation_mode().into(),
            provider: raw.provider(),
            consumer: raw.consumer(),
            swap_to_dfp: raw.swap_to_dfp(),
            swap_to_ufp: raw.swap_to_ufp(),
            swap_to_src: raw.swap_to_src(),
            swap_to_snk: raw.swap_to_snk(),
        }
    }
}

impl From<ResponseData> for ResponseBitsRaw {
    fn from(data: ResponseData) -> Self {
        let mut raw = ResponseBitsRaw(0);
        raw.set_operation_mode(data.operation_mode.into());
        raw.set_provider(data.provider);
        raw.set_consumer(data.consumer);
        raw.set_swap_to_dfp(data.swap_to_dfp);
        raw.set_swap_to_ufp(data.swap_to_ufp);
        raw.set_swap_to_src(data.swap_to_src);
        raw.set_swap_to_snk(data.swap_to_snk);
        raw
    }
}

impl From<u16> for ResponseData {
    fn from(raw: u16) -> Self {
        ResponseBitsRaw(raw).into()
    }
}

impl From<ResponseData> for u16 {
    fn from(data: ResponseData) -> Self {
        ResponseBitsRaw::from(data).0
    }
}

/// Raw wire format of [`ResponseData`]
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Zeroable, Pod)]
pub struct ResponseDataRaw {
    /// Response bits, see [`ResponseBitsRaw`]
    pub bits: U16LE,
}

impl ResponseDataRaw {
    /// Length of the raw response data in bytes
    pub const LEN: usize = size_of::<Self>();
}

#[cfg(feature = "defmt")]
impl defmt::Format for ResponseDataRaw {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "ResponseDataRaw {{ bits: {} }}", ResponseBitsRaw(self.bits.get()))
    }
}

impl From<ResponseDataRaw> for ResponseData {
    fn from(raw: ResponseDataRaw) -> Self {
        raw.bits.get().into()
    }
}

impl From<ResponseData> for ResponseDataRaw {
    fn from(data: ResponseData) -> Self {
        Self {
            bits: U16LE::new(data.into()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::GlobalPortId;

    /// Mask of all bits defined by [`ResponseBitsRaw`]
    const DEFINED_BITS: u16 = 0x3FFF;

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
    fn test_response_bits_roundtrip() {
        for bit in 0..u16::BITS {
            let raw = 1u16 << bit;
            // Undefined bits are dropped by the roundtrip
            assert_eq!(u16::from(ResponseData::from(raw)), raw & DEFINED_BITS);
        }
    }

    #[test]
    fn test_response_data_roundtrip() {
        // Byte 0
        // Operation mode - Rp only + USB2 + Alt mode
        // Byte 1
        // Bits 8-13 all set
        let bytes: [u8; RESPONSE_DATA_LEN] = [0xA1, 0x3F];

        let expected = ResponseData {
            operation_mode: OperationModeFlags {
                rp_only: true,
                usb2: true,
                alternate_mode: true,
                ..Default::default()
            },
            provider: true,
            consumer: true,
            swap_to_dfp: true,
            swap_to_ufp: true,
            swap_to_src: true,
            swap_to_snk: true,
        };

        assert_eq!(
            ResponseData::from(bytemuck::must_cast::<_, ResponseDataRaw>(bytes)),
            expected
        );
        let encoded: [u8; RESPONSE_DATA_LEN] = bytemuck::must_cast(ResponseDataRaw::from(expected));
        assert_eq!(encoded, bytes);
    }
}

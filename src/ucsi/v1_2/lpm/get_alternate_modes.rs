//! Types for GET_ALTERNATE_MODES command, see UCSI spec 6.5.11

use bitfield::bitfield;
use bytemuck::{Pod, Zeroable};
use pack1::{U16LE, U32LE};

use super::Recipient;
use crate::ucsi::v1_2::lpm::InvalidRecipient;
use crate::ucsi::v1_2::{CommandHeaderRaw, COMMAND_LEN};
use crate::vdm::structured::Svid;
use crate::vdm::AltModeId;
use crate::PortId;

/// Data length for the GET_ALTERNATE_MODES command response
pub const RESPONSE_DATA_LEN: usize = 12;
/// Command padding
pub const COMMAND_PADDING: usize = COMMAND_LEN - size_of::<CommandHeaderRaw>() - size_of::<ArgBitsRaw>();

/// Invalid number of modes error, contains the invalid number of modes value
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct InvalidNumModes(pub u8);

/// Top-level error for this command
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    InvalidRecipient(InvalidRecipient),
    InvalidNumModes(InvalidNumModes),
}

bitfield! {
    /// Raw argument bits
    #[derive(Copy, Clone, Default, PartialEq, Eq)]
    pub struct ArgBitsRaw(u32);
    impl Debug;

    /// Recipient
    pub u8, recipient, set_recipient: 2, 0;
    /// Connector number, unlike most commands this is not in the first byte
    pub u8, connector_number, set_connector_number: 14, 8;
    /// Alternate mode offset
    pub u8, mode_offset, set_mode_offset: 23, 16;
    /// Number of alternate modes
    pub u8, num_modes, set_num_modes: 25, 24;
}

#[cfg(feature = "defmt")]
impl defmt::Format for ArgBitsRaw {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "ArgBitsRaw {{ .0: {}, recipient: {}, connector_number: {}, mode_offset: {}, num_modes: {} }}",
            self.0,
            self.recipient(),
            self.connector_number(),
            self.mode_offset(),
            self.num_modes()
        )
    }
}

/// Command arguments
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Args<T: PortId> {
    /// Recipient
    pub recipient: Recipient,
    /// Connector number
    pub connector_number: T,
    /// Alternate mode offset
    pub mode_offset: u8,
    /// Number of alternate modes
    pub num_modes: u8,
}

impl<T: PortId, Out: PortId> crate::ucsi::MapPort<T, Out> for Args<T> {
    type Output = Args<Out>;

    fn map_port<F>(self, f: F) -> Self::Output
    where
        F: Fn(T) -> Out,
    {
        Args {
            recipient: self.recipient,
            connector_number: f(self.connector_number),
            mode_offset: self.mode_offset,
            num_modes: self.num_modes,
        }
    }
}

impl<T: PortId> Default for Args<T> {
    fn default() -> Self {
        Self {
            recipient: Recipient::Connector,
            connector_number: T::from(0),
            mode_offset: 0,
            num_modes: 1,
        }
    }
}

impl<T: PortId> TryFrom<ArgBitsRaw> for Args<T> {
    type Error = Error;

    fn try_from(raw: ArgBitsRaw) -> Result<Self, Self::Error> {
        Ok(Self {
            recipient: raw.recipient().try_into().map_err(Error::InvalidRecipient)?,
            connector_number: raw.connector_number().into(),
            mode_offset: raw.mode_offset(),
            num_modes: match raw.num_modes() {
                0 => Ok(1),
                1 => Ok(2),
                v => Err(Error::InvalidNumModes(InvalidNumModes(v))),
            }?,
        })
    }
}

impl<T: PortId> TryFrom<Args<T>> for ArgBitsRaw {
    type Error = Error;

    fn try_from(args: Args<T>) -> Result<Self, Self::Error> {
        let mut raw = ArgBitsRaw(0);
        raw.set_recipient(args.recipient.into());
        raw.set_connector_number(args.connector_number.into());
        raw.set_mode_offset(args.mode_offset);
        if args.num_modes == 1 || args.num_modes == 2 {
            raw.set_num_modes(args.num_modes - 1);
        } else {
            return Err(Error::InvalidNumModes(InvalidNumModes(args.num_modes)));
        }

        Ok(raw)
    }
}

impl<T: PortId> TryFrom<u32> for Args<T> {
    type Error = Error;

    fn try_from(raw: u32) -> Result<Self, Self::Error> {
        ArgBitsRaw(raw).try_into()
    }
}

impl<T: PortId> TryFrom<Args<T>> for u32 {
    type Error = Error;

    fn try_from(args: Args<T>) -> Result<Self, Self::Error> {
        Ok(ArgBitsRaw::try_from(args)?.0)
    }
}

/// Raw wire format of [`Args`]
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Zeroable, Pod)]
pub struct ArgsRaw {
    /// Argument bits, see [`ArgBitsRaw`]
    pub bits: U32LE,
    /// Reserved bytes, filling out the remainder of the command
    _reserved: [u8; COMMAND_PADDING],
}

impl ArgsRaw {
    /// Length of the raw arguments in bytes
    pub const LEN: usize = size_of::<Self>();
}

#[cfg(feature = "defmt")]
impl defmt::Format for ArgsRaw {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "ArgsRaw {{ bits: {} }}", ArgBitsRaw(self.bits.get()))
    }
}

impl<T: PortId> TryFrom<Args<T>> for ArgsRaw {
    type Error = Error;

    fn try_from(args: Args<T>) -> Result<Self, Self::Error> {
        Ok(Self {
            bits: U32LE::new(args.try_into()?),
            ..Default::default()
        })
    }
}

impl<T: PortId> TryFrom<ArgsRaw> for Args<T> {
    type Error = Error;

    fn try_from(raw: ArgsRaw) -> Result<Self, Self::Error> {
        raw.bits.get().try_into()
    }
}

/// Representation of a single alt-mode
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct AltMode {
    pub svid: Svid,
    pub mid: AltModeId,
}

/// Raw wire format of [`AltMode`]
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Zeroable, Pod)]
pub struct AltModeRaw {
    /// SVID
    pub svid: U16LE,
    /// Mode ID
    pub mid: U32LE,
}

#[cfg(feature = "defmt")]
impl defmt::Format for AltModeRaw {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "AltModeRaw {{ svid: {}, mid: {} }}",
            self.svid.get(),
            self.mid.get()
        )
    }
}

impl From<AltMode> for AltModeRaw {
    fn from(alt_mode: AltMode) -> Self {
        Self {
            svid: U16LE::new(alt_mode.svid.0),
            mid: U32LE::new(alt_mode.mid.0),
        }
    }
}

impl From<AltModeRaw> for AltMode {
    fn from(raw: AltModeRaw) -> Self {
        Self {
            svid: Svid(raw.svid.get()),
            mid: AltModeId(raw.mid.get()),
        }
    }
}

/// Length of the alternate modes array
pub const ALT_MODES_LEN: usize = 2;

/// GET_ALTERNATE_MODES response data
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ResponseData {
    pub alt_modes: [AltMode; ALT_MODES_LEN],
}

/// Raw wire format of [`ResponseData`]
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Zeroable, Pod)]
pub struct ResponseDataRaw {
    /// Alternate modes
    pub alt_modes: [AltModeRaw; ALT_MODES_LEN],
}

impl ResponseDataRaw {
    /// Length of the raw response data in bytes
    pub const LEN: usize = size_of::<Self>();
}

#[cfg(feature = "defmt")]
impl defmt::Format for ResponseDataRaw {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "ResponseDataRaw {{ alt_modes: {} }}", self.alt_modes)
    }
}

impl From<ResponseData> for ResponseDataRaw {
    fn from(data: ResponseData) -> Self {
        Self {
            alt_modes: data.alt_modes.map(AltModeRaw::from),
        }
    }
}

impl From<ResponseDataRaw> for ResponseData {
    fn from(raw: ResponseDataRaw) -> Self {
        Self {
            alt_modes: raw.alt_modes.map(AltMode::from),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::GlobalPortId;

    #[test]
    fn test_raw_len() {
        assert_eq!(ArgsRaw::LEN, COMMAND_LEN - size_of::<CommandHeaderRaw>());
        assert_eq!(ResponseDataRaw::LEN, RESPONSE_DATA_LEN);
    }

    #[test]
    fn test_args_raw_roundtrip() {
        // SOP on connector 3, mode offset 1, 2 requested alt modes
        let encoded: [u8; ArgsRaw::LEN] = [0x01, 0x03, 0x01, 0x01, 0x00, 0x00];
        let expected: Args<GlobalPortId> = Args {
            recipient: Recipient::Sop,
            connector_number: GlobalPortId(3),
            mode_offset: 1,
            num_modes: 2,
        };

        assert_eq!(Args::try_from(bytemuck::must_cast::<_, ArgsRaw>(encoded)), Ok(expected));
        let bytes: [u8; ArgsRaw::LEN] = bytemuck::must_cast(ArgsRaw::try_from(expected).unwrap());
        assert_eq!(bytes, encoded);
    }

    #[test]
    fn test_args_raw_invalid_recipient() {
        // Invalid recipient (0x7), connector 3, 2 requested alt modes, mode offset 1
        let encoded: [u8; ArgsRaw::LEN] = [0x07, 0x03, 0x01, 0x01, 0x00, 0x00];
        assert_eq!(
            Args::<GlobalPortId>::try_from(bytemuck::must_cast::<_, ArgsRaw>(encoded)),
            Err(Error::InvalidRecipient(InvalidRecipient(0x07)))
        );
    }

    #[test]
    fn test_args_raw_invalid_num_modes() {
        // SOP on connector 3, 3 requested alt modes, mode offset 1
        let encoded: [u8; ArgsRaw::LEN] = [0x01, 0x03, 0x00, 0x02, 0x00, 0x00];
        assert_eq!(
            Args::<GlobalPortId>::try_from(bytemuck::must_cast::<_, ArgsRaw>(encoded)),
            Err(Error::InvalidNumModes(InvalidNumModes(0x02)))
        );
    }

    #[test]
    fn test_args_invalid_num_modes() {
        let args = Args::<GlobalPortId> {
            num_modes: 0,
            ..Default::default()
        };
        let raw: Result<ArgsRaw, _> = args.try_into();
        assert_eq!(raw, Err(Error::InvalidNumModes(InvalidNumModes(0))));
        let args = Args::<GlobalPortId> {
            num_modes: 3,
            ..Default::default()
        };
        let raw: Result<ArgsRaw, _> = args.try_into();
        assert_eq!(raw, Err(Error::InvalidNumModes(InvalidNumModes(3))));
    }

    #[test]
    fn test_response_data_raw_roundtrip() {
        // No particular meaning to these values
        let encoded: [u8; ResponseDataRaw::LEN] =
            [0x34, 0x12, 0x78, 0x56, 0x34, 0x12, 0x12, 0x34, 0x12, 0x34, 0x56, 0x78];
        let mut expected = ResponseData::default();
        expected.alt_modes[0].svid = Svid(0x1234);
        expected.alt_modes[0].mid = AltModeId(0x12345678);
        expected.alt_modes[1].svid = Svid(0x3412);
        expected.alt_modes[1].mid = AltModeId(0x78563412);

        assert_eq!(
            ResponseData::from(bytemuck::must_cast::<_, ResponseDataRaw>(encoded)),
            expected
        );
        let bytes: [u8; ResponseDataRaw::LEN] = bytemuck::must_cast(ResponseDataRaw::from(expected));
        assert_eq!(bytes, encoded);
    }
}

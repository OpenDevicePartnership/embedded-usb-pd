//! Types for GET_PDOs command, see UCSI spec 6.5.15
use bitfield::bitfield;
use bytemuck::{Pod, Zeroable};
use pack1::U32LE;

use crate::ucsi::v1_2::{CommandHeaderRaw, COMMAND_LEN};
use crate::{PortId, PowerRole};

/// Command padding
pub const COMMAND_PADDING: usize = COMMAND_LEN - size_of::<CommandHeaderRaw>() - size_of::<ArgBitsRaw>();
/// Max response data length, supports up to 4 PDOs
pub const RESPONSE_DATA_LEN: usize = MAX_PDOS * 4;
/// Maximum number of PDOs supported
pub const MAX_PDOS: usize = 4;

bitfield! {
    /// Raw argument bits
    #[derive(Copy, Clone, Default, PartialEq, Eq)]
    pub struct ArgBitsRaw(u32);
    impl Debug;

    /// Connector number
    pub u8, connector_number, set_connector_number: 6, 0;
    /// Partner
    pub bool, partner, set_partner: 7;
    /// PDO offset,
    pub u8, pdo_offset, set_pdo_offset: 15, 8;
    /// Number of PDOs, minus one
    pub u8, num_pdos, set_num_pdos: 17, 16;
    /// Source or sink PDOs?
    pub bool, source, set_source: 18;
    /// Source type capability
    pub u8, source_capability_type, set_source_capability_type: 20, 19;
}

#[cfg(feature = "defmt")]
impl defmt::Format for ArgBitsRaw {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "ArgBitsRaw {{ .0: {}, connector_number: {}, partner: {}, pdo_offset: {}, num_pdos: {}, source: {}, source_capability_type: {} }}",
            self.0,
            self.connector_number(),
            self.partner(),
            self.pdo_offset(),
            self.num_pdos(),
            self.source(),
            self.source_capability_type()
        )
    }
}

/// Source capability type to query
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SourceCapabilityType {
    /// Current source capabilities
    #[default]
    Current,
    /// Advertised source capabilities
    Advertised,
    /// Maximum source capabilities
    Maximum,
}

/// Invalid source capability type error
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct InvalidSourceCapabilityType(pub u8);

impl TryFrom<u8> for SourceCapabilityType {
    type Error = InvalidSourceCapabilityType;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x0 => Ok(SourceCapabilityType::Current),
            0x1 => Ok(SourceCapabilityType::Advertised),
            0x2 => Ok(SourceCapabilityType::Maximum),
            v => Err(InvalidSourceCapabilityType(v)),
        }
    }
}

impl From<SourceCapabilityType> for u8 {
    fn from(value: SourceCapabilityType) -> Self {
        match value {
            SourceCapabilityType::Current => 0x0,
            SourceCapabilityType::Advertised => 0x1,
            SourceCapabilityType::Maximum => 0x2,
        }
    }
}

/// Invalid number of PDOs error, contains the out of range value
///
/// The number of PDOs must be in the range `1..=MAX_PDOS`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct InvalidNumPdos(pub u8);

/// Command arguments
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Args<T: PortId> {
    /// Connector number
    pub connector_number: T,
    /// Retrieve the partner's PDOs instead of the connector's
    pub partner: bool,
    /// PDO offset
    pub pdo_offset: u8,
    /// Number of PDOs to retrieve, must be in the range `1..=MAX_PDOS`
    pub num_pdos: u8,
    /// Retrieve source or sink PDOs
    pub role: PowerRole,
    /// Source capability type
    pub source_capability_type: SourceCapabilityType,
}

impl<T: PortId> Default for Args<T> {
    fn default() -> Self {
        Self {
            connector_number: T::from(0),
            partner: false,
            pdo_offset: 0,
            num_pdos: 1,
            role: PowerRole::default(),
            source_capability_type: SourceCapabilityType::default(),
        }
    }
}

impl<T: PortId> TryFrom<ArgBitsRaw> for Args<T> {
    type Error = InvalidSourceCapabilityType;

    fn try_from(raw: ArgBitsRaw) -> Result<Self, Self::Error> {
        Ok(Self {
            connector_number: raw.connector_number().into(),
            partner: raw.partner(),
            pdo_offset: raw.pdo_offset(),
            // +1 as per UCSI spec
            num_pdos: raw.num_pdos() + 1,
            role: if raw.source() {
                PowerRole::Source
            } else {
                PowerRole::Sink
            },
            source_capability_type: raw.source_capability_type().try_into()?,
        })
    }
}

impl<T: PortId> TryFrom<Args<T>> for ArgBitsRaw {
    type Error = InvalidNumPdos;

    fn try_from(args: Args<T>) -> Result<Self, Self::Error> {
        if args.num_pdos == 0 || args.num_pdos > MAX_PDOS as u8 {
            return Err(InvalidNumPdos(args.num_pdos));
        }

        let mut raw = ArgBitsRaw(0);
        raw.set_connector_number(args.connector_number.into());
        raw.set_partner(args.partner);
        raw.set_pdo_offset(args.pdo_offset);
        // -1 as per UCSI spec
        raw.set_num_pdos(args.num_pdos - 1);
        raw.set_source(args.role == PowerRole::Source);
        raw.set_source_capability_type(args.source_capability_type.into());
        Ok(raw)
    }
}

impl<T: PortId> TryFrom<u32> for Args<T> {
    type Error = InvalidSourceCapabilityType;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        ArgBitsRaw(value).try_into()
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
    type Error = InvalidNumPdos;

    fn try_from(args: Args<T>) -> Result<Self, Self::Error> {
        Ok(Self {
            bits: U32LE::new(ArgBitsRaw::try_from(args)?.0),
            ..Default::default()
        })
    }
}

impl<T: PortId> TryFrom<ArgsRaw> for Args<T> {
    type Error = InvalidSourceCapabilityType;

    fn try_from(raw: ArgsRaw) -> Result<Self, Self::Error> {
        raw.bits.get().try_into()
    }
}

/// GET_PDO response data, supports up to [`MAX_PDOS`] PDOs
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ResponseData {
    /// Raw PDOs, the first zero PDO terminates the list
    pub pdos: [u32; MAX_PDOS],
}

impl ResponseData {
    /// Iterator over valid PDOs
    pub fn iter(&self) -> impl ExactSizeIterator<Item = u32> + '_ {
        // NOTE: If this changes the panic safety comment below should be revisited
        let last_pdo = self.pdos.iter().position(|&pdo| pdo == 0).unwrap_or(self.pdos.len());
        // Panic safety: `last_pdo` will always be in bounds
        #[allow(clippy::indexing_slicing)]
        self.pdos.as_slice()[..last_pdo].iter().copied()
    }

    /// Mutable iterator over valid PDOs
    pub fn iter_mut(&mut self) -> impl ExactSizeIterator<Item = &mut u32> + '_ {
        // NOTE: If this changes the panic safety comment below should be revisited
        let last_pdo = self.pdos.iter().position(|&pdo| pdo == 0).unwrap_or(self.pdos.len());
        // Panic safety: `last_pdo` will always be in bounds
        #[allow(clippy::indexing_slicing)]
        self.pdos.as_mut_slice()[..last_pdo].iter_mut()
    }
}

/// Raw wire format of [`ResponseData`]
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Zeroable, Pod)]
pub struct ResponseDataRaw {
    /// Raw PDOs
    pub pdos: [U32LE; MAX_PDOS],
}

impl ResponseDataRaw {
    /// Length of the raw response data in bytes
    pub const LEN: usize = size_of::<Self>();
}

#[cfg(feature = "defmt")]
impl defmt::Format for ResponseDataRaw {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "ResponseDataRaw {{ pdos: {} }}", self.pdos.map(U32LE::get))
    }
}

impl From<ResponseData> for ResponseDataRaw {
    fn from(data: ResponseData) -> Self {
        Self {
            pdos: data.pdos.map(U32LE::new),
        }
    }
}

impl From<ResponseDataRaw> for ResponseData {
    fn from(raw: ResponseDataRaw) -> Self {
        Self {
            pdos: raw.pdos.map(U32LE::get),
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
    fn test_response_data_raw_roundtrip() {
        let bytes: [u8; ResponseDataRaw::LEN] = [
            0x12, 0x00, 0x00, 0x00, 0x34, 0x00, 0x00, 0x00, 0x56, 0x00, 0x00, 0x00, 0x78, 0x00, 0x00, 0x00,
        ];
        let expected = ResponseData {
            pdos: [0x12, 0x34, 0x56, 0x78],
        };

        assert_eq!(
            ResponseData::from(bytemuck::must_cast::<_, ResponseDataRaw>(bytes)),
            expected
        );
        let encoded: [u8; ResponseDataRaw::LEN] = bytemuck::must_cast(ResponseDataRaw::from(expected));
        assert_eq!(encoded, bytes);
    }

    #[test]
    fn test_args_raw_roundtrip() {
        // Partner, connector 3, 1 PDO, source, maximum capabilities, offset 4
        let encoded: [u8; ArgsRaw::LEN] = [0x83, 0x04, 0x14, 0x00, 0x00, 0x00];
        let expected: Args<GlobalPortId> = Args {
            connector_number: GlobalPortId(3),
            partner: true,
            pdo_offset: 4,
            num_pdos: 1,
            role: PowerRole::Source,
            source_capability_type: SourceCapabilityType::Maximum,
        };

        assert_eq!(Args::try_from(bytemuck::must_cast::<_, ArgsRaw>(encoded)), Ok(expected));
        let bytes: [u8; ArgsRaw::LEN] = bytemuck::must_cast(ArgsRaw::try_from(expected).unwrap());
        assert_eq!(bytes, encoded);
    }

    #[test]
    fn test_args_raw_max_num_pdos() {
        // Sink, connector 1, 4 PDOs
        let encoded: [u8; ArgsRaw::LEN] = [0x01, 0x00, 0x03, 0x00, 0x00, 0x00];
        let expected: Args<GlobalPortId> = Args {
            connector_number: GlobalPortId(1),
            num_pdos: MAX_PDOS as u8,
            ..Default::default()
        };

        assert_eq!(Args::try_from(bytemuck::must_cast::<_, ArgsRaw>(encoded)), Ok(expected));
        let bytes: [u8; ArgsRaw::LEN] = bytemuck::must_cast(ArgsRaw::try_from(expected).unwrap());
        assert_eq!(bytes, encoded);
    }

    #[test]
    fn test_args_raw_invalid_num_pdos() {
        for num_pdos in [0, MAX_PDOS as u8 + 1, u8::MAX] {
            let args: Args<GlobalPortId> = Args {
                num_pdos,
                ..Default::default()
            };
            assert_eq!(ArgsRaw::try_from(args), Err(InvalidNumPdos(num_pdos)));
        }
    }

    #[test]
    fn test_args_raw_invalid_source_capability_type() {
        // Partner, connector 3, 1 PDO, source, invalid source-capability-type (0x3), offset 4
        let encoded: [u8; ArgsRaw::LEN] = [0x83, 0x04, 0x1C, 0x00, 0x00, 0x00];
        assert_eq!(
            Args::<GlobalPortId>::try_from(bytemuck::must_cast::<_, ArgsRaw>(encoded)),
            Err(InvalidSourceCapabilityType(0x03))
        );
    }

    #[test]
    fn test_response_iterator() {
        let response = ResponseData {
            pdos: [0x12, 0x34, 0x56, 0x00],
        };
        let mut iter = response.iter();
        assert_eq!(iter.len(), 3);
        assert_eq!(iter.next(), Some(0x12));
        assert_eq!(iter.len(), 2);
        assert_eq!(iter.next(), Some(0x34));
        assert_eq!(iter.len(), 1);
        assert_eq!(iter.next(), Some(0x56));
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.len(), 0);
    }

    #[test]
    fn test_response_iterator_empty() {
        let response = ResponseData { pdos: [0x00; MAX_PDOS] };
        let mut iter = response.iter();
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.len(), 0);
    }

    #[test]
    fn test_response_iterator_full() {
        let response = ResponseData {
            pdos: [0x12, 0x34, 0x56, 0x78],
        };
        let mut iter = response.iter();
        assert_eq!(iter.len(), 4);
        assert_eq!(iter.next(), Some(0x12));
        assert_eq!(iter.len(), 3);
        assert_eq!(iter.next(), Some(0x34));
        assert_eq!(iter.len(), 2);
        assert_eq!(iter.next(), Some(0x56));
        assert_eq!(iter.len(), 1);
        assert_eq!(iter.next(), Some(0x78));
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.len(), 0);
    }
}

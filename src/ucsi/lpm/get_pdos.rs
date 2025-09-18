//! Types for GET_PDOs command, see UCSI spec 6.5.15
use bincode::de::Decoder;
use bincode::enc::Encoder;
use bincode::error::{AllowedEnumVariants, DecodeError, EncodeError};
use bincode::{Decode, Encode};
use bitfield::bitfield;

use crate::ucsi::{CommandHeaderRaw, COMMAND_LEN};
use crate::PowerRole;

/// Command padding
pub const COMMAND_PADDING: usize = COMMAND_LEN - size_of::<CommandHeaderRaw>() - size_of::<ArgsRaw>();
/// Response data length, supports up to 4 PDOs
pub const RESPONSE_DATA_LEN: usize = MAX_PDOS * 4;
/// Maximum number of PDOs supported
pub const MAX_PDOS: usize = 4;

bitfield! {
    /// Raw arguments
    #[derive(Copy, Clone, Default, PartialEq, Eq)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub(super) struct ArgsRaw(u32);
    impl Debug;

    /// Connector number
    pub u8, connector_number, set_connector_number: 6, 0;
    /// Partner
    pub bool, partner, set_partner: 7;
    /// PDO offset,
    pub u8, pdo_offset, set_pdo_offset: 15, 8;
    /// Number of PDOs
    pub u8, num_pdos, set_num_pdos: 17, 16;
    /// Source or sink PDOs?
    pub bool, source, set_source: 18;
    /// Source type capability
    pub u8, source_capability_type, set_source_capability_type: 20, 19;
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

impl From<InvalidSourceCapabilityType> for DecodeError {
    fn from(value: InvalidSourceCapabilityType) -> Self {
        DecodeError::UnexpectedVariant {
            type_name: "SourceCapabilityType",
            found: value.0 as u32,
            allowed: &AllowedEnumVariants::Allowed(&[
                SourceCapabilityType::Current as u32,
                SourceCapabilityType::Advertised as u32,
                SourceCapabilityType::Maximum as u32,
            ]),
        }
    }
}

/// Command arguments
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Args(ArgsRaw);

impl Args {
    pub fn connector_number(&self) -> u8 {
        self.0.connector_number()
    }

    pub fn set_connector_number(&mut self, connector_number: u8) -> &mut Self {
        self.0.set_connector_number(connector_number);
        self
    }

    pub fn partner(&self) -> bool {
        self.0.partner()
    }

    pub fn set_partner(&mut self, partner: bool) -> &mut Self {
        self.0.set_partner(partner);
        self
    }

    pub fn pdo_offset(&self) -> u8 {
        self.0.pdo_offset()
    }

    pub fn set_pdo_offset(&mut self, pdo_offset: u8) -> &mut Self {
        self.0.set_pdo_offset(pdo_offset);
        self
    }

    pub fn num_pdos(&self) -> u8 {
        self.0.num_pdos()
    }

    pub fn set_num_pdos(&mut self, num_pdos: u8) -> &mut Self {
        self.0.set_num_pdos(num_pdos);
        self
    }

    pub fn role(&self) -> PowerRole {
        if self.0.source() {
            PowerRole::Source
        } else {
            PowerRole::Sink
        }
    }

    pub fn set_role(&mut self, pdo_type: PowerRole) -> &mut Self {
        self.0.set_source(pdo_type == PowerRole::Source);
        self
    }

    pub fn source_capability_type(&self) -> SourceCapabilityType {
        // Won't panic, validated in try_from
        self.0.source_capability_type().try_into().unwrap()
    }

    pub fn set_source_capability_type(&mut self, source_capabilities_type: SourceCapabilityType) -> &mut Self {
        self.0.set_source_capability_type(source_capabilities_type.into());
        self
    }
}

impl TryFrom<ArgsRaw> for Args {
    type Error = InvalidSourceCapabilityType;

    fn try_from(value: ArgsRaw) -> Result<Self, Self::Error> {
        // Validate source capability type
        let _: SourceCapabilityType = value.source_capability_type().try_into()?;
        Ok(Self(value))
    }
}

impl TryFrom<u32> for Args {
    type Error = InvalidSourceCapabilityType;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        ArgsRaw(value).try_into()
    }
}

impl Encode for Args {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.0 .0.encode(encoder)?;
        // Padding to fill the command length
        [0u8; COMMAND_PADDING].encode(encoder)
    }
}

impl<Context> Decode<Context> for Args {
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, DecodeError> {
        let raw = u32::decode(decoder)?;
        // Read padding
        let _padding: [u8; COMMAND_PADDING] = Decode::decode(decoder)?;
        Ok(Self(ArgsRaw(raw)))
    }
}

/// GET_PDO response data, supports up to [`MAX_PDOS`] PDOs
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ResponseData {
    pub raw: [u32; MAX_PDOS],
}

impl Encode for ResponseData {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.raw.encode(encoder)
    }
}

impl<Context> Decode<Context> for ResponseData {
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, DecodeError> {
        <[u32; MAX_PDOS]>::decode(decoder).map(|v| ResponseData { raw: v })
    }
}

#[cfg(test)]
mod test {
    use bincode::config::standard;
    use bincode::decode_from_slice;

    use super::*;

    #[test]
    fn test_encode_response_data() {
        let bytes: [u8; RESPONSE_DATA_LEN] = [
            0x12, 0x00, 0x00, 0x00, 0x34, 0x00, 0x00, 0x00, 0x56, 0x00, 0x00, 0x00, 0x78, 0x00, 0x00, 0x00,
        ];
        let expected = ResponseData {
            raw: [0x12, 0x34, 0x56, 0x78],
        };
        let (data, len): (ResponseData, _) =
            decode_from_slice(&bytes, standard().with_fixed_int_encoding()).expect("Decoding failed");
        assert_eq!(data, expected);
        assert_eq!(len, RESPONSE_DATA_LEN);
    }

    #[test]
    fn test_decode_args() {
        // Partner, connector 3, 1 PDO, source, maximum capabilities, offset 4
        let encoded: [u8; 6] = [0x83, 0x04, 0x15, 0x00, 0x00, 0x00];
        let (decoded, size): (Args, usize) = decode_from_slice(&encoded, standard().with_fixed_int_encoding()).unwrap();
        assert_eq!(size, 6);

        let expected = *Args::default()
            .set_connector_number(3)
            .set_partner(true)
            .set_pdo_offset(4)
            .set_num_pdos(1)
            .set_role(PowerRole::Source)
            .set_source_capability_type(SourceCapabilityType::Maximum);
        assert_eq!(decoded, expected);
    }
}

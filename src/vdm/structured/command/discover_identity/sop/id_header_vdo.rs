use super::{DfpProductTypeVdos, UfpProductTypeVdos};
use crate::vdm::structured::command::discover_identity::ConnectorType;

/// The ID Header VDO contains information corresponding to the Power Delivery Product.
///
/// This type differs from [`crate::vdm::structured::command::discover_identity::IdHeaderVdo`]
/// in that it contains the product type fields, which are encoded into the [`ResponseVdos::dfp_product_type_vdos`]
/// and [`ResponseVdos::ufp_product_type_vdos`] fields. This type is meant to be parsed directly from the raw VDO.
///
/// See PD spec 6.4.4.3.1.1 ID Header VDO, table 6.3.3 ID Header VDO.
///
/// [`ResponseVdos::dfp_product_type_vdos`]: super::ResponseVdos::dfp_product_type_vdos
/// [`ResponseVdos::ufp_product_type_vdos`]: super::ResponseVdos::ufp_product_type_vdos
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct IdHeaderVdo {
    /// The USB Vendor ID as assigned by the USB-IF.
    pub usb_vendor_id: u16,

    /// Identifies the device as either a USB Type-C receptacle of a USB Type-C plug.
    pub connector_type: ConnectorType,

    /// Indicates the type of Product when in DFP Data Role, whether a VDO will be
    /// returned, and if so, the type of VDO to be returned.
    ///
    /// The value of this type changes how [`ResponseVdos::dfp_product_type_vdos`] is interpreted.
    ///
    /// [`ResponseVdos::dfp_product_type_vdos`]: super::ResponseVdos::dfp_product_type_vdos
    pub product_type_dfp: ProductTypeDfp,

    /// Indicates whether or not the Product (either a Cable Plug or a device that
    /// can operate in the UFP role) is capable of supporting Modes.
    pub modal_operation_supported: bool,

    /// Indicates the type of Product when in UFP Data Role, whether a VDO will be
    /// returned, and if so, the type of VDO to be returned.
    ///
    /// The value of this type changes how [`ResponseVdos::ufp_product_type_vdos`] is interpreted.
    ///
    /// [`ResponseVdos::ufp_product_type_vdos`]: super::ResponseVdos::ufp_product_type_vdos
    pub product_type_ufp: ProductTypeUfp,

    /// Whether or not the Port has a USB Device Capability.
    pub usb_communication_capable_as_usb_device: bool,

    /// Whether or not the Port has a USB Host Capability.
    pub usb_communication_capable_as_usb_host: bool,
}

bitfield::bitfield! {
    /// The raw value of an [`IdHeaderVdo`], before parsing enumerations and bitfields.
    #[derive(Copy, Clone, PartialEq, Eq)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub struct Raw(u32);
    impl Debug;

    /// See [`IdHeaderVdo::usb_vendor_id`].
    pub u16, usb_vendor_id, set_usb_vendor_id: 15, 0;

    /// See [`IdHeaderVdo::connector_type`].
    pub u8, connector_type, set_connector_type: 22, 21;

    /// See [`IdHeaderVdo::product_type_dfp`].
    pub u8, product_type_dfp, set_product_type_dfp: 25, 23;

    /// See [`IdHeaderVdo::modal_operation_supported`].
    pub bool, modal_operation_supported, set_modal_operation_supported: 26;

    /// See [`IdHeaderVdo::product_type_ufp`].
    pub u8, product_type_ufp, set_product_type_ufp: 29, 27;

    /// See [`IdHeaderVdo::usb_communication_capable_as_usb_device`].
    pub bool, usb_communication_capable_as_usb_device, set_usb_communication_capable_as_usb_device: 30;

    /// See [`IdHeaderVdo::usb_communication_capable_as_usb_host`].
    pub bool, usb_communication_capable_as_usb_host, set_usb_communication_capable_as_usb_host: 31;
}

/// Errors that can occur when parsing an [`IdHeaderVdo`] from its raw value.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ParseIdHeaderVdoError {
    InvalidConnectorType,
    InvalidProductTypeDfp,
    InvalidProductTypeUfp,
}

impl TryFrom<Raw> for IdHeaderVdo {
    type Error = ParseIdHeaderVdoError;

    fn try_from(raw: Raw) -> Result<Self, Self::Error> {
        Ok(Self {
            usb_vendor_id: raw.usb_vendor_id(),
            connector_type: raw
                .connector_type()
                .try_into()
                .map_err(|()| ParseIdHeaderVdoError::InvalidConnectorType)?,
            product_type_dfp: raw
                .product_type_dfp()
                .try_into()
                .map_err(|()| ParseIdHeaderVdoError::InvalidProductTypeDfp)?,
            modal_operation_supported: raw.modal_operation_supported(),
            product_type_ufp: raw
                .product_type_ufp()
                .try_into()
                .map_err(|()| ParseIdHeaderVdoError::InvalidProductTypeUfp)?,
            usb_communication_capable_as_usb_device: raw.usb_communication_capable_as_usb_device(),
            usb_communication_capable_as_usb_host: raw.usb_communication_capable_as_usb_host(),
        })
    }
}

impl TryFrom<u32> for IdHeaderVdo {
    type Error = ParseIdHeaderVdoError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Raw(value).try_into()
    }
}

impl TryFrom<[u8; 4]> for IdHeaderVdo {
    type Error = ParseIdHeaderVdoError;

    fn try_from(bytes: [u8; 4]) -> Result<Self, Self::Error> {
        u32::from_le_bytes(bytes).try_into()
    }
}

impl From<IdHeaderVdo> for crate::vdm::structured::command::discover_identity::IdHeaderVdo {
    fn from(value: IdHeaderVdo) -> Self {
        Self {
            usb_vendor_id: value.usb_vendor_id,
            connector_type: value.connector_type,
            modal_operation_supported: value.modal_operation_supported,
            usb_communication_capable_as_usb_device: value.usb_communication_capable_as_usb_device,
            usb_communication_capable_as_usb_host: value.usb_communication_capable_as_usb_host,
        }
    }
}

/// The [`IdHeaderVdo::product_type_dfp`] field indicates the type of Product when
/// in DFP Data Role, whether a VDO will be returned, and if so, the type of VDO
/// to be returned.
///
/// See PD spec 6.4.4.3.1.1.6 Product Type (DFP), table 6.36 Product Types (DFP).
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ProductTypeDfp {
    /// This is not a DFP.
    ///
    /// The Product Type VDOs is empty.
    NotADfp,

    /// The product is a PDUSB Hub.
    ///
    /// If the device is not a Dual-Role Device, the first item in the Product Type
    /// VDOs is a [`DfpVdo`][`super::DfpVdo`].
    ///
    /// If the device is a Dual-Role Device, the first item in the Product Type VDOs
    /// is defined by [`IdHeaderVdo::product_type_ufp`], the second is padding (all 0s),
    /// and the third item is a [`DfpVdo`][`super::DfpVdo`].
    Hub,

    /// The product is a PDUSB Host or a PDUSB host that supports one or more Alternate
    /// Modes as an AMC.
    ///
    /// If the device is not a Dual-Role Device, the first item in the Product Type
    /// VDOs is a [`DfpVdo`][`super::DfpVdo`].
    ///
    /// If the device is a Dual-Role Device, the first item in the Product Type VDOs
    /// is defined by [`IdHeaderVdo::product_type_ufp`], the second is padding (all 0s),
    /// and the third item is a [`DfpVdo`][`super::DfpVdo`].
    Host,

    /// The product is a charger / power brick.
    ///
    /// If the device is not a Dual-Role Device, the first item in the Product Type
    /// VDOs is a [`DfpVdo`][`super::DfpVdo`].
    ///
    /// If the device is a Dual-Role Device, the first item in the Product Type VDOs
    /// is defined by [`IdHeaderVdo::product_type_ufp`], the second is padding (all 0s),
    /// and the third item is a [`DfpVdo`][`super::DfpVdo`].
    Charger,
}

impl TryFrom<u8> for ProductTypeDfp {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0b000 => Ok(Self::NotADfp),
            0b001 => Ok(Self::Hub),
            0b010 => Ok(Self::Host),
            0b011 => Ok(Self::Charger),
            _ => Err(()),
        }
    }
}

impl From<UfpProductTypeVdos> for ProductTypeUfp {
    fn from(ufp_product_type_vdos: UfpProductTypeVdos) -> Self {
        match ufp_product_type_vdos {
            UfpProductTypeVdos::NotAUfp => Self::NotAUfp,
            UfpProductTypeVdos::Hub(_) => Self::Hub,
            UfpProductTypeVdos::Peripheral(_) => Self::Peripheral,
            UfpProductTypeVdos::Psd => Self::Psd,
        }
    }
}

/// The [`IdHeaderVdo::product_type_ufp`] field indicates the type of Product when
/// in the UFP Data Role, whether a VDO will be returned, and if so, the type of
/// VDO to be returned.
///
/// See PD spec 6.4.4.3.1.1.3 Product Type (UFP), table 6.34 Product Types (UFP).
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ProductTypeUfp {
    /// This is not a UFP.
    ///
    /// The Product Type VDOs is empty.
    NotAUfp,

    /// The product is a PDUSB Hub.
    ///
    /// The first item in the Product Type VDOs is a [`UfpVdo`][`super::UfpVdo`].
    Hub,

    /// The product is a PDUSB Device other than a Hub.
    ///
    /// The first item in the Product Type VDOs is a [`UfpVdo`][`super::UfpVdo`].
    Peripheral,

    /// The product is a PSD, e.g., power bank.
    ///
    /// The Product Type VDOs is empty.
    Psd,
}

impl TryFrom<u8> for ProductTypeUfp {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0b000 => Ok(Self::NotAUfp),
            0b001 => Ok(Self::Hub),
            0b010 => Ok(Self::Peripheral),
            0b011 => Ok(Self::Psd),
            _ => Err(()),
        }
    }
}

impl From<DfpProductTypeVdos> for ProductTypeDfp {
    fn from(dfp_product_type_vdos: DfpProductTypeVdos) -> Self {
        match dfp_product_type_vdos {
            DfpProductTypeVdos::NotADfp => Self::NotADfp,
            DfpProductTypeVdos::Hub(_) => Self::Hub,
            DfpProductTypeVdos::Host(_) => Self::Host,
            DfpProductTypeVdos::Charger(_) => Self::Charger,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod product_type_dfp {
        use super::*;

        #[test]
        fn all_valid_variants() {
            let cases: [(u8, ProductTypeDfp); 4] = [
                (0b000, ProductTypeDfp::NotADfp),
                (0b001, ProductTypeDfp::Hub),
                (0b010, ProductTypeDfp::Host),
                (0b011, ProductTypeDfp::Charger),
            ];
            for (raw, expected) in cases {
                assert_eq!(ProductTypeDfp::try_from(raw), Ok(expected), "raw={raw}");
            }
        }

        #[test]
        fn invalid_values() {
            for v in [0b100, 0b101, 0b110, 0b111] {
                assert!(ProductTypeDfp::try_from(v).is_err(), "raw={v} should be invalid");
            }
        }
    }

    mod product_type_ufp {
        use super::*;

        #[test]
        fn all_valid_variants() {
            let cases: [(u8, ProductTypeUfp); 4] = [
                (0b000, ProductTypeUfp::NotAUfp),
                (0b001, ProductTypeUfp::Hub),
                (0b010, ProductTypeUfp::Peripheral),
                (0b011, ProductTypeUfp::Psd),
            ];
            for (raw, expected) in cases {
                assert_eq!(ProductTypeUfp::try_from(raw), Ok(expected), "raw={raw}");
            }
        }

        #[test]
        fn invalid_values() {
            for v in [0b100, 0b101, 0b110, 0b111] {
                assert!(ProductTypeUfp::try_from(v).is_err(), "raw={v} should be invalid");
            }
        }
    }
}

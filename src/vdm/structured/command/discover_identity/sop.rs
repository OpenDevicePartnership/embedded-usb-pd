//! [`Response`] contains the response to a Discover Identity Command targetting SOP.

use super::{CertStatVdo, ConnectorType, ProductVdo};
use crate::vdm::structured::{command::discover_identity::ProductTypeVdo, Header};

/// The response to a Discover Identity Command using SOP.
///
/// Each response contains up to 7 VDOs.
///
/// See PD spec 6.4.4.3.1 Discover Identity, table 6.16 Discover Identity Command response.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Response {
    /// The header for this Structured VDM Message.
    pub header: Header,

    /// Information corresponding to the Product.
    pub id: IdHeaderVdo,

    /// The XID assigned by the USB-IF to the product.
    pub cert_stat: Option<CertStatVdo>,

    /// Identity information relating to the product.
    pub product: Option<ProductVdo>,

    /// The Product-specific VDOs.
    ///
    /// The types of these VDOs are determined by fields in the [`Self::id`] field.
    pub product_type_vdos: [ProductTypeVdo; 3],
}

/// The ID Header VDO contains information corresponding to the Power Delivery Product.
///
/// See PD spec 6.4.4.3.1.1 ID Header VDO, table 6.3.3 ID Header VDO.
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
    /// The value of this type changes how [`Response::product_type_vdos`] is interpreted.
    pub product_type_dfp: ProductTypeDfp,

    /// Indicates whether or not the Product (either a Cable Plug or a device that
    /// can operate in the UFP role) is capable of supporting Modes.
    pub modal_operation_supported: bool,

    /// Indicates the type of Product when in UFP Data Role, whether a VDO will be
    /// returned, and if so, the type of VDO to be returned.
    ///
    /// The value of this type changes how [`Response::product_type_vdos`] is interpreted.
    pub product_type_ufp: ProductTypeUfp,

    /// Whether or not the Port has a USB Device Capability.
    pub usb_communication_capable_as_usb_device: bool,

    /// Whether or not the Port has a USB Host Capability.
    pub usb_communication_capable_as_usb_host: bool,
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
    /// [`Response::product_type_vdos`] is empty.
    NotADfp,

    /// The product is a PDUSB Hub.
    ///
    /// If the device is not a Dual-Role Device, the first item in [`Response::product_type_vdos`]
    /// is a [`DfpVdo`][`super::DfpVdo`].
    ///
    /// If the device is a Dual-Role Device, the first item in [`Response::product_type_vdos`]
    /// is defined by [`IdHeaderVdo::product_type_ufp`], the second is padding (all 0s),
    /// and the third item is a [`DfpVdo`][`super::DfpVdo`].
    Hub,

    /// The product is a PDUSB Host or a PDUSB host that supports one or more Alternate
    /// Modes as an AMC.
    ///
    /// If the device is not a Dual-Role Device, the first item in [`Response::product_type_vdos`]
    /// is a [`DfpVdo`][`super::DfpVdo`].
    ///
    /// If the device is a Dual-Role Device, the first item in [`Response::product_type_vdos`]
    /// is defined by [`IdHeaderVdo::product_type_ufp`], the second is padding (all 0s),
    /// and the third item is a [`DfpVdo`][`super::DfpVdo`].
    Host,

    /// The product is a charger / power brick.
    ///
    /// If the device is not a Dual-Role Device, the first item in [`Response::product_type_vdos`]
    /// is a [`DfpVdo`][`super::DfpVdo`].
    ///
    /// If the device is a Dual-Role Device, the first item in [`Response::product_type_vdos`]
    /// is defined by [`IdHeaderVdo::product_type_ufp`], the second is padding (all 0s),
    /// and the third item is a [`DfpVdo`][`super::DfpVdo`].
    Charger,
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
    /// [`Response::product_type_vdos`] is empty.
    NotAUfp,

    /// The product is a PDUSB Hub.
    ///
    /// The first item in [`Response::product_type_vdos`] is a [`UfpVdo`][`super::UfpVdo`].
    Hub,

    /// The product is a PDUSB Device other than a Hub.
    ///
    /// The first item in [`Response::product_type_vdos`] is a [`UfpVdo`][`super::UfpVdo`].
    Peripheral,

    /// The product is a PSD, e.g., power bank.
    ///
    /// [`Response::product_type_vdos`] is empty.
    Psd,
}

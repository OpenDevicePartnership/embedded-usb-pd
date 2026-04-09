//! [`Response`] contains the response to a Discover Identity Command targetting SOP'.

use super::{CertStatVdo, ConnectorType, ProductVdo};
use crate::vdm::structured::{command::discover_identity::ProductTypeVdo, Header};

/// The response to a Discover Identity Command using `SOP'`.
///
/// Each response contains up to 7 VDOs. The first three
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

    /// Indicates whether or not the Product (either a Cable Plug or a device that
    /// can operate in the UFP role) is capable of supporting Modes.
    pub modal_operation_supported: bool,

    /// Indicates the type of Product when the Product is a Cable Plug or VPD, whether
    /// a VDO will be returned, and if so, the type of VDO to be returned.
    ///
    /// The value of this type changes how [`Response::product_type_vdos`] is interpreted.
    pub product_type: ProductType,

    /// Whether or not the Port has a USB Device Capability.
    pub usb_communication_capable_as_usb_device: bool,

    /// Whether or not the Port has a USB Host Capability.
    pub usb_communication_capable_as_usb_host: bool,
}

/// The `SOP'` Product Type (Cable Plug/VPD) field indicates the type of Product
/// when the Product is a Cable Plug or VPD, whether a VDO will be returned, and
/// if so, the type of VDO to be returned.
///
/// The type changes how [`Response::product_type_vdos`] is interpreted.
///
/// See PD spec 6.4.4.3.1.1.4 Product Type (Cable Plug), table 6.35 Product Types (Cable Plug/VPD).
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ProductType {
    /// No other Product Type is appropriate.
    ///
    /// [`Response::product_type_vdos`] is empty.
    NotACablePlugVpd,

    /// The Product is a cable that does not incorporate signal conditioning circuits.
    ///
    /// The first item in [`Response::product_type_vdos`] is a [`PassiveCableVdo`][`super::PassiveCableVdo`].
    PassiveCable,

    /// The Product is a cable that incorporates signal conditioning circuits.
    ///
    /// The first item in [`Response::product_type_vdos`] is a [`ActiveCableVdo1`][`super::ActiveCableVdo1`].
    /// The second item is a [`ActiveCableVdo2`][`super::ActiveCableVdo2`].
    ActiveCable,

    /// The Product is a `VCONN`-powered USB device.
    ///
    /// The first item in [`Response::product_type_vdos`] is a [`VpdVdo`][`super::VpdVdo`].
    Vpd,
}

//! [`ResponseVdos`] contains the response VDOs to a Discover Identity Command targeting SOP'.

use crate::vdm::structured::command::discover_identity::{
    ActiveCableVdo1, ActiveCableVdo2, CertStatVdo, PassiveCableVdo, ProductVdo, VpdVdo,
};

pub mod id_header_vdo;

pub use id_header_vdo::IdHeaderVdo;

/// The response VDOs to a Discover Identity Command using `SOP'`.
///
/// See PD spec 6.4.4.3.1 Discover Identity, table 6.16 Discover Identity Command response.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ResponseVdos {
    /// Information corresponding to the Product.
    ///
    /// To get an SOP'-specific ID Header VDO, use [`Self::id()`].
    pub id: crate::vdm::structured::command::discover_identity::IdHeaderVdo,

    /// The XID assigned by the USB-IF to the product.
    pub cert_stat: CertStatVdo,

    /// Identity information relating to the product.
    pub product: ProductVdo,

    /// The Product-specific VDOs.
    ///
    /// These are determined by [`IdHeaderVdo::product_type`] during parsing.
    pub product_type_vdos: ProductTypeVdos,
}

impl ResponseVdos {
    /// Gets the SOP'-specific ID Header VDO from this response.
    pub fn id(&self) -> IdHeaderVdo {
        IdHeaderVdo {
            usb_vendor_id: self.id.usb_vendor_id,
            connector_type: self.id.connector_type,
            modal_operation_supported: self.id.modal_operation_supported,
            product_type: self.product_type_vdos.into(),
            usb_communication_capable_as_usb_device: self.id.usb_communication_capable_as_usb_device,
            usb_communication_capable_as_usb_host: self.id.usb_communication_capable_as_usb_host,
        }
    }
}

/// The Product Type VDOs, parsed based on [`IdHeaderVdo::product_type`].
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ProductTypeVdos {
    /// No other Product Type is appropriate.
    NotACablePlugVpd,

    /// The Product is a cable that does not incorporate signal conditioning circuits.
    PassiveCable(PassiveCableVdo),

    /// The Product is a cable that incorporates signal conditioning circuits.
    ActiveCable(ActiveCableVdo1, ActiveCableVdo2),

    /// The Product is a `VCONN`-powered USB device.
    Vpd(VpdVdo),
}

//! [`ResponseVdos`] contains the response VDOs to a Discover Identity Command targeting SOP.

use crate::vdm::structured::command::discover_identity::{CertStatVdo, DfpVdo, ProductVdo, UfpVdo};

pub mod id_header_vdo;

pub use id_header_vdo::IdHeaderVdo;

/// The response VDOs to a Discover Identity Command using SOP.
///
/// See PD spec 6.4.4.3.1 Discover Identity, table 6.16 Discover Identity Command response.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ResponseVdos {
    /// Information corresponding to the Product.
    ///
    /// To get an SOP-specific ID Header VDO, use [`Self::id()`].
    pub id: crate::vdm::structured::command::discover_identity::IdHeaderVdo,

    /// The XID assigned by the USB-IF to the product.
    pub cert_stat: CertStatVdo,

    /// Identity information relating to the product.
    pub product: ProductVdo,

    /// The Product-specific DFP VDOs.
    ///
    /// These are determined by the [`IdHeaderVdo::product_type_dfp`] field during
    /// parsing.
    pub dfp_product_type_vdos: DfpProductTypeVdos,

    /// The Product-specific UFP VDOs.
    ///
    /// These are determined by the [`IdHeaderVdo::product_type_ufp`] field during
    /// parsing.
    pub ufp_product_type_vdos: UfpProductTypeVdos,
}

impl ResponseVdos {
    /// Gets the SOP-specific ID Header VDO from this response.
    pub fn id(&self) -> IdHeaderVdo {
        IdHeaderVdo {
            usb_vendor_id: self.id.usb_vendor_id,
            connector_type: self.id.connector_type,
            product_type_dfp: self.dfp_product_type_vdos.into(),
            modal_operation_supported: self.id.modal_operation_supported,
            product_type_ufp: self.ufp_product_type_vdos.into(),
            usb_communication_capable_as_usb_device: self.id.usb_communication_capable_as_usb_device,
            usb_communication_capable_as_usb_host: self.id.usb_communication_capable_as_usb_host,
        }
    }
}

/// The Product Type DFP VDOs, parsed based on [`IdHeaderVdo::product_type_dfp`].
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DfpProductTypeVdos {
    /// This is not a DFP.
    NotADfp,

    /// The product is a PDUSB Hub.
    Hub(DfpVdo),

    /// The product is a PDUSB Host or a PDUSB host that supports one or more Alternate
    /// Modes as an AMC.
    Host(DfpVdo),

    /// The product is a charger / power brick.
    Charger(DfpVdo),
}

/// The Product Type UFP VDOs, parsed based on [`IdHeaderVdo::product_type_ufp`].
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UfpProductTypeVdos {
    /// This is not a UFP.
    NotAUfp,

    /// The product is a PDUSB Hub.
    Hub(UfpVdo),

    /// The product is a PDUSB Device other than a Hub.
    Peripheral(UfpVdo),

    /// The product is a PSD, e.g., power bank.
    Psd,
}

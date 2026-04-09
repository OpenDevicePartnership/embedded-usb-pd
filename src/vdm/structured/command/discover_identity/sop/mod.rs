//! [`Response`] contains the response to a Discover Identity Command targetting SOP.

use crate::vdm::structured::{
    command::discover_identity::{CertStatVdo, ProductTypeVdo, ProductVdo},
    Header,
};

pub mod id_header_vdo;

pub use id_header_vdo::IdHeaderVdo;

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

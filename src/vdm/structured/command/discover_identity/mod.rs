//! The Discover Identity Command is used to identify a Port Partner and the Responder (Cable Plug or VPD).
//!
//! See PD spec 6.4.4.3.1 Discover Identity.

pub mod active_cable_vdo;
pub mod dfp_vdo;
pub mod id_header_vdo;
pub mod passive_cable_vdo;
pub mod product_vdo;
pub mod sop;
pub mod sop_prime;
pub mod ufp_vdo;
pub mod vpd_vdo;

pub use active_cable_vdo::{ActiveCableVdo1, ActiveCableVdo2};
pub use dfp_vdo::DfpVdo;
pub use id_header_vdo::IdHeaderVdo;
pub use passive_cable_vdo::PassiveCableVdo;
pub use product_vdo::ProductVdo;
pub use ufp_vdo::UfpVdo;
pub use vpd_vdo::VpdVdo;

/// Identifies the device as either a USB Type-C receptacle of a USB Type-C plug.
///
/// See PD spec 6.4.4.3.1.1.7 Connector Type Field.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ConnectorType {
    /// The device is a USB Type-C receptacle.
    Receptacle,

    /// The device is a USB Type-C plug.
    Plug,
}

impl TryFrom<u8> for ConnectorType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0b10 => Ok(Self::Receptacle),
            0b11 => Ok(Self::Plug),
            _ => Err(()),
        }
    }
}

/// Contains the XID assigned by USB-IF to the product before certification in binary format.
///
/// See PD spec 6.4.4.3.1.2 Cert Stat VDO, table 6.37 Cert Stat VDO.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct CertStatVdo(pub u32);

/// An unspecified Product Type VDO in the Product Type VDO(s) of the Discover
/// Identity Command response.
///
/// The type of this VDO is determined by the ID Header VDO and whether targeting
/// SOP or SOP'.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ProductTypeVdo(pub u32);

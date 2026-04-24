//! [`ProductVdo`] contains identity information relating to the product.
//!
//! See PD spec 6.4.4.3.1.3 Product VDO, table 6.38 Product VDO.

use crate::usb::{Bcd, ProductId};

/// The Product VDO contains identity information relating to the product.
///
/// See PD spec 6.4.4.3.1.3 Product VDO, table 6.38 Product VDO.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ProductVdo {
    /// The USB Product ID, as defined by the USB 2.0 / USB 3.2 specifications.
    pub usb_product_id: ProductId,

    /// The USB Device Release Number, as defined by the USB 2.0 / USB 3.2 specifications.
    pub bcd_device: Bcd,
}

bitfield::bitfield! {
    /// The Raw value of a [`ProductVdo`], before parsing bitfields.
    #[derive(Copy, Clone, PartialEq, Eq)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub struct Raw(u32);
    impl Debug;

    /// See [`ProductVdo::usb_product_id`].
    pub u16, usb_product_id, set_usb_product_id: 15, 0;

    /// See [`ProductVdo::bcd_device`].
    pub u16, bcd_device, set_bcd_device: 31, 16
}

impl From<Raw> for ProductVdo {
    fn from(raw: Raw) -> Self {
        Self {
            usb_product_id: ProductId(raw.usb_product_id()),
            bcd_device: Bcd(raw.bcd_device()),
        }
    }
}

impl From<u32> for ProductVdo {
    fn from(value: u32) -> Self {
        Raw(value).into()
    }
}

impl From<[u8; 4]> for ProductVdo {
    fn from(bytes: [u8; 4]) -> Self {
        u32::from_le_bytes(bytes).into()
    }
}

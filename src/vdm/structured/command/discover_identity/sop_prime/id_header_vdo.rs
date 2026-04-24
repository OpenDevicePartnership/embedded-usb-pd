use super::ProductTypeVdos;
use crate::vdm::structured::command::discover_identity::ConnectorType;

/// The ID Header VDO contains information corresponding to the Power Delivery Product.
///
/// This type differs from [`crate::vdm::structured::command::discover_identity::IdHeaderVdo`]
/// in that it contains the product type fields, which are encoded into the [`ResponseVdos::product_type_vdos`]
/// field. This type is meant to be parsed directly from the raw VDO.
///
/// See PD spec 6.4.4.3.1.1 ID Header VDO, table 6.3.3 ID Header VDO.
///
/// [`ResponseVdos::product_type_vdos`]: super::ResponseVdos::product_type_vdos
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
    /// The value of this type changes how [`ResponseVdos::product_type_vdos`] is interpreted.
    ///
    /// [`ResponseVdos::product_type_vdos`]: super::ResponseVdos::product_type_vdos
    pub product_type: ProductType,

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

    /// See [`IdHeaderVdo::modal_operation_supported`].
    pub bool, modal_operation_supported, set_modal_operation_supported: 26;

    /// See [`IdHeaderVdo::product_type`].
    pub u8, product_type, set_product_type: 29, 27;

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
    InvalidProductType,
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
            modal_operation_supported: raw.modal_operation_supported(),
            product_type: raw
                .product_type()
                .try_into()
                .map_err(|()| ParseIdHeaderVdoError::InvalidProductType)?,
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
    fn from(id_header_vdo: IdHeaderVdo) -> Self {
        Self {
            usb_vendor_id: id_header_vdo.usb_vendor_id,
            connector_type: id_header_vdo.connector_type,
            modal_operation_supported: id_header_vdo.modal_operation_supported,
            usb_communication_capable_as_usb_device: id_header_vdo.usb_communication_capable_as_usb_device,
            usb_communication_capable_as_usb_host: id_header_vdo.usb_communication_capable_as_usb_host,
        }
    }
}

/// The `SOP'` Product Type (Cable Plug/VPD) field indicates the type of Product
/// when the Product is a Cable Plug or VPD, whether a VDO will be returned, and
/// if so, the type of VDO to be returned.
///
/// See PD spec 6.4.4.3.1.1.4 Product Type (Cable Plug), table 6.35 Product Types (Cable Plug/VPD).
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ProductType {
    /// No other Product Type is appropriate.
    ///
    /// [`ResponseVdos::product_type_vdos`] is empty.
    ///
    /// [`ResponseVdos::product_type_vdos`]: super::ResponseVdos::product_type_vdos
    NotACablePlugVpd,

    /// The Product is a cable that does not incorporate signal conditioning circuits.
    ///
    /// The first item in [`ResponseVdos::product_type_vdos`] is a [`PassiveCableVdo`][`super::PassiveCableVdo`].
    ///
    /// [`ResponseVdos::product_type_vdos`]: super::ResponseVdos::product_type_vdos
    PassiveCable,

    /// The Product is a cable that incorporates signal conditioning circuits.
    ///
    /// The first item in [`ResponseVdos::product_type_vdos`] is a [`ActiveCableVdo1`][`super::ActiveCableVdo1`].
    /// The second item is a [`ActiveCableVdo2`][`super::ActiveCableVdo2`].
    ///
    /// [`ResponseVdos::product_type_vdos`]: super::ResponseVdos::product_type_vdos
    ActiveCable,

    /// The Product is a `VCONN`-powered USB device.
    ///
    /// The first item in [`ResponseVdos::product_type_vdos`] is a [`VpdVdo`][`super::VpdVdo`].
    ///
    /// [`ResponseVdos::product_type_vdos`]: super::ResponseVdos::product_type_vdos
    Vpd,
}

impl TryFrom<u8> for ProductType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0b000 => Ok(Self::NotACablePlugVpd),
            0b011 => Ok(Self::PassiveCable),
            0b100 => Ok(Self::ActiveCable),
            0b110 => Ok(Self::Vpd),
            _ => Err(()),
        }
    }
}

impl From<ProductTypeVdos> for ProductType {
    fn from(product_type_vdos: ProductTypeVdos) -> Self {
        match product_type_vdos {
            ProductTypeVdos::NotACablePlugVpd => Self::NotACablePlugVpd,
            ProductTypeVdos::PassiveCable(_) => Self::PassiveCable,
            ProductTypeVdos::ActiveCable(_, _) => Self::ActiveCable,
            ProductTypeVdos::Vpd(_) => Self::Vpd,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod product_type {
        use super::*;

        #[test]
        fn all_valid_variants() {
            let cases: [(u8, ProductType); 4] = [
                (0b000, ProductType::NotACablePlugVpd),
                (0b011, ProductType::PassiveCable),
                (0b100, ProductType::ActiveCable),
                (0b110, ProductType::Vpd),
            ];
            for (raw, expected) in cases {
                assert_eq!(ProductType::try_from(raw), Ok(expected), "raw={raw}");
            }
        }

        #[test]
        fn invalid_values() {
            for v in [0b001, 0b010, 0b101, 0b111] {
                assert!(ProductType::try_from(v).is_err(), "raw={v} should be invalid");
            }
        }
    }
}

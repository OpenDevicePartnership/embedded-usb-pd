use crate::vdm::structured::command::discover_identity::ConnectorType;

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

bitfield::bitfield! {
    /// The raw value of an [`IdHeaderVdo`], before parsing enumerations and bitfields.
    #[derive(Copy, Clone, PartialEq, Eq)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub struct Raw(u32);
    impl Debug;
    pub u16, usb_vendor_id, set_usb_vendor_id: 15, 0;
    pub u8, connector_type, set_connector_type: 22, 21;
    pub bool, modal_operation_supported, set_modal_operation_supported: 26;
    pub u8, product_type, set_product_type: 29, 27;
    pub bool, usb_communication_capable_as_usb_device, set_usb_communication_capable_as_usb_device: 30;
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

impl TryFrom<u8> for ProductType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0b000 => Ok(Self::NotACablePlugVpd),
            0b001 => Ok(Self::PassiveCable),
            0b010 => Ok(Self::ActiveCable),
            0b011 => Ok(Self::Vpd),
            _ => Err(()),
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
                (0b001, ProductType::PassiveCable),
                (0b010, ProductType::ActiveCable),
                (0b011, ProductType::Vpd),
            ];
            for (raw, expected) in cases {
                assert_eq!(ProductType::try_from(raw), Ok(expected), "raw={raw}");
            }
        }

        #[test]
        fn invalid_values() {
            for v in [0b100, 0b101, 0b110, 0b111] {
                assert!(ProductType::try_from(v).is_err(), "raw={v} should be invalid");
            }
        }
    }
}

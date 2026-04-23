use crate::vdm::structured::command::discover_identity::ConnectorType;

/// The ID Header VDO contains information corresponding to the Power Delivery Product.
///
/// See PD spec 6.4.4.3.1.1 ID Header VDO, table 6.3.3 ID Header VDO.
///
/// This type differs from [`sop::IdHeaderVdo`] and [`sop_prime::IdHeaderVdo`] in
/// that it does not contain the product type fields. These fields are encoded into
/// the [`sop::ResponseVdos`] and [`sop_prime::ResponseVdos`] instead, in their
/// [`sop::ResponseVdos::dfp_product_type_vdos`], [`sop::ResponseVdos::ufp_product_type_vdos`],
/// and [`sop_prime::ResponseVdos::product_type_vdos`] fields, which provide a more
/// ergonomic API for accessing the Product Type VDOs.
///
/// This type is not meant to be parsed directly; use the [`From`] implementations
/// on [`sop::IdHeaderVdo`] and [`sop_prime::IdHeaderVdo`] instead.
///
/// [`sop::IdHeaderVdo`]: crate::vdm::structured::command::discover_identity::sop::IdHeaderVdo
/// [`sop_prime::IdHeaderVdo`]: crate::vdm::structured::command::discover_identity::sop_prime::IdHeaderVdo
/// [`sop::ResponseVdos`]: crate::vdm::structured::command::discover_identity::sop::ResponseVdos
/// [`sop_prime::ResponseVdos`]: crate::vdm::structured::command::discover_identity::sop_prime::ResponseVdos
/// [`sop::ResponseVdos::dfp_product_type_vdos`]: crate::vdm::structured::command::discover_identity::sop::ResponseVdos::dfp_product_type_vdos
/// [`sop::ResponseVdos::ufp_product_type_vdos`]: crate::vdm::structured::command::discover_identity::sop::ResponseVdos::ufp_product_type_vdos
/// [`sop_prime::ResponseVdos::product_type_vdos`]: crate::vdm::structured::command::discover_identity::sop_prime::ResponseVdos::product_type_vdos
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

    /// Whether or not the Port has a USB Device Capability.
    pub usb_communication_capable_as_usb_device: bool,

    /// Whether or not the Port has a USB Host Capability.
    pub usb_communication_capable_as_usb_host: bool,
}

/// The Standard or Vendor ID (SVID) field Shall contain either a 16-bit USB Standard
/// ID value (SID) or the 16-bit Vendor ID (VID) assigned to the vendor by the USB-IF.
///
/// See PD spec 6.4.4.2.1 SVID.
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Svid(pub u16);

impl Svid {
    /// The Standard ID allocated to the PD specification by USB-IF.
    pub const PD: Self = Self(0xFF00);

    /// The Standard ID allocated to DisplayPort Type-C (DPTC) by USB-IF.
    pub const DISPLAY_PORT_TYPE_C: Self = Self(0xFF01);

    /// The Vendor ID assigned to Thunderbolt by USB-IF.
    pub const THUNDERBOLT: Self = Self(0x8087);
}

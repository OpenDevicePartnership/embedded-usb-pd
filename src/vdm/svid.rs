/// Standard or Vendor ID (SVID) newtype, see PD spec 6.4.4.2.1
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Svid(pub u16);

impl Svid {
    pub const PD: Self = Self(0xFF00);
    pub const DISPLAY_PORT_TYPE_C: Self = Self(0xFF01);
    pub const USB4: Self = Self(0xFF03);
    pub const THUNDERBOLT: Self = Self(0x8087);
}

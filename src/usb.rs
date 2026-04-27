//! Universal Serial Bus (USB) related types and traits from the USB 2.0 and 3.2
//! specifications.

/// A Binary-Coded Decimal (BCD) format as defined by the USB 2.0 specification.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Bcd(pub u16);

impl Bcd {
    /// Parse the BCD value into its major, minor, and subminor components in the
    /// format `jj.m.n` where
    /// - `jj` is the major version (2 nibbles)
    /// - `m` is the minor version (1 nibble)
    /// - `n` is the subminor version (1 nibble)
    pub const fn jjmn(&self) -> (u8, u8, u8) {
        let jj = (self.0 >> 8) as u8;
        let m = ((self.0 >> 4) & 0xF) as u8;
        let n = (self.0 & 0xF) as u8;
        (jj, m, n)
    }
}

/// The USB Product ID as assigned by the USB-IF.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ProductId(pub u16);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bcd_jjmn() {
        assert_eq!(Bcd(0x1234).jjmn(), (0x12, 0x3, 0x4));
        assert_eq!(Bcd(0xFEDC).jjmn(), (0xFE, 0xD, 0xC));
    }
}

//! Types for the `pdinfo` command to query the PD controller for information on the current port and status.

/// The status for the switches on the power path.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct PowerPathStatus(inner::PowerPathStatus);

impl PowerPathStatus {
    /// Creates a new [`PowerPathStatus`] instance with the given status bits.
    pub fn new(ext_vbus_sw_en: bool, int_vbus_sw_en: bool) -> Self {
        Self(inner::PowerPathStatus::new(
            ext_vbus_sw_en.into(),
            int_vbus_sw_en.into(),
        ))
    }

    /// Creates a new [`PowerPathStatus`] instance with no switches enabled
    pub const fn none() -> Self {
        Self(inner::PowerPathStatus(0))
    }

    /// Returns true if the external VBUS switch is enabled.
    pub fn ext_vbus_sw_en(&self) -> bool {
        self.0.ext_vbus_sw_en() == 1
    }

    /// Returns true if the internal VBUS switch is enabled.
    pub fn int_vbus_sw_en(&self) -> bool {
        self.0.int_vbus_sw_en() == 1
    }
}

impl core::fmt::Debug for PowerPathStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // Implementing Debug manually to avoid the `.0: 123` field in the bitfield debug impl.
        f.debug_struct("PowerPathStatus")
            .field("ext_vbus_sw_en", &self.ext_vbus_sw_en())
            .field("int_vbus_sw_en", &self.int_vbus_sw_en())
            .finish()
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for PowerPathStatus {
    fn format(&self, fmt: defmt::Formatter) {
        // Implementing Format manually to avoid the `.0: 123` field in the bitfield debug impl.
        defmt::write!(
            fmt,
            "PowerPathStatus {{ ext_vbus_sw_en: {}, int_vbus_sw_en: {} }}",
            self.ext_vbus_sw_en(),
            self.int_vbus_sw_en()
        )
    }
}

/// The Alternate Mode the controller is in.
///
/// Alternate Modes are vendor-defined therefore the meaning of the `userX` getters, such as
/// [`AltMode::user0()`], are defined by the particular driver. Additionally, common alternate
/// modes are defined and should be used by drivers that support them.
#[derive(Clone, Copy)]
pub struct AltMode(inner::AltMode);

impl core::fmt::Debug for AltMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // Implementing Debug manually to avoid the `.0: 123` field in the bitfield debug impl.
        f.debug_struct("AltMode")
            .field("user0", &self.user0())
            .field("user1", &self.user1())
            .field("user2", &self.user2())
            .field("user3", &self.user3())
            .field("display_port", &self.display_port())
            .field("thunderbolt", &self.thunderbolt())
            .field("usb4", &self.usb4())
            .finish()
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for AltMode {
    fn format(&self, fmt: defmt::Formatter) {
        // Implementing Format manually to avoid the `.0: 123` field in the bitfield debug impl.
        defmt::write!(
            fmt,
            "AltMode {{ user0: {}, user1: {}, user2: {}, user3: {}, display_port: {}, thunderbolt: {}, usb4: {} }}",
            self.user0(),
            self.user1(),
            self.user2(),
            self.user3(),
            self.display_port(),
            self.thunderbolt(),
            self.usb4(),
        )
    }
}

impl AltMode {
    /// Creates a new [`AltMode`] instance with the given status bits.
    pub fn new(
        user0: bool,
        user1: bool,
        user2: bool,
        user3: bool,
        display_port: bool,
        thunderbolt: bool,
        usb4: bool,
    ) -> Self {
        Self(inner::AltMode::new(
            user0.into(),
            user1.into(),
            user2.into(),
            user3.into(),
            display_port.into(),
            thunderbolt.into(),
            usb4.into(),
        ))
    }

    /// Creates a new [`AltMode`] instance with no alternate mode active.
    pub const fn none() -> Self {
        Self(inner::AltMode(0))
    }

    /// User-defined alternate mode 0 is active.
    pub fn user0(&self) -> bool {
        self.0.user0() == 1
    }

    /// User-defined alternate mode 1 is active.
    pub fn user1(&self) -> bool {
        self.0.user1() == 1
    }

    /// User-defined alternate mode 2 is active.
    pub fn user2(&self) -> bool {
        self.0.user2() == 1
    }

    /// User-defined alternate mode 3 is active.
    pub fn user3(&self) -> bool {
        self.0.user3() == 1
    }

    /// DisplayPort alternate mode is active.
    pub fn display_port(&self) -> bool {
        self.0.display_port() == 1
    }

    /// Thunderbolt alternate mode is active.
    pub fn thunderbolt(&self) -> bool {
        self.0.thunderbolt() == 1
    }

    /// USB4 alternate mode is active.
    pub fn usb4(&self) -> bool {
        self.0.usb4() == 1
    }
}

mod inner {
    use bitfield::bitfield;

    bitfield! {
        #[derive(Clone, Copy, Default, PartialEq, Eq)]
        pub struct PowerPathStatus(u8);
        impl new;
        u8;
        pub ext_vbus_sw_en, set_ext_vbus_sw_en: 0, 0;
        pub int_vbus_sw_en, set_int_vbus_sw_en: 1, 1;
    }

    bitfield! {
        #[derive(Clone, Copy)]
        pub struct AltMode(u8);
        impl new;
        u8;
        pub user0, set_user0: 0, 0;
        pub user1, set_user1: 1, 1;
        pub user2, set_user2: 2, 2;
        pub user3, set_user3: 3, 3;
        pub display_port, set_display_port: 4, 4;
        pub thunderbolt, set_thunderbolt: 5, 5;
        pub usb4, set_usb4: 6, 6;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    extern crate std;

    #[test]
    fn power_path_status_debug_contains_type_and_field_names() {
        let status = PowerPathStatus(inner::PowerPathStatus(0b0000_0001));
        let debug = std::format!("{:?}", status);
        assert!(debug.contains("PowerPathStatus"));
        assert!(debug.contains("ext_vbus_sw_en:"));
        assert!(debug.contains("int_vbus_sw_en:"));
    }

    #[test]
    fn alt_mode_debug_contains_type_and_field_names() {
        let status = AltMode(inner::AltMode(0b0000_0001));
        let debug = std::format!("{:?}", status);
        assert!(debug.contains("AltMode"));
        assert!(debug.contains("user0:"));
        assert!(debug.contains("user1:"));
        assert!(debug.contains("user2:"));
        assert!(debug.contains("user3:"));
        assert!(debug.contains("display_port:"));
        assert!(debug.contains("thunderbolt:"));
        assert!(debug.contains("usb4:"));
    }
}

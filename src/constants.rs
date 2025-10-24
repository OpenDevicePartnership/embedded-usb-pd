//! Constants for USB Power Delivery (USB PD) protocol

use wrappers::{Maximum, Minimum, Nominal, Range};

/// Source transition request time in milliseconds for SPR mode.
///
/// This is `tPSTransition` for SPR mode in the PD spec.
pub const T_PS_TRANSITION_SPR_MS: Range<u16> = Range {
    minimum: Minimum(450),
    nominal: Nominal(500),
    maximum: Maximum(550),
};

/// Source transition request time in milliseconds for EPR mode.
///
/// This is `tPSTransition` for EPR mode in the PD spec.
pub const T_PS_TRANSITION_EPR_MS: Range<u16> = Range {
    minimum: Minimum(830),
    nominal: Nominal(925),
    maximum: Maximum(1020),
};

pub mod wrappers {
    /// A minimum value.
    ///
    /// Typically paired with and less than a [`Maximum`], such as with a [`Range`].
    #[derive(Debug, Copy, Clone, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub struct Minimum<T>(pub T);

    impl<T: core::fmt::Display> core::fmt::Display for Minimum<T> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "Minimum({})", self.0)
        }
    }

    /// A nominal value.
    ///
    /// Typically paired with and between a [`Minimum`] and [`Maximum`], such as with a [`Range`].
    #[derive(Debug, Copy, Clone, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub struct Nominal<T>(pub T);

    impl<T: core::fmt::Display> core::fmt::Display for Nominal<T> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "Nominal({})", self.0)
        }
    }

    /// A maximum value.
    ///
    /// Typically paired with and greater than a [`Minimum`], such as with a [`Range`].
    #[derive(Debug, Copy, Clone, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub struct Maximum<T>(pub T);

    impl<T: core::fmt::Display> core::fmt::Display for Maximum<T> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "Maximum({})", self.0)
        }
    }

    /// A range of timing values with an inclusive minimum and maximum.
    #[derive(Debug, Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct Range<T> {
        /// The lower, inclusive bound of the timing range.
        pub minimum: Minimum<T>,

        /// The nominal value within the timing range.
        pub nominal: Nominal<T>,

        /// The upper, inclusive bound of the timing range.
        pub maximum: Maximum<T>,
    }
}

//! VCONN-Powered USB Devices (VPDs) are Ports that consume power from a DFP.
//!
//! See PD spec 6.4.4.3.1.9 VCONN Powered USB Device VDO.

use crate::vdm::structured::command::discover_identity::ProductTypeVdo;

/// A `VCONN`-Powered USB Device (VPD).
///
/// Sent based on the value of [`sop_prime::IdHeaderVdo::product_type`][super::sop_prime::IdHeaderVdo::product_type].
///
/// See PD spec 6.4.4.3.1.9 VCONN Powered USB Device VDO, table 6.44 VPD VDO.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct VpdVdo {
    /// Whether the VPD supports Charge Through.
    pub charge_through_support: bool,

    /// The impedance through ground the VPD adds in series between the Source and Sink.
    pub ground_impedance: GroundImpedance,

    /// The impedance through `VBUS` the VPD adds in series between the Source and Sink.
    pub vbus_impedance: VbusImpedance,

    /// The level of current the VPD can pass through when Charge Through is active.
    pub charge_through_current_support: ChargeThroughCurrentSupport,

    /// The maximum voltage that a Sink shall negotiate through the VPD Charge Through
    /// Port as part of an Explicit Contract.
    pub maximum_vbus_voltage: MaximumVbusVoltage,

    /// The FW version assigned by the VID owner.
    pub fw_version: u8,

    /// The HW version assigned by the VID owner.
    pub hw_version: u8,
}

/// Errors that can occur when parsing a [`VpdVdo`] from its raw value.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ParseVpdVdoError {
    /// [`VpdVdo::charge_through_current_support`] contains an invalid value.
    InvalidChargeThroughCurrentSupport,

    /// [`VpdVdo::maximum_vbus_voltage`] contains an invalid value.
    InvalidMaximumVbusVoltage,
}

impl TryFrom<Raw> for VpdVdo {
    type Error = ParseVpdVdoError;

    fn try_from(raw: Raw) -> Result<Self, Self::Error> {
        Ok(Self {
            charge_through_support: raw.charge_through_support(),
            ground_impedance: GroundImpedance(raw.ground_impedance()),
            vbus_impedance: VbusImpedance(raw.vbus_impedance()),
            charge_through_current_support: raw
                .charge_through_current_support()
                .try_into()
                .map_err(|_| ParseVpdVdoError::InvalidChargeThroughCurrentSupport)?,
            maximum_vbus_voltage: raw
                .maximum_vbus_voltage()
                .try_into()
                .map_err(|_| ParseVpdVdoError::InvalidMaximumVbusVoltage)?,
            fw_version: raw.fw_version(),
            hw_version: raw.hw_version(),
        })
    }
}

impl TryFrom<u32> for VpdVdo {
    type Error = ParseVpdVdoError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Raw(value).try_into()
    }
}

impl TryFrom<ProductTypeVdo> for VpdVdo {
    type Error = ParseVpdVdoError;

    fn try_from(value: ProductTypeVdo) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

bitfield::bitfield! {
    /// The raw value of a [`VpdVdo`], before parsing enumerations and bitfields.
    #[derive(Copy, Clone, PartialEq, Eq)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub struct Raw(u32);
    impl Debug;

    /// See [`VpdVdo::charge_through_support`].
    pub bool, charge_through_support, set_charge_through_support: 0;

    /// See [`VpdVdo::ground_impedance`].
    pub u8, ground_impedance, set_ground_impedance: 6, 1;

    /// See [`VpdVdo::vbus_impedance`].
    pub u8, vbus_impedance, set_vbus_impedance: 12, 7;

    /// See [`VpdVdo::charge_through_current_support`].
    pub u8, charge_through_current_support, set_charge_through_current_support: 14, 14;

    /// See [`VpdVdo::maximum_vbus_voltage`].
    pub u8, maximum_vbus_voltage, set_maximum_vbus_voltage: 16, 15;

    /// See [`VpdVdo::fw_version`].
    pub u8, fw_version, set_fw_version: 27, 24;

    /// See [`VpdVdo::hw_version`].
    pub u8, hw_version, set_hw_version: 31, 28;
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
/// Ground impedance through the VPD in 2mΩ increments.
pub struct GroundImpedance(pub u8);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
/// `VBUS` impedance through the VPD in 2mΩ increments.
pub struct VbusImpedance(pub u8);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ChargeThroughCurrentSupport {
    /// 3A
    ThreeAmps,

    /// 5A
    FiveAmps,
}

impl TryFrom<u8> for ChargeThroughCurrentSupport {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0b0 => Ok(Self::ThreeAmps),
            0b1 => Ok(Self::FiveAmps),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
/// The maximum voltage that a Sink shall negotiate through the VPD Charge Through
/// Port as part of an Explicit Contract.
pub enum MaximumVbusVoltage {
    /// 20V
    TwentyVolts,

    /// 30V
    ThirtyVolts,

    /// 40V
    FortyVolts,

    /// 50V
    FiftyVolts,
}

impl TryFrom<u8> for MaximumVbusVoltage {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0b00 => Ok(Self::TwentyVolts),
            0b01 => Ok(Self::ThirtyVolts),
            0b10 => Ok(Self::FortyVolts),
            0b11 => Ok(Self::FiftyVolts),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod charge_through_current_support {
        use super::*;

        #[test]
        fn all_valid_variants() {
            let cases: [(u8, ChargeThroughCurrentSupport); 2] = [
                (0, ChargeThroughCurrentSupport::ThreeAmps),
                (1, ChargeThroughCurrentSupport::FiveAmps),
            ];
            for (raw, expected) in cases {
                assert_eq!(ChargeThroughCurrentSupport::try_from(raw), Ok(expected), "raw={raw}");
            }
        }

        #[test]
        fn invalid_values() {
            for v in 2..=255u8 {
                assert!(
                    ChargeThroughCurrentSupport::try_from(v).is_err(),
                    "raw={v} should be invalid"
                );
            }
        }
    }

    mod maximum_vbus_voltage {
        use super::*;

        #[test]
        fn all_valid_variants() {
            let cases: [(u8, MaximumVbusVoltage); 4] = [
                (0b00, MaximumVbusVoltage::TwentyVolts),
                (0b01, MaximumVbusVoltage::ThirtyVolts),
                (0b10, MaximumVbusVoltage::FortyVolts),
                (0b11, MaximumVbusVoltage::FiftyVolts),
            ];
            for (raw, expected) in cases {
                assert_eq!(MaximumVbusVoltage::try_from(raw), Ok(expected), "raw={raw}");
            }
        }

        #[test]
        fn invalid_values() {
            for v in 4..=255u8 {
                assert!(MaximumVbusVoltage::try_from(v).is_err(), "raw={v} should be invalid");
            }
        }
    }
}

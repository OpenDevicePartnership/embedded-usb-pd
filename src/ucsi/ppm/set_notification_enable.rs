use bincode::de::{Decode, Decoder};
use bincode::enc::{Encode, Encoder};
use bincode::error::{DecodeError, EncodeError};
use bitfield::bitfield;

use crate::ucsi::{CommandHeaderRaw, COMMAND_LEN};

bitfield! {
    /// Argument for SET_NOTIFICATION_ENABLE see USCI spec 6.5.5
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub(super) struct NotificationEnableRaw(u16);
    impl Debug;

    /// Notify on command complete
    pub bool, cmd_complete, set_cmd_complete: 0;
    /// Notify on external supply change
    pub bool, external_supply_change, set_external_supply_change: 1;
    /// Notify on power operation mode change
    pub bool, power_op_mode_change, set_power_op_mode_change: 2;
    /// Notify on provider capabilities change
    pub bool, provider_caps_change, set_provider_caps_change: 5;
    /// Notify on power level change
    pub bool, power_lvl_change, set_power_lvl_change: 6;
    /// Notify on PD reset complete
    pub bool, pd_reset_complete, set_pd_reset_complete: 7;
    /// Notify on connector alternate mode change
    pub bool, cam_change, set_cam_change: 8;
    /// Notify on battery charge change
    pub bool, battery_charge_change, set_battery_charge_change: 9;
    /// Notify on connector partner change
    pub bool, connector_partner_change, set_connector_partner_change: 11;
    /// Notify on power direction change
    pub bool, power_dir_change, set_power_dir_change: 12;
    /// Notify on connect change
    pub bool, connect_change, set_connect_change: 14;
    /// Notify on error
    pub bool, error, set_error: 15;
}

#[cfg(feature = "defmt")]
impl defmt::Format for NotificationEnableRaw {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "NotificationEnableRaw {{ .0: {}, \
            cmd_complete: {}, \
            external_supply_change: {}, \
            power_op_mode_change: {}, \
            provider_caps_change: {}, \
            power_lvl_change: {}, \
            pd_reset_complete: {}, \
            cam_change: {}, \
            battery_charge_change: {}, \
            connector_partner_change: {}, \
            power_dir_change: {}, \
            connect_change: {}, \
            error: {} }}",
            self.0,
            self.cmd_complete(),
            self.external_supply_change(),
            self.power_op_mode_change(),
            self.provider_caps_change(),
            self.power_lvl_change(),
            self.pd_reset_complete(),
            self.cam_change(),
            self.battery_charge_change(),
            self.connector_partner_change(),
            self.power_dir_change(),
            self.connect_change(),
            self.error()
        )
    }
}

/// Higher-level wrapper around [`SetNotificationEnableRaw`]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct NotificationEnable(NotificationEnableRaw);

impl NotificationEnable {
    /// Returns command complete notification status
    pub fn cmd_complete(&self) -> bool {
        self.0.cmd_complete()
    }

    /// Set command complete notification status
    pub fn set_cmd_complete(&mut self, cmd_complete: bool) -> &mut Self {
        self.0.set_cmd_complete(cmd_complete);
        self
    }

    /// Returns external supply change notification status
    pub fn external_supply_change(&self) -> bool {
        self.0.external_supply_change()
    }

    /// Set external supply change notification status
    pub fn set_external_supply_change(&mut self, val: bool) -> &mut Self {
        self.0.set_external_supply_change(val);
        self
    }

    /// Returns power operation mode change notification status
    pub fn power_op_mode_change(&self) -> bool {
        self.0.power_op_mode_change()
    }

    /// Set power operation mode change notification status
    pub fn set_power_op_mode_change(&mut self, val: bool) -> &mut Self {
        self.0.set_power_op_mode_change(val);
        self
    }

    /// Returns provider capabilities change notification status
    pub fn provider_caps_change(&self) -> bool {
        self.0.provider_caps_change()
    }

    /// Set provider capabilities change notification status
    pub fn set_provider_caps_change(&mut self, val: bool) -> &mut Self {
        self.0.set_provider_caps_change(val);
        self
    }

    /// Returns power level change notification status
    pub fn power_lvl_change(&self) -> bool {
        self.0.power_lvl_change()
    }

    /// Set power level change notification status
    pub fn set_power_lvl_change(&mut self, val: bool) -> &mut Self {
        self.0.set_power_lvl_change(val);
        self
    }

    /// Returns PD reset complete notification status
    pub fn pd_reset_complete(&self) -> bool {
        self.0.pd_reset_complete()
    }

    /// Set PD reset complete notification status
    pub fn set_pd_reset_complete(&mut self, val: bool) -> &mut Self {
        self.0.set_pd_reset_complete(val);
        self
    }

    /// Returns connector alt mode change notification status
    pub fn cam_change(&self) -> bool {
        self.0.cam_change()
    }

    /// Set connector alt mode change notification status
    pub fn set_cam_change(&mut self, val: bool) -> &mut Self {
        self.0.set_cam_change(val);
        self
    }

    /// Returns battery charge change notification status
    pub fn battery_charge_change(&self) -> bool {
        self.0.battery_charge_change()
    }

    /// Set battery charge change notification status
    pub fn set_battery_charge_change(&mut self, val: bool) -> &mut Self {
        self.0.set_battery_charge_change(val);
        self
    }

    /// Returns connector partner change notification status
    pub fn connector_partner_change(&self) -> bool {
        self.0.connector_partner_change()
    }

    /// Set connector partner change notification status
    pub fn set_connector_partner_change(&mut self, val: bool) -> &mut Self {
        self.0.set_connector_partner_change(val);
        self
    }

    /// Returns power direction change notification status
    pub fn power_dir_change(&self) -> bool {
        self.0.power_dir_change()
    }

    /// Set power direction change notification status
    pub fn set_power_dir_change(&mut self, val: bool) -> &mut Self {
        self.0.set_power_dir_change(val);
        self
    }

    /// Returns connect change notification status
    pub fn connect_change(&self) -> bool {
        self.0.connect_change()
    }

    /// Set connect change notification status
    pub fn set_connect_change(&mut self, val: bool) -> &mut Self {
        self.0.set_connect_change(val);
        self
    }

    /// Returns error notification status
    pub fn error(&self) -> bool {
        self.0.error()
    }

    /// Set error notification status
    pub fn set_error(&mut self, val: bool) -> &mut Self {
        self.0.set_error(val);
        self
    }

    /// Returns true if no notification is enabled
    pub fn is_empty(&self) -> bool {
        self.0 .0 == 0
    }

    /// Returns true if any status change flags are set
    pub fn any(&self) -> bool {
        !self.is_empty()
    }

    /// Returns the union of two notification enable sets
    pub fn union(&self, other: &Self) -> Self {
        NotificationEnable(NotificationEnableRaw(self.0 .0 | other.0 .0))
    }

    /// Returns the intersection of two notification enable sets
    pub fn intersection(&self, other: &Self) -> Self {
        NotificationEnable(NotificationEnableRaw(self.0 .0 & other.0 .0))
    }
}

impl From<u16> for NotificationEnable {
    fn from(raw: u16) -> Self {
        NotificationEnable(NotificationEnableRaw(raw))
    }
}

impl From<NotificationEnable> for u16 {
    fn from(enable: NotificationEnable) -> Self {
        enable.0 .0
    }
}

impl Default for NotificationEnable {
    fn default() -> Self {
        NotificationEnable(NotificationEnableRaw(0))
    }
}

impl Encode for NotificationEnable {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        Encode::encode(&self.0 .0, encoder)
    }
}

impl<Context> Decode<Context> for NotificationEnable {
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, DecodeError> {
        let raw = u16::decode(decoder)?;
        Ok(NotificationEnable::from(raw))
    }
}

/// Set notification enable command
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Args {
    /// Notification enable flags
    pub notification_enable: NotificationEnable,
}

/// Data length for the SET_NOTIFICATION_ENABLE command response
pub const RESPONSE_DATA_LEN: u8 = 0;
/// Command padding
pub const COMMAND_PADDING: usize = COMMAND_LEN - size_of::<CommandHeaderRaw>() - size_of::<NotificationEnable>();

impl Encode for Args {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.notification_enable.encode(encoder)?;
        // Padding to match the expected header size
        [0u8; COMMAND_PADDING].encode(encoder)
    }
}

impl<Context> Decode<Context> for Args {
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, DecodeError> {
        let notification_enable = NotificationEnable::decode(decoder)?;
        // Read padding
        let _padding: [u8; COMMAND_PADDING] = Decode::decode(decoder)?;
        Ok(Args { notification_enable })
    }
}

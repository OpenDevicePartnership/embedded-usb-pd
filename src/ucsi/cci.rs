use bitfield::bitfield;

bitfield! {
    /// Command status and connect change indicator, see UCSI spec 4.2
    #[derive(Copy, Clone)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    struct CciRaw(u32);
    impl Debug;

    /// End of message
    pub bool, eom, set_eom: 0;
    /// Connector change on the given port
    pub u8, connector_change, set_connector_change: 7, 1;
    /// Length of returned data
    pub u8, data_len, set_data_len: 15, 8;
    /// Vendor defined message
    pub bool, vendor_message, set_vendor_message: 16;
    /// Security request
    pub bool, security_req, set_security_req: 23;
    /// Firmware update request
    pub bool, fw_update_req, set_fw_update_req: 24;
    /// Command not supported
    pub bool, not_supported, set_not_supported: 25;
    /// Cancel complete
    pub bool, cancel_complete, set_cancel_complete: 26;
    /// PPM reset complete
    pub bool, reset_complete, set_reset_complete: 27;
    /// Busy
    pub bool, busy, set_busy: 28;
    /// Acknowledgment command
    pub bool, ack_command, set_ack_command: 29;
    /// Command error
    pub bool, error, set_error: 30;
    /// Command complete
    pub bool, cmd_complete, set_cmd_complete: 31;
}

/// Higher-level wrapper around [`CciRaw`]
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Cci(CciRaw);

impl Cci {
    /// Returns EOM status
    pub fn eom(&self) -> bool {
        self.0.eom()
    }

    /// Set EOM status
    pub fn set_eom(&mut self, eom: bool) -> &mut Self {
        self.0.set_eom(eom);
        self
    }

    /// Returns connector change port
    pub fn connector_change<T: From<u8>>(&self) -> T {
        self.0.connector_change().into()
    }

    /// Set connector change port
    pub fn set_connector_change<T: Into<u8>>(&mut self, port: T) -> &mut Self {
        self.0.set_connector_change(port.into());
        self
    }

    /// Returns data length
    pub fn data_len(&self) -> usize {
        self.0.data_len() as usize
    }

    /// Set data length
    pub fn set_data_len(&mut self, len: usize) -> &mut Self {
        self.0.set_data_len(len as u8);
        self
    }

    /// Returns vendor message status
    pub fn vendor_message(&self) -> bool {
        self.0.vendor_message()
    }

    /// Set vendor message status
    pub fn set_vendor_message(&mut self, vendor_message: bool) -> &mut Self {
        self.0.set_vendor_message(vendor_message);
        self
    }

    /// Returns security request status
    pub fn security_req(&self) -> bool {
        self.0.security_req()
    }

    /// Set security request status
    pub fn set_security_req(&mut self, security_req: bool) -> &mut Self {
        self.0.set_security_req(security_req);
        self
    }

    /// Returns firmware update request status
    pub fn fw_update_req(&self) -> bool {
        self.0.fw_update_req()
    }

    /// Set firmware update request status
    pub fn set_fw_update_req(&mut self, fw_update_req: bool) -> &mut Self {
        self.0.set_fw_update_req(fw_update_req);
        self
    }

    /// Returns command not supported status
    pub fn not_supported(&self) -> bool {
        self.0.not_supported()
    }

    /// Set command not supported status
    pub fn set_not_supported(&mut self, not_supported: bool) -> &mut Self {
        self.0.set_not_supported(not_supported);
        self
    }

    /// Returns cancel complete status
    pub fn cancel_complete(&self) -> bool {
        self.0.cancel_complete()
    }

    /// Set cancel complete status
    pub fn set_cancel_complete(&mut self, cancel_complete: bool) -> &mut Self {
        self.0.set_cancel_complete(cancel_complete);
        self
    }

    /// Returns PPM reset complete status
    pub fn reset_complete(&self) -> bool {
        self.0.reset_complete()
    }

    /// Set PPM reset complete status
    pub fn set_reset_complete(&mut self, reset_complete: bool) -> &mut Self {
        self.0.set_reset_complete(reset_complete);
        self
    }

    /// Returns busy status
    pub fn busy(&self) -> bool {
        self.0.busy()
    }

    /// Set busy status
    pub fn set_busy(&mut self, busy: bool) -> &mut Self {
        self.0.set_busy(busy);
        self
    }

    /// Returns acknowledgment command status
    pub fn ack_command(&self) -> bool {
        self.0.ack_command()
    }

    /// Set acknowledgment command status
    pub fn set_ack_command(&mut self, ack_command: bool) -> &mut Self {
        self.0.set_ack_command(ack_command);
        self
    }

    /// Returns command error status
    pub fn error(&self) -> bool {
        self.0.error()
    }

    /// Set command error status
    pub fn set_error(&mut self, error: bool) -> &mut Self {
        self.0.set_error(error);
        self
    }

    /// Returns command complete status
    pub fn cmd_complete(&self) -> bool {
        self.0.cmd_complete()
    }

    /// Set command complete status
    pub fn set_cmd_complete(&mut self, cmd_complete: bool) -> &mut Self {
        self.0.set_cmd_complete(cmd_complete);
        self
    }

    /// Create a new CCI with command complete set
    pub fn new_cmd_complete() -> Self {
        *Cci::default().set_cmd_complete(true)
    }

    /// Create a new CCI with busy set
    pub fn new_busy() -> Self {
        *Cci::default().set_busy(true)
    }

    /// Create a new CCI with reset complete set
    pub fn new_reset_complete() -> Self {
        *Cci::default().set_reset_complete(true)
    }

    /// Create a new CCI with error set
    pub fn new_error() -> Self {
        *Cci::default().set_error(true)
    }
}

impl From<u32> for Cci {
    fn from(raw: u32) -> Self {
        Cci(CciRaw(raw))
    }
}

impl From<Cci> for u32 {
    fn from(cci: Cci) -> Self {
        cci.0 .0
    }
}

impl Default for Cci {
    fn default() -> Self {
        Cci(CciRaw(0))
    }
}

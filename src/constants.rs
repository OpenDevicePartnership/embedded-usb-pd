//! Constants for USB Power Delivery (USB PD) protocol

/// Source transition request time in milliseconds for SPR mode.
///
/// This is the max value of `tPSTransition` for EPR mode.
pub const T_SRC_TRANS_REQ_SPR_MS: u16 = 550;

/// Source transition request time in milliseconds for EPR mode.
///
/// This is the max value of `tPSTransition` for EPR mode.
pub const T_SRC_TRANS_REQ_EPR_MS: u16 = 1020;

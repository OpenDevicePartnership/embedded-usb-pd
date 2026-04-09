//! Structured VDMs define the contents of bits `[0, 14]` in the VDM Header.
//!
//! See PD spec 6.4.4.2 Structured VDM.

pub mod command;
pub mod header;
mod svid;

pub use header::Header;
pub use svid::Svid;

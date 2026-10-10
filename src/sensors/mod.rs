//! The sensors module contains what's needed to read sensors.
//! Wrappers of unsafe code are defined here to avoid unsafe
//! being used everywhere. Example: sysctl.

pub mod load;
#[cfg(sndio)]
pub mod sndio;
pub mod statvfs;
pub mod sysctl;

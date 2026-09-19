//! The sensors module contains the needed to read sensors.
//! Wrappers of unsafe code are defined here to avoid unsafe
//! being used everywhere. Example: sysctl.

#[cfg(sndio)]
pub mod sndio;
pub mod statvfs;
pub mod sysctl;

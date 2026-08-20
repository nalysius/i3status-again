//! The sensors module contains the needed to read sensors.
//! Wrappers of unsafe code are defined here to avoid unsafe
//! being used everywhere. Example: sysctl.

#[cfg(target_os = "openbsd")]
pub mod sysctl;
#[cfg(target_os = "openbsd")]
pub mod openbsd_constants;

// TODO: re-export functions and structs

//! The os::battery module impements an abstraction layer to get information
//! about the battery.

use crate::sensors::sysctl::openbsd::SysctlError;
use std::convert::From;
use std::fmt;

#[cfg(target_os = "openbsd")]
pub mod openbsd;
#[cfg(target_os = "openbsd")]
pub use crate::os::battery::openbsd::*;

#[cfg(not(any(target_os = "openbsd")))]
pub mod notsupported;
#[cfg(not(any(target_os = "openbsd")))]
pub use crate::os::battery::notsupported::*;

/// The errors that can occur when querying a battery.
pub enum BatteryError {
    /// The battery wasn't found.
    BatNotFound,
    /// The operating system doesn't support this block.
    OsNotSupported,
    /// The is a compatibility error with the sysctl's structures.
    SysctlCompat,
}

impl fmt::Display for BatteryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            BatteryError::BatNotFound => write!(f, "Battery not found"),
            BatteryError::OsNotSupported => write!(f, "Not supported on this OS"),
            BatteryError::SysctlCompat => write!(f, "Sysctl compat. error"),
        }
    }
}

impl From<SysctlError> for BatteryError {
    /// Convert a SysctlError to a BatteryError.
    fn from(value: SysctlError) -> Self {
        match value {
            SysctlError::NotFound => Self::BatNotFound,
            _ => Self::SysctlCompat,
        }
    }
}

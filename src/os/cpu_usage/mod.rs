//! The os::cpu_usage module implements an abstraction layer to get the
//! CPU usage.

use crate::sensors::sysctl::openbsd::SysctlError;
use std::convert::From;
use std::fmt;

#[cfg(target_os = "openbsd")]
mod openbsd;
#[cfg(target_os = "openbsd")]
pub use crate::os::cpu_usage::openbsd::*;

#[cfg(not(any(target_os = "openbsd")))]
mod notsupported;
#[cfg(not(any(target_os = "openbsd")))]
pub use crate::os::cpu_usage::notsupported::*;

/// The errors that can occur when reading the frequency of the CPU.
pub enum CpuUsageError {
    /// The CPU could not be found.
    CpuNotFound,
    /// The OS doesn't support this block.
    OsNotSupported,
    /// A compatibility error with sysctl, probably the structures.
    SysctlCompat,
    /// The requested CPU is disabled.
    CpuDisabled,
}

impl fmt::Display for CpuUsageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            CpuUsageError::CpuNotFound => write!(f, "CPU not found"),
            CpuUsageError::OsNotSupported => write!(f, "OS not supported"),
            CpuUsageError::SysctlCompat => write!(f, "Sysctl compat. error"),
            CpuUsageError::CpuDisabled => write!(f, "CPU disabled"),
        }
    }
}

impl From<SysctlError> for CpuUsageError {
    /// Convert a SysctlError to a CpuUsageError.
    fn from(value: SysctlError) -> Self {
        match value {
            SysctlError::NotFound => Self::CpuNotFound,
            _ => Self::SysctlCompat,
        }
    }
}

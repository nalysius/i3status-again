//! The os::cpu_freq module implements an abstraction layer to get the
//! CPU frequency.

use crate::sensors::sysctl::openbsd::SysctlError;
use std::convert::From;
use std::fmt;

#[cfg(target_os = "openbsd")]
pub mod openbsd;
#[cfg(target_os = "openbsd")]
pub use crate::os::cpu_freq::openbsd::*;

/// The errors that can occur when reading the frequency of the CPU.
pub enum CpuFreqError {
    CpuNotFound,
    SysctlCompatError,
}

impl fmt::Display for CpuFreqError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            CpuFreqError::CpuNotFound => write!(f, "CPU not found"),
            CpuFreqError::SysctlCompatError => write!(f, "Sysctl compat. error"),
        }
    }
}

impl From<SysctlError> for CpuFreqError {
    /// Convert a SysctlError to a CpuError.
    fn from(value: SysctlError) -> Self {
        match value {
            SysctlError::NotFound => Self::CpuNotFound,
            _ => Self::SysctlCompatError,
        }
    }
}

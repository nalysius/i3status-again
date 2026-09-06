//! The os::memory module implements an abstraction layer to get information
//! about the memory.

use crate::sensors::sysctl::openbsd::SysctlError;
use std::convert::From;
use std::fmt;

#[cfg(target_os = "openbsd")]
pub mod openbsd;
#[cfg(target_os = "openbsd")]
pub use crate::os::memory::openbsd::*;

#[cfg(not(any(target_os = "openbsd")))]
pub mod notsupported;
#[cfg(not(any(target_os = "openbsd")))]
pub use crate::os::memory::notsupported::*;

/// The errors that can occur when querying memory information.
pub enum MemoryError {
    /// The data returned by the kernel are inconsistent.
    InvalidData,
    /// The OS doesn't support the block.
    OsNotSupported,
    /// The values are too big to fit in a number.
    Overflow,
    /// There is a compatibility error with the sysctl's structures.
    SysctlCompat,
}

impl fmt::Display for MemoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            MemoryError::InvalidData => write!(f, "Invalid data error"),
            MemoryError::OsNotSupported => write!(f, "OS not supported"),
            MemoryError::Overflow => write!(f, "Overflow error"),
            MemoryError::SysctlCompat => write!(f, "Sysctl compat. error"),
        }
    }
}

impl From<SysctlError> for MemoryError {
    /// Convert a SysctlError to a MemoryError.
    fn from(value: SysctlError) -> Self {
        match value {
            _ => Self::SysctlCompat,
        }
    }
}

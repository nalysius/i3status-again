//! The os::disk_usage module implements an abstraction layer to get information
//! about how the disk is used.

use crate::sensors::statvfs::openbsd::StatvfsError;
use std::convert::From;
use std::fmt;

#[cfg(target_os = "openbsd")]
pub mod openbsd;
#[cfg(target_os = "openbsd")]
pub use crate::os::disk_usage::openbsd::*;

#[cfg(not(any(target_os = "openbsd")))]
pub mod notsupported;
#[cfg(not(any(target_os = "openbsd")))]
pub use crate::os::battery::notsupported::*;

/// The errors that can occur when reading the disk.
pub enum DiskUsageError {
    /// The mount point wasn't found.
    MountPointNotFound,
    /// Some permissions are missing.
    MissingPermission,
    /// The operating system doesn't support this block.
    OsNotSupported,
    /// There is a compatibility error with the statvfs structure.
    StatvfsCompat,
}

impl fmt::Display for DiskUsageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DiskUsageError::MountPointNotFound => write!(f, "Mount point not found"),
            DiskUsageError::MissingPermission => write!(f, "Missing permission"),
            DiskUsageError::OsNotSupported => write!(f, "Not supported on this OS"),
            DiskUsageError::StatvfsCompat => write!(f, "Sysctl compat. error"),
        }
    }
}

impl From<StatvfsError> for DiskUsageError {
    /// Convert a StatvfsError to a DiskUsageError.
    fn from(value: StatvfsError) -> Self {
        match value {
            StatvfsError::InvalidMountPoint => Self::MountPointNotFound,
            StatvfsError::Access => Self::MissingPermission,
            StatvfsError::Other(_) => Self::StatvfsCompat,
        }
    }
}

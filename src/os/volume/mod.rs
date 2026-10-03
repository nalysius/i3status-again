//! The os::volume module implements an abstraction layer to get information
//! about the sound volume.

use std::fmt;

#[cfg(target_os = "openbsd")]
pub mod openbsd;
#[cfg(target_os = "openbsd")]
pub use crate::os::volume::openbsd::*;

#[cfg(not(any(target_os = "openbsd")))]
pub mod notsupported;
#[cfg(not(any(target_os = "openbsd")))]
pub use crate::os::volume::notsupported::*;

/// The errors that can occur when querying the volume.
pub enum VolumeError {
    /// The context (eg. sndio) is invalid / dead.
    InvalidContext,
    /// The operating system doesn't support this block.
    OsNotSupported,
    /// Backend (eg. sndio) cannot be reached.
    BackendUnreachable,
}

impl fmt::Display for VolumeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            VolumeError::InvalidContext => write!(f, "Volume context is invalid"),
            VolumeError::OsNotSupported => write!(f, "Not supported on this OS"),
            VolumeError::BackendUnreachable => write!(f, "Unreachable volume backend"),
        }
    }
}

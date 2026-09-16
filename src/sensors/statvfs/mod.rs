//! The sensors::statvfs module defines a wrapper around statvfs.

#[cfg(target_os = "openbsd")]
pub mod openbsd;

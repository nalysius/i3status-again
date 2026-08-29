//! The sensors::sysctl module defines a wrapper around sysctl.

#[cfg(target_os = "openbsd")]
pub mod openbsd;

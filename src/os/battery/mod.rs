//! The os::battery module impements an abstraction layer to get information
//! about the battery.
//! All supported OS must implement a submodule, and provide the required
//! functions. See the public functions in the openbsd submodule for reference.

#[cfg(target_os = "openbsd")]
pub mod openbsd;
#[cfg(target_os = "openbsd")]
pub use crate::os::battery::openbsd::*;

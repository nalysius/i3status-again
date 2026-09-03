//! The os::battery module impements an abstraction layer to get information
//! about the battery.
//! All supported OS must implement a submodule, and provide the required
//! functions. See the public functions in the openbsd submodule for reference.

use std::convert::From;
use std::fmt;
use crate::sensors::sysctl::openbsd::SysctlError;

#[cfg(target_os = "openbsd")]
pub mod openbsd;
#[cfg(target_os = "openbsd")]
pub use crate::os::battery::openbsd::*;

/// The errors that can occur when querying a battery.
pub enum BatteryError {
	/// The battery wasn't found.
	BatNotFound,
	/// The is a compatibility error with the sysctl's structures
	SysctlCompatError,
}

impl fmt::Display for BatteryError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match &self {
			BatteryError::BatNotFound => write!(f, "Battery not found"),
			BatteryError::SysctlCompatError => write!(f, "Sysctl compat. error")
		}
	}
}

impl From<SysctlError> for BatteryError {
	/// Convert a SysctlError to a BatteryError.
	fn from(value: SysctlError) -> Self {
		match value {
			SysctlError::NotFound => Self::BatNotFound,
			_ => Self::SysctlCompatError,
		}
	}
}

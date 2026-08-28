//! The backends module defines the blocks implementation, depending
//! on the OS. OpenBSD and Linux don't handle everything the same way,
//! like networking and battery. So, the implementation is different for
//! each OS.
//!
//! The backends, like crate::backends::openbsd::datetime::DateTimeBackend
//! are re-exported so they look like crate::backends::DateTimeBackend.
//! This way the other parts of the program don't need to know that backends
//! are OS-specific. 

use crate::bar::BlockOutput;

#[cfg(target_os = "openbsd")]
pub mod openbsd;
#[cfg(target_os = "openbsd")]
pub use crate::backends::openbsd::*;

/// An enum to represent the types of backends.
/// This way it's one type that can represent several backends.
pub enum BackendType {
	DateTime(DateTimeBackend),
	Battery(BatteryBackend),
}

impl BackendType {
	/// A shortcut function to call get_output on the backend.
	pub fn get_output(&self) -> BlockOutput {
		match &self {
			Self::DateTime(d) => d.get_output(),
			Self::Battery(b) => b.get_output(),
		}
	}
}

pub trait Backend {
	/// Main method of a backend to generate an output.
	fn get_output(&self) -> BlockOutput;
}

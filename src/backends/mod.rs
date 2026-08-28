//! The backends module defines the blocks implementation.
//!
//! Enumerations and traits are defined to make usage of backends outside
//! of the module easier. For example a Vec<BackendType> can contains several
//! backends, even if they are different.
//!
//! Backends are shared between OS, no OS-dependant code can live here. It lives
//! in the os module.

pub mod battery;
pub mod datetime;

pub use crate::backends::datetime::DateTimeBackend;
pub use crate::backends::battery::BatteryBackend;

use crate::bar::BlockOutput;

/// BackendType is an enumeration used to represent backend types.
///
/// It makes storing different backends together easier. Instead of a
/// Vec<dyn Backend>, store a Vec<BackendType>.
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

/// A simple trait to enforce some methods in every traits.
pub trait Backend {
	/// Main method of a backend to generate an output.
	fn get_output(&self) -> BlockOutput;
}

//! The backends::openbsd module contains the blocks implementations for the
//! OpenBSD OS.

pub mod battery;
pub mod datetime;

pub use crate::backends::openbsd::battery::BatteryBackend;
pub use crate::backends::openbsd::datetime::DateTimeBackend;

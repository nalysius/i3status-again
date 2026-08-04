//! The backends::openbsd module contains the blocks implementations for the
//! OpenBSD OS.

pub mod datetime;

pub use crate::backends::openbsd::datetime::DateTimeBackend;

//! The os::load module implements an OS wrapper to get the
//! load.

use crate::sensors::load::LoadError;
use std::convert::From;
use std::fmt;

// Works for all OS, no need for #[cfg(...)]
pub mod common;
pub use crate::os::load::common::*;

/// The errors that can occur when reading the load.
pub enum LoadOsError {
    /// Error are rare, so the errors found in the sensors module are grouped
    /// in a common error.
    NotAvailable,
}

impl fmt::Display for LoadOsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoadOsError::NotAvailable => write!(f, "Cannot read load"),
        }
    }
}

impl From<LoadError> for LoadOsError {
    /// Converts a LoadError to a LoadOsError.
    ///
    /// The InvalidUsage is specific to invalid values being passed to the
    /// getloadavg function, which shouldn't happen since it's done in the
    /// os module. So, it's fine to map it to NotAvailable.
    fn from(value: LoadError) -> Self {
        match value {
            LoadError::LoadNotAvailable => Self::NotAvailable,
            LoadError::InvalidUsage => Self::NotAvailable,
        }
    }
}

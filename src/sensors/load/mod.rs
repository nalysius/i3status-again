//! The sensors::load module defines a wrapper around getloadavg.

use libc::{self, c_double, c_int};
use std::fmt;

#[derive(Debug)]
pub enum LoadError {
    /// A generic error when not all (or none) data has been fetched.
    LoadNotAvailable,
    /// An error when the array doesn't have enough space to contain the data
    /// or more than 3 elements are requested.
    InvalidUsage,
}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoadError::LoadNotAvailable => write!(f, "load: not available."),
            LoadError::InvalidUsage => write!(f, "load: invalid usage."),
        }
    }
}

/// Get the average load
///
/// loadavg is a mutable array of at most 3 items.
/// nelem is the number of averages to read. Usually the length of
/// loadavg.
///
/// The averages are written in the loadavg parameter, and returns an error
/// only if there is one.
pub fn getloadavg(loadavg: &mut [c_double], nelem: c_int) -> Option<LoadError> {
    if (loadavg.len() as c_int) < nelem || nelem > 3 {
        return Some(LoadError::InvalidUsage);
    }
    if nelem == unsafe { libc::getloadavg(loadavg.as_mut_ptr(), nelem) } {
        return None;
    }
    Some(LoadError::LoadNotAvailable)
}

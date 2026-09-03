//! The os::cpu_temp module implements an abstraction layer to get the
//! CPU temperature.

use crate::sensors::sysctl::openbsd::SysctlError;
use std::convert::From;
use std::fmt;

#[cfg(target_os = "openbsd")]
pub mod openbsd;
#[cfg(target_os = "openbsd")]
pub use crate::os::cpu_temp::openbsd::*;

/// The errors that can occur when reading the temperature of the CPU.
pub enum CpuError {
    CpuNotFound,
    SysctlCompatError,
}

impl fmt::Display for CpuError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            CpuError::CpuNotFound => write!(f, "CPU not found"),
            CpuError::SysctlCompatError => write!(f, "Sysctl compat. error"),
        }
    }
}

impl From<SysctlError> for CpuError {
    /// Convert a SysctlError to a CpuError.
    fn from(value: SysctlError) -> Self {
        match value {
            SysctlError::NotFound => Self::CpuNotFound,
            _ => Self::SysctlCompatError,
        }
    }
}

/// Converts a temperature from Kelvin to Celsius.
pub fn kelvin_to_celsius(temp: u32) -> u8 {
    (temp as f64 - 273.15) as u8
}

/// Converts a temperature from Celsius to Fahrenheit.
pub fn celsius_to_fahrenheit(temp: u8) -> u8 {
    (temp as f64 * 1.8 + 32.0) as u8
}

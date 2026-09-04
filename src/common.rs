//! The common module contains structs or functions that can be used in any
//! other module.

use serde::Deserialize;
use std::fmt;

/// Temperature unit.
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TempUnit {
    #[default]
    Celsius,
    Fahrenheit,
}

impl fmt::Display for TempUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            TempUnit::Celsius => write!(f, "°C"),
            TempUnit::Fahrenheit => write!(f, "°F"),
        }
    }
}

/// Aggregation unit
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AggregatUnit {
    #[default]
    Average,
    Maximum,
}

/// Frequency unit
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum FreqUnit {
    /// Mega Hertz
    MHz,
    /// Giga Herts
    #[default]
    GHz,
}

impl fmt::Display for FreqUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            FreqUnit::MHz => write!(f, "MHz"),
            FreqUnit::GHz => write!(f, "GHz"),
        }
    }
}

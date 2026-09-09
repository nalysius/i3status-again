//! The common module contains structs or functions that can be used in any
//! other module.

use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
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

/// Memory unit
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MemoryUnit {
    MebiByte,
    #[default]
    GibiByte,
}

impl fmt::Display for MemoryUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            MemoryUnit::MebiByte => write!(f, "MiB"),
            MemoryUnit::GibiByte => write!(f, "GiB"),
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Serialize)]
#[serde(try_from = "String", into = "String")]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Color {
    pub fn red() -> Self {
        Color {
            red: 255,
            green: 0,
            blue: 0,
        }
    }
}

impl TryFrom<String> for Color {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl TryFrom<&str> for Color {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let digits = value
            .strip_prefix('#')
            .ok_or_else(|| format!("invalid color '{value}': expected leading '#'"))?;

        if digits.len() != 6 {
            return Err(format!(
                "invalid color '{value}': expected 6 hex digits (got {})",
                digits.len()
            ));
        }

        let pair = |s: &str| u8::from_str_radix(s, 16)
            .map_err(|_| format!("invalid color '{value}': invalid hex digit"));

        Ok(Color {
            red: pair(&digits[0..2])?,
            green: pair(&digits[2..4])?,
            blue: pair(&digits[4..6])?,
        })
    }
}

impl From<Color> for String {
    fn from(color: Color) -> String {
        format!("#{:02x}{:02x}{:02x}", color.red, color.green, color.blue)
    }
}

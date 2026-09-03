//! The common module contains structs or functions that can be used in any
//! other module.

use serde::Deserialize;

/// Temperature unit.
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TempUnit {
    #[default]
    Celsius,
    Fahrenheit,
}

//! The os::battery::notsupported module is the default empty implementation
//! that is used on unsupported operating sytems.

use crate::os::battery::BatteryError;

pub fn get_battery_level(bat_index: Option<u8>) -> Result<u8, BatteryError> {
    Err(BatteryError::OsNotSupported)
}

pub fn get_remaining_time(bat_index: Option<u8>) -> Result<String, BatteryError> {
    Err(BatteryError::OsNotSupported)
}

pub fn get_battery_state() -> Result<String, BatteryError> {
    Err(BatteryError::OsNotSupported)
}

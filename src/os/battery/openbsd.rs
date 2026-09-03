//! The os::battery::openbsd module implements the OpenBSD way to get information
//! about the battery. It provides the public functions required to the battery
//! backend to easily get access to the battery informations.

use crate::os::battery::BatteryError;
use crate::sensors::sysctl::openbsd::*;

/// Get the remaining capacity and last full capacity of the battery.
///
/// If bat_index is None, all the batteries are read and their values are
/// combined. If it's Some(0) for example, acpibat0 will be used.
///
/// Returns (remaining_cap, last_full_cap), in uWh.
fn get_battery_watthours(bat_index: Option<u8>) -> Result<(i64, i64), BatteryError> {
    let mut remaining_cap: i64 = 0;
    let mut last_full_cap: i64 = 0;

    // Get all the watthours for all the batteries
    let sensors = match sysctl_sensors(SensorDevType::SensorDevBattery, SensorType::SensorWatthour)
    {
        Ok(v) => v,
        Err(e) => return Err(e.into()),
    };

    let bat_id: u8 = bat_index.unwrap_or(u8::MAX);
    for (device, sensor) in sensors {
        let sensor_desc = sensor.get_desc();
        let device_id = device.get_id();

        if bat_id == device_id || bat_index.is_none() {
            if sensor_desc == "remaining capacity" {
                remaining_cap += sensor.value;
            } else if sensor_desc == "last full capacity" {
                last_full_cap += sensor.value;
            }
        }
    }
    Ok((remaining_cap, last_full_cap))
}

/// Get the power of the battery.
///
/// If bat_index is None, all the batteries are read and their values are
/// combined. If it's Some(0) for example, acpibat0 will be used.
///
/// Returns the power in uWh.
fn get_battery_power(bat_index: Option<u8>) -> Result<i64, BatteryError> {
    let mut power: i64 = 0;

    // Get all the powers for all the batteries
    let sensors = match sysctl_sensors(SensorDevType::SensorDevBattery, SensorType::SensorWatts) {
        Ok(v) => v,
        Err(e) => return Err(e.into()),
    };

    let bat_id: u8 = bat_index.unwrap_or(u8::MAX);
    for (device, sensor) in sensors {
        let sensor_desc = sensor.get_desc();
        let device_id = device.get_id();

        if bat_id == device_id || bat_index.is_none() {
            if sensor_desc == "rate" {
                power += sensor.value;
            }
        }
    }
    Ok(power)
}

/// Get the level of battery in percent.
///
/// bat_index is the index of the battery to monitor. If None, all the batteries
/// found are used to be displayed in a single block.
/// To monitor only acpibat0 for example, give Some(0).
///
/// Returns 0 if there is an error.
/// percentage = SUM(remaining capacity) / SUM(last full capacity) * 100
pub fn get_battery_level(bat_index: Option<u8>) -> Result<u8, BatteryError> {
    let (remaining_cap, last_full_cap) = get_battery_watthours(bat_index)?;
    if last_full_cap > 0 {
        return Ok((remaining_cap as f64 / last_full_cap as f64 * 100.0)
            .round()
            .clamp(0.0, 100.0) as u8);
    }
    return Ok(0);
}

/// Return whether the battery is charging.
fn is_charging() -> Result<bool, BatteryError> {
    let mib = match sysctlnametomib("hw.sensors.acpiac0.indicator0") {
        Ok(m) => m,
        Err(e) => return Err(e.into()),
    };

    let sensor = match sysctl_sensor(&mib) {
        Ok(s) => s,
        Err(e) => return Err(e.into()),
    };
    return Ok(sensor.value > 0);
}

/// Get the remaining time of the battery.
///
/// When the battery is charging, returns the time before the battery is full.
/// When the battery is discharging, returns the time before the battery is empty.
///
/// If there are several batteries, only one is used at a time. The other one
/// has a rate (power0) of 0 uW, meaning the remaining time cannot be computed
/// since the battery doesn't consume energy.
/// Instead of displaying 00:00, the time isn't displayed in this situation.
///
/// minutes_bef_full = SUM(remaining capacity) / SUM(rate) * 60
/// minutes_bef_empty = ((SUM(last full capacity) - SUM(remaining capacity)) / SUM(rate)) * 60
pub fn get_remaining_time(bat_index: Option<u8>) -> Result<String, BatteryError> {
    let power = get_battery_power(bat_index)?;
    let (remaining_cap, last_full_cap) = get_battery_watthours(bat_index)?;

    if power == 0 {
        return Ok("".to_string());
    }

    let mut minutes: u32 = if is_charging()? {
        (((last_full_cap as f64 - remaining_cap as f64) / power as f64) * 60.0) as u32
    } else {
        (remaining_cap as f64 / power as f64 * 60.0) as u32
    };
    let hours = minutes / 60;
    minutes = minutes % 60;
    return Ok(format!("{:02}:{:02}", hours, minutes));
}

/// Get the state of the battery.
///
/// Returns "CHR" if charging, "BAT" otherwise.
pub fn get_battery_state() -> Result<String, BatteryError> {
    if is_charging()? {
        return Ok("CHR".to_string());
    } else {
        return Ok("BAT".to_string());
    }
}

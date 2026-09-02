//! The os::battery::openbsd module implements the OpenBSD way to get information
//! about the battery. It provides the public functions required to the battery
//! backend to easily get access to the battery informations.

// TODO: let the sysctl errors flow to the backends as String.

use crate::sensors::sysctl::openbsd::*;

/// Get the remaining capacity and last full capacity of the battery.
///
/// If bat_index is None, all the batteries are read and their values are
/// combined. If it's Some(0) for example, acpibat0 will be used.
///
/// Returns (remaining_cap, last_full_cap), in uWh.
fn get_battery_watthours(bat_index: Option<u8>) -> (i64, i64) {
	let mut remaining_cap: i64 = 0;
	let mut last_full_cap: i64 = 0;

	// Get all the watthours for all the batteries
	let sensors_res = sysctl_sensors(SensorDevType::SensorDevBattery, SensorType::SensorWatthour);
	if sensors_res.is_err() {
		return (0, 0);
	}
	let sensors = sensors_res.unwrap();
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
	(remaining_cap, last_full_cap)
}

/// Get the level of battery in percent.
///
/// bat_index is the index of the battery to monitor. If None, all the batteries
/// found are used to be displayed in a single block.
/// To monitor only acpibat0 for example, give Some(0).
///
/// Returns 0 if there is an error.
/// percentage = (hw.sensors.acpibat0.watthour3 (remaining capacity)) / (hw.sensors.acpibat0.watthour0 (last full capacity)) * 100
pub fn get_battery_level(bat_index: Option<u8>) -> u8 {
	let (remaining_cap, last_full_cap) = get_battery_watthours(bat_index);
	if last_full_cap > 0 {
		return (remaining_cap as f64 / last_full_cap as f64 * 100.0)
			.round()
			.clamp(0.0, 100.0) as u8;
	}
	return 0;
}

/// Return whether the battery is charging.
fn is_charging() -> bool {
	let mib = match sysctlnametomib("hw.sensors.acpiac0.indicator0") {
		Err(_e) => return false,
		Ok(m) => m,
	};

	let sensor = match sysctl_sensor(&mib) {
		Err(_e) => return false,
		Ok(s) => s,
	};
	return sensor.value > 0;
}

/// Get the remaining time of the battery.
///
/// Return 00:00 if there is an error.
pub fn get_remaining_time() -> String {
	"00:00".to_string()
}

/// Get the state of the battery.
///
/// Returns "CHR" if charging, "BAT" otherwise.
pub fn get_battery_state() -> String {
	if is_charging() {
		return "CHR".to_string();
	} else {
		return "BAT".to_string();
	}
}

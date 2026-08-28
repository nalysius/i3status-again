//! The os::battery::openbsd module implements the OpenBSD way to get information
//! about the battery. It provides the public functions required to the battery
//! backend to easily get access to the battery informations.

use crate::sensors::apm::openbsd::*;

/// Get the level of battery in percent.
///
/// Returns 0 if there is an error.
pub fn get_battery_level() -> u8 {
    if let Ok(info) = read_apm_info() {
		return info.battery_life
	}
	return 0;
}

/// Return whether the battery is charging.
///
/// Note: ac_state is used, not battery_state.
/// It means it returns true as long as it's plugged, even
/// if the battery is full.
fn is_charging() -> bool {
    read_apm_info().ok()
        .map(|info| u32::from(info.ac_state) == APM_AC_ON)
        .unwrap_or(false)
}

/// Get the remaining time of the battery.
///
/// If it's charging, the remaining time is before being full,
/// otherwise it's before being empty.
/// Returns a string in the form of HH:mm with HH being a
/// two-digits hour and mm a two-digits minute.
///
/// Return 00:00 if there is an error.
pub fn get_remaining_time() -> String {
	let time = read_apm_info().ok()
        .map(|info| u32::from(info.minutes_left))
        .unwrap_or(0);

	let hours = time / 60;
	let minutes = time % 60;
	return format!("{:02}:{:02}", hours, minutes);
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

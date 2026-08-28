//! The backends::openbsd::battery module implements the battery
//! block for OpenBSD.
//! For OpenBSD values like ApmPowerInfo and constants, see
//! /// See /usr/includes/machine/apmvar.h

use crate::backends::Backend;
use crate::bar::BlockOutput;
use crate::config::BatteryConfig;
use std::fs::File;
use std::io;
use std::os::fd::AsRawFd;

/// The struct returned when reading apm.
#[repr(C)]
#[derive(Debug)]
pub struct ApmPowerInfo {
    battery_state: u8,
    ac_state: u8,
    battery_life: u8,
    spare1: u8,
    minutes_left: u32,
    spare2: [u32;6],
}

pub const APM_IOC_GETPOWER: u32 = 0x40204103;
pub const APM_AC_ON: u32 = 0x01;
pub const APM_BATT_HIGH: u32 = 0x00;
pub const APM_BATT_LOW: u32 = 0x01;
pub const APM_BATT_CHARGING: u32 = 0x03;
pub const APM_BATT_ABSENT: u32 = 0x04;

/// Read info from apm
fn read_apm_info() -> io::Result<ApmPowerInfo> {
    let file = File::open("/dev/apm")
        .map_err(|e| io::Error::new(e.kind(), "Cannot open /dev/apm"))?;

    let mut info = std::mem::MaybeUninit::<ApmPowerInfo>::uninit();

    let rc = unsafe {
        libc::ioctl(
            file.as_raw_fd(),
            APM_IOC_GETPOWER as _,
            info.as_mut_ptr() as *mut libc::c_void,
        )
    };

    if rc == -1 {
        return Err(io::Error::last_os_error());
    }

    Ok(unsafe { info.assume_init() })
}

/// Get the level of battery in percent.
fn get_battery_level() -> u8 {
    if let Ok(info) = read_apm_info() {
		return info.battery_life
	}
	return 0;
}

/// Return whether the battery is charging.
/// Note: ac_state is used, not battery_state.
/// It means it returns true as long as it's plugged, even
/// if the battery is full.
fn is_charging() -> bool {
    read_apm_info().ok()
        .map(|info| u32::from(info.ac_state) == APM_AC_ON)
        .unwrap_or(false)
}

/// Get the remaining time of the battery.
/// If it's charging, the remaining is before being full,
/// otherwise it's before being empty.
/// Returns a string in the form of HH:mm with HH being a
/// two-digits hour and mm a two-digits minute.
fn get_remaining_time() -> String {
	let time = read_apm_info().ok()
        .map(|info| u32::from(info.minutes_left))
        .unwrap_or(0);

	let hours = time / 60;
	let minutes = time % 60;
	return format!("{:02}:{:02}", hours, minutes);
}

/// Get the state of the battery.
/// Returns "CHR" if charging, "BAT" otherwise.
fn get_battery_state() -> String {
	if is_charging() {
		return "CHR".to_string();
	} else {
		return "BAT".to_string();
	}
}

/// The battery backend.
pub struct BatteryBackend {
	/// The format string to use.
	/// Described in BatteryConfig.
	format: String,
}

impl BatteryBackend {
	pub fn from_config(config: &BatteryConfig) -> Self {
		BatteryBackend {
			format: config.format.to_string(),
		}
	}
}

impl Backend for BatteryBackend {
	fn get_output(&self) -> BlockOutput {
		let rem_percentage = get_battery_level();
		let bat_state = get_battery_state();
		let rem_time = get_remaining_time();
		let out = self.format
			.replace("{rem_percent}", &rem_percentage.to_string())
			.replace("{chr_state}", &bat_state)
			.replace("{rem_time}", &rem_time);
		BlockOutput::new(&format!("{}", out))
	}
}

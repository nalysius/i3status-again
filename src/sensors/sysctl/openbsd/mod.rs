//! The sensors::sysctl::openbsd module defines what's needed to use sysctl
//! on OpenBSD.

use std::error::Error;
use std::fmt;
use libc::{c_int, c_void, size_t, sysctl};

pub mod headers;

pub use crate::sensors::sysctl::openbsd::headers::sensors::*;
pub use crate::sensors::sysctl::openbsd::headers::sysctl::*;

/// An error that can occurs using sysctl
#[derive(Debug)]
pub enum SysctlError {
	/// The name is invalid or the doesn't match a MIB.
	NameNotFound,
	/// The value doesn't match what was expected and isn't usable.
	InvalidValue,
	/// There was an internal error, like the Sensor / SensorDev struct
	/// not up to date with the system.
	InternalError,
}

impl fmt::Display for SysctlError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match &self {
			SysctlError::NameNotFound => write!(f, "Sysctl: name not found."),
			SysctlError::InvalidValue => write!(f, "Sysctl: invalid value."),
			SysctlError::InternalError => write!(f, "Sysctl: internal error, Sensor or SensorDev has an invalid size."),
		}
    }
}

impl Error for SysctlError {
}

/// Get a sysctl MIB from a name.
///
/// Works only for hw.sensors.*, we shouldn't need anothing else in this
/// project.
///
/// sysctl cannot be used with "hw.sensors.cpu0.temp0", it required an MIB.
/// this function searches in the list to find the corresponding one.
pub fn sysctlnametomib(name: &str) -> Result<Vec<c_int>, SysctlError> {
	// Loop over all the sensor devices
	let mut device_id = 0;
	loop {
		let mib = [CTL_HW, HW_SENSORS, device_id];

		// No need to request the size with a first sysctl call, SensorDev has a
		// fixed size
		let mut size = size_of::<SensorDev>();

		// Read the data of the sensor device #device_id
		let mut buf: Vec<u8> = Vec::with_capacity(size);
		unsafe {
			let ret = sysctl(
				mib.as_ptr(), // name
				mib.len() as u32, // namelen
				buf.as_mut_ptr() as *mut c_void, // oldp
				&mut size, //oldlenp
				std::ptr::null_mut(), // newp
				0 as size_t // newlen
			);
			if ret != 0 {
				return Err(SysctlError::NameNotFound);
			}
		}

		if size != size_of::<SensorDev>() {
			return Err(SysctlError::InternalError);
		}
		// Force buf len otherwise it's zero and casting to SensorDev is useless
		unsafe {
			buf.set_len(size);
		}

		let device: SensorDev;
		let device_name: String;
		unsafe {
			// Cast buf to a SensorDev struct
			device = std::ptr::read(buf.as_ptr() as *const SensorDev);
		}
		// The device name has a length of 16, so the null chars used to fill the
		// string need to be trimmed.
		device_name = String::from_utf8(device.xname.to_vec())
			.unwrap()
			.trim_matches(char::from(0))
			.to_string();
		
		// Loop over the device' sensors
		// SensorDev.max_numt is index by type of sensor. See sensors::sysctl::openbsd::SensorType.
		// SensorTemp = 0, SensorFanrpm = 1, etc.
		for sensor_type_id in 0..SENSOR_MAX_TYPES {
			let sensor_number = device.max_numt[sensor_type_id];
			for sensor_id in 0..sensor_number {
				let mib = [CTL_HW, HW_SENSORS, device_id, sensor_type_id.try_into().unwrap(), sensor_id];
				// The size is fixed, no need to call sysctl twice to get the size
				let mut size = size_of::<Sensor>();
				let mut buf: Vec<u8> = Vec::with_capacity(size);
				unsafe {
					let ret = sysctl(
						mib.as_ptr(), // name
						mib.len() as u32, // namelen
						buf.as_mut_ptr() as *mut c_void, // oldp
						&mut size, //oldlenp
						std::ptr::null_mut(), // newp
						0 as size_t // newlen
					);
					if ret != 0 {
						return Err(SysctlError::NameNotFound);
					}
				}
				if size != size_of::<Sensor>() {
					return Err(SysctlError::InternalError);
				}
				// Force the length of buf
				unsafe {
					buf.set_len(size);
				}
				let sensor: Sensor;
				unsafe {
					// Cast it to a Sensor struct
					sensor = std::ptr::read(buf.as_ptr() as *const Sensor);
				}

				// Note: the values are raw. Example: hw.sensors.cpu0.temp is in
				// micro Kelvin, not Celsius.
				let sensor_name = sensor.type_.to_string();
				let found_name = &format!("hw.sensors.{}.{}{}", device_name, sensor_name, sensor_id);
				if found_name == name {
					return Ok(vec![CTL_HW, HW_SENSORS, device_id, sensor_type_id.try_into().unwrap(), sensor_id]);
				}
			}
		}
		device_id += 1;
	}
}

//! The sensors::sysctl::openbsd module defines what's needed to use sysctl
//! on OpenBSD.

use std::error::Error;
use std::fmt;
use std::mem::MaybeUninit;
use libc::{c_int, c_void, size_t, sysctl};

pub mod headers;

pub use crate::sensors::sysctl::openbsd::headers::sensors::*;
pub use crate::sensors::sysctl::openbsd::headers::sysctl::*;

/// An error that can occurs using sysctl
#[derive(Debug)]
pub enum SysctlError {
	/// The item associated with the MIB is not found.
	NotFound,
	/// The address of name, oldp, newp, or length pointer oldlenp is invalid.
	InvalidAddress,
	/// The MIB is less than two or greater than CTL_MAXNAME. Or
	/// a non-null newp pointer is given and its specified length in newlen is
	/// too large or too small.
	InvalidName,
	/// The size of the data is invalid, either too long or too short.
	/// Contains the expected size and the actual size, in this order.
	InvalidSize(usize, usize),
	Other(std::io::Error),
}

impl fmt::Display for SysctlError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match &self {
			SysctlError::NotFound => write!(f, "Sysctl: name not found."),
			SysctlError::InvalidAddress => write!(f, "Sysctl: invalid address."),
			SysctlError::InvalidName => write!(f, "Sysctl: invalid name."),
			SysctlError::InvalidSize(_, _) => write!(f, "Sysctl: Sensor or SensorDev has an invalid size."),
			SysctlError::Other(_) => write!(f, "Sysctl: other error."),
		}
    }
}

impl Error for SysctlError {
}

/// A wrapper around sysctl to get a Sensor.
pub fn sysctl_sensor(mib: &[c_int]) -> Result<Sensor, SysctlError> {
	sysctl_fixed(mib)
}

/// A wrapper around sysctl to get a SensorDev.
pub fn sysctl_sensordev(mib: &[c_int]) -> Result<SensorDev, SysctlError> {
	sysctl_fixed(mib)
}

/// A wrapper around sysctl.
///
/// Works for any type with a fixed size, like Sensor or SensorDev.
/// DON'T use it to query a String or any type with a dynamic size.
fn sysctl_fixed<T: Copy>(mib: &[c_int]) -> Result<T, SysctlError> {
	let mut size = size_of::<T>();
	// Read the data of the sensor device #device_id
	let mut buf = MaybeUninit::uninit();
	let ret: c_int;
	unsafe {
		ret = sysctl(
			mib.as_ptr(), // name
			mib.len() as u32, // namelen
			buf.as_mut_ptr() as *mut c_void, // oldp
			&mut size, //oldlenp
			std::ptr::null_mut(), // newp
			0 as size_t // newlen
		);
	}

	if ret != 0 {
		return match std::io::Error::last_os_error() {
			e if e.raw_os_error() == Some(libc::ENOENT) => Err(SysctlError::NotFound),
			e if e.raw_os_error() == Some(libc::EFAULT) => Err(SysctlError::InvalidAddress),
			e if e.raw_os_error() == Some(libc::EINVAL) => Err(SysctlError::InvalidName),
			e => Err(SysctlError::Other(e)),
		}
	}

	if size != size_of::<T>() {
		return Err(SysctlError::InvalidSize(size_of::<T>(), size));
	}

	Ok(unsafe { buf.assume_init() })
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

		let device: SensorDev = sysctl_sensordev(&mib)?;
		// The device name has a length of 16, so the null chars used to fill the
		// string need to be trimmed.
		let device_name: String = String::from_utf8(device.xname.to_vec())
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
				let sensor: Sensor = sysctl_sensor(&mib)?;
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

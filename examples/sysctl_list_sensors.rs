//! This example shows how to list the available sensors with their raw values
//! using sysctl on OpenBSD.
//! See https://man.openbsd.org/sysctl.2
//! and https://docs.rs/libc/latest/libc/fn.sysctl.html

use i3status_again::sensors::sysctl::openbsd::headers::sensors::{SENSOR_MAX_TYPES, Sensor, SensorDev};
use i3status_again::sensors::sysctl::openbsd::headers::sysctl::{CTL_HW, HW_SENSORS};
use libc::{c_void, size_t, sysctl};
use std::mem::MaybeUninit;

fn main() {
	// Loop over all the sensor devices
	let mut device_id = 0;
	loop {
		let mib = [CTL_HW, HW_SENSORS, device_id];

		// No need to request the size with a first sysctl call, SensorDev has a
		// fixed size
		let mut size = size_of::<SensorDev>();

		// Read the data of the sensor device #device_id
		let mut buf = MaybeUninit::uninit();
		let ret: i32;
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
			println!("No more devices.");
			break;
		}

		if size != size_of::<SensorDev>() {
			println!("Size is invalid. A field could have been updated / added in SensorDev.");
			break;
		}

		let device: SensorDev = unsafe { buf.assume_init() };
		let device_name: String;
		device_name = String::from_utf8(device.xname.to_vec()).unwrap();
		
		// Loop over the device' sensors
		// SensorDev.max_numt is index by type of sensor. See sensors::sysctl::openbsd::SensorType.
		// SensorTemp = 0, SensorFanrpm = 1, etc.
		for sensor_type_id in 0..SENSOR_MAX_TYPES {
			let sensor_number = device.max_numt[sensor_type_id];
			for sensor_id in 0..sensor_number {
				let mib = [CTL_HW, HW_SENSORS, device_id, sensor_type_id.try_into().unwrap(), sensor_id];
				// The size is fixed, no need to call sysctl twice to get the size
				let mut size = size_of::<Sensor>();
				let mut buf = MaybeUninit::uninit();
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
						println!("No more devices.");
						break;
					}
				}
				if size != size_of::<Sensor>() {
					println!("Size is invalid. A field could have been updated / added in Sensor.");
					break;
				}
				let sensor: Sensor = unsafe { buf.assume_init() };

				// Note: the values are raw. Example: hw.sensors.cpu0.temp is in
				// micro Kelvin, not Celsius.
				let sensor_name = sensor.type_.to_string();
				println!("hw.sensors.{}.{}{} = {}", device_name, sensor_name, sensor_id, sensor.value);
			}
		}
		
		device_id += 1;
	}
}

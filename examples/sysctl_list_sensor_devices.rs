//! This example shows how to list the available device sensors using sysctl on
//! OpenBSD.
//! On other operating systems, changing the value of mib should be enough.
//! See https://man.openbsd.org/sysctl.2
//! and https://docs.rs/libc/latest/libc/fn.sysctl.html

// [CTL_HW, HW_SENSORS, device_id, sensor_type, j]

use i3status_again::sensors::sysctl::openbsd::sensors::{SensorDev};
use i3status_again::sensors::sysctl::openbsd::sysctl::{CTL_HW, HW_SENSORS};


use libc::{c_void, size_t, sysctl};

fn main() {
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
			if ret == -1 {
				println!("No more devices.");
				break;
			}
			buf.set_len(size);
		}

		unsafe {
			// Cast it to a SensorDev struct
			let device: SensorDev = std::ptr::read(buf.as_ptr() as *const SensorDev);
			let device_name = String::from_utf8(device.xname.to_vec()).unwrap();
			println!("#{}: hw.sensors.{}", device.num, device_name);
		}
		device_id += 1;
	}
}

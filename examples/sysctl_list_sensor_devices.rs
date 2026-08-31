//! This example shows how to list the available device sensors using sysctl on
//! OpenBSD.
//! See https://man.openbsd.org/sysctl.2
//! and https://docs.rs/libc/latest/libc/fn.sysctl.html

use i3status_again::sensors::sysctl::openbsd::headers::sensors::{SensorDev};
use i3status_again::sensors::sysctl::openbsd::headers::sysctl::{CTL_HW, HW_SENSORS};
use std::mem::MaybeUninit;

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
		if ret == -1 {
			println!("No more devices.");
			break;
		}

		if size != size_of::<SensorDev>() {
			println!("Size is invalid. A field could have been updated / added in SensorDev.");
			break;
		}

		let device: SensorDev = unsafe { buf.assume_init() };
		let device_name = String::from_utf8(device.xname.to_vec()).unwrap();
		println!("#{}: hw.sensors.{}", device.num, device_name);
		device_id += 1;
	}
}

//! This example shows how to query hw.version using sysctl on OpenBSD.
//! See https://man.openbsd.org/sysctl.2
//! and https://docs.rs/libc/latest/libc/fn.sysctl.html

use i3status_again::sensors::sysctl::openbsd::headers::sysctl::{CTL_HW, HW_VERSION};

use libc::{c_void, size_t, sysctl};

fn main() {
	let mib = [CTL_HW, HW_VERSION];

	// Step 1: get the size of the item that will be requested
	// To do so, oldp is given null
	let mut size: usize = 0;
	unsafe {
		sysctl(
			mib.as_ptr(), // name
			mib.len() as u32, // namelen
			std::ptr::null_mut(), // oldp
			&mut size, //oldlenp
			std::ptr::null_mut(), // newp
			0 as size_t // newlen
		);
	}
	println!("Size: {:?}", size);

	// Step 2: get the data. A Vec must be used since array must have a size
	// known at compile time. The method set_len() must be called after systcl
	// has written into it, otherwise buf doesn't know it has a size greater than
	// 0.
	let mut buf: Vec<u8> = Vec::with_capacity(size);
	unsafe {
		sysctl(
			mib.as_ptr(), // name
			mib.len() as u32, // namelen
			buf.as_mut_ptr() as *mut c_void, // oldp
			&mut size, //oldlenp
			std::ptr::null_mut(), // newp
			0 as size_t // newlen
		);
		buf.set_len(size);
	}
	println!("{}", String::from_utf8(buf).unwrap());
}

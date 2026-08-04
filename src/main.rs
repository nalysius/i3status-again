#[cfg(target_os = "openbsd")]
use i3status_again::backends::openbsd::*;

use i3status_again::blocks::*;

use std::io::Write;
use std::{thread, time};

fn main() {
	let sleep_time = time::Duration::from_millis(1000);
	println!(r#"{{"version":1}}"#);
	println!();
	std::io::stdout().flush().unwrap();

	let datetime_backend = DateTimeBackend{};
	println!("[");

	loop {
		println!(
			r#"[ {{"full_text":"{}"}} ],"#,
			datetime_backend.get_datetime()
		);
		std::io::stdout().flush().unwrap();

		thread::sleep(sleep_time);
	}
	
}

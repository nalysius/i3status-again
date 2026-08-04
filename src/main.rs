use std::io::Write;
use std::{thread, time};

fn main() {
	let sleep_time = time::Duration::from_millis(1000);
	println!(r#"{{"version":1}}"#);
	println!();
	std::io::stdout().flush().unwrap();

	loop {
		println!(r#"[ {{"full_text":"🕐 14:32"}}, {{"full_text":"CPU 23%"}}, {{"full_text":"wlan0: -52dBm"}} ]"#);
		std::io::stdout().flush().unwrap();

		thread::sleep(sleep_time);
	}
}

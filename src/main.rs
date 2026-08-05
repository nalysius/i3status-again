use i3status_again::config::load_config;

use std::env;
use std::io::Write;
use std::{thread, time};

fn main() {
	let mut argv = env::args();
	if argv.len() < 2 {
		panic!("Usage: i3status-again path/to/config.toml");
	}
	let config_file = argv.nth(1).unwrap();
	let config = load_config(&config_file).expect("Unable to read configuration");
	let backends = config.to_backends();


	let sleep_time = time::Duration::from_millis(1000);
	println!(r#"{{"version":1}}"#);
	println!("\n[");
	std::io::stdout().flush().unwrap();


	loop {
		let mut json = String::from("[");
		let mut first = true;
		for backend in &backends {
			if !first {
				json.push(',');
			}
			first = false;
			let out = backend.get_output();
			json.push_str(&format!(r#" {{"full_text":"{}"}} "#, out));
		}
		json.push_str("],");
		println!("{}", json);
		std::io::stdout().flush().unwrap();

		thread::sleep(sleep_time);
	}
	
}

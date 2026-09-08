use i3status_again::config::load_config;

use serde_json::json;
use std::env;
use std::io::Write;
use std::{thread, time};

fn main() {
    let mut argv = env::args();
    if argv.len() < 2 {
        println!("Usage: i3status-again path/to/config.toml");
        return;
    }
    let config_file = argv.nth(1).unwrap();
    let config = load_config(&config_file).expect("Unable to read configuration");
    let blocks = config.to_blocks();
    let sleep_time = time::Duration::from_millis(config.interval.into());

    if sleep_time < time::Duration::from_millis(10) {
        eprintln!(
            "Invalid configuration: 'interval' is in milliseconds, it must be greater than 10."
        );
        return;
    }

    println!(r#"{{"version":1}}"#);
    // Open a JSON array
    println!("\n[");
    std::io::stdout().flush().unwrap();

    loop {
        let mut outputs = Vec::new();
        for block in &blocks {
            outputs.push(block.get_output());
        }

        println!("{},", json!(outputs));
        std::io::stdout().flush().unwrap();
        thread::sleep(sleep_time);
    }
}

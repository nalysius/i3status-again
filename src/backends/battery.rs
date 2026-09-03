//! The backends::battery module implements the battery block.

use crate::backends::Backend;
use crate::bar::BlockOutput;
use crate::config::BatteryConfig;
use crate::os::battery::*;

pub struct BatteryBackend {
	/// The format string to use.
	format: String,
	/// The 0-based index of the battery to monitor.
	index: Option<u8>,
}

impl BatteryBackend {
	pub fn from_config(config: &BatteryConfig) -> Self {
		BatteryBackend {
			format: config.format.to_string(),
			index: config.index,
		}
	}
}

impl Backend for BatteryBackend {
	fn get_output(&self) -> BlockOutput {
		let rem_percentage: String = match get_battery_level(self.index) {
			Ok(l) => format!("{}", l),
			Err(e) => e.to_string(),
		};
		let bat_state: String = match get_battery_state() {
			Ok(s) => format!("{}", s),
			Err(e) => e.to_string(),
		};
		let rem_time: String = match get_remaining_time(self.index) {
			Ok(t) => format!("{}", t),
			Err(e) => e.to_string(),
		};
		let out = self.format
			.replace("{rem_percent}", &rem_percentage)
			.replace("{chr_state}", &bat_state)
			.replace("{rem_time}", &rem_time);
		BlockOutput::new(&format!("{}", out))
	}
}

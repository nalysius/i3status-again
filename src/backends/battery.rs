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
		let rem_percentage = get_battery_level(self.index).to_string();
		let bat_state = get_battery_state();
		let rem_time = get_remaining_time(self.index);
		let out = self.format
			.replace("{rem_percent}", &rem_percentage)
			.replace("{chr_state}", &bat_state)
			.replace("{rem_time}", &rem_time);
		BlockOutput::new(&format!("{}", out))
	}
}

//! The backends::battery module implements the battery block.


use crate::backends::Backend;
use crate::bar::BlockOutput;
use crate::config::BatteryConfig;
use crate::os::battery::*;

pub struct BatteryBackend {
	/// The format string to use.
	/// Described in config::BatteryConfig.
	format: String,
}

impl BatteryBackend {
	pub fn from_config(config: &BatteryConfig) -> Self {
		BatteryBackend {
			format: config.format.to_string(),
		}
	}
}

impl Backend for BatteryBackend {
	fn get_output(&self) -> BlockOutput {
		let rem_percentage = get_battery_level().to_string();
		let bat_state = get_battery_state();
		let rem_time = get_remaining_time();
		let out = self.format
			.replace("{rem_percent}", &rem_percentage)
			.replace("{chr_state}", &bat_state)
			.replace("{rem_time}", &rem_time);
		BlockOutput::new(&format!("{}", out))
	}
}

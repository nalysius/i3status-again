//! The backends::datetime module implements the datetime block.

use chrono::Local;
use crate::backends::Backend;
use crate::bar::BlockOutput;
use crate::config::DateTimeConfig;

pub struct DateTimeBackend {
	/// The datetime format string. See the chrono crate for
	/// documentation.
	pub format: String,
}

impl DateTimeBackend {
	pub fn from_config(config: &DateTimeConfig) -> Self {
		DateTimeBackend {
			format: config.format.to_string(),
		}
	}
}

impl Backend for DateTimeBackend {
	fn get_output(&self) -> BlockOutput {
		BlockOutput::new(
			&format!(
				"{}",
				Local::now().format(self.format.as_str())
			)
		)
	}
}

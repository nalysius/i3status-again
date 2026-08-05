//! The backends::openbsd::datetime module implements the datetime
//! block for OpenBSD.

use crate::backends::Backend;
use crate::blocks::DateTimeBlock;
use crate::config::DateTimeConfig;

pub struct DateTimeBackend {
	pub format: String,
}

/// datetime is common between operating systems, so it's implemented
/// directly in the block.
impl DateTimeBlock for DateTimeBackend {
}

impl DateTimeBackend {
	pub fn from_config(config: &DateTimeConfig) -> Self {
		DateTimeBackend {
			format: config.format.to_string(),
		}
	}
}

impl Backend for DateTimeBackend {
	fn get_output(&self) -> String {
		self.get_datetime(self.format.as_str())
	}
}

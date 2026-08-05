//! The backends::openbsd::datetime module implements the datetime
//! block for OpenBSD.

use chrono::Local;
use crate::backends::Backend;
use crate::config::DateTimeConfig;

pub struct DateTimeBackend {
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
	fn get_output(&self) -> String {
		format!("{}", Local::now().format(self.format.as_str()))
	}
}

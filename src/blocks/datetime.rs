//! The blocks::datetime module defines the DateTimeBlock trait.

use chrono::Local;

pub trait DateTimeBlock {
	/// Get the current date/time based on the given format.
	///
	/// Documentation for format can be found at
	/// https://docs.rs/chrono/latest/chrono/format/strftime/index.html
	fn get_datetime(&self, format: &str) -> String {
		format!("{}", Local::now().format(format))
	}
}

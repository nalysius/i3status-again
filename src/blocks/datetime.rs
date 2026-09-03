//! The blocks::datetime module implements the datetime block.

use crate::bar::BlockOutput;
use crate::blocks::Block;
use crate::config::DateTimeConfig;
use chrono::Local;

pub struct DateTimeBlock {
    /// The datetime format string. See the chrono crate for
    /// documentation.
    pub format: String,
}

impl DateTimeBlock {
    pub fn from_config(config: &DateTimeConfig) -> Self {
        DateTimeBlock {
            format: config.format.to_string(),
        }
    }
}

impl Block for DateTimeBlock {
    fn get_output(&self) -> BlockOutput {
        BlockOutput::new(&format!("{}", Local::now().format(self.format.as_str())))
    }
}

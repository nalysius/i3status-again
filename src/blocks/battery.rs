//! The blocks::battery module implements the battery block.

use crate::bar::BlockOutput;
use crate::blocks::Block;
use crate::common::Color;
use crate::config::BatteryConfig;
use crate::os::battery::*;

pub struct BatteryBlock {
    /// The format string to use.
    format: String,
    /// The 0-based index of the battery to monitor.
    index: Option<u8>,
    /// The background color when the level is critical.
    bg_critical: Color,
    /// The percentage from which the level is considered critical.
    critical_level: u8,
}

impl BatteryBlock {
    pub fn from_config(config: &BatteryConfig) -> Self {
        BatteryBlock {
            format: config.format.to_string(),
            index: config.index,
            bg_critical: config.bg_critical,
            critical_level: config.critical_level,
        }
    }
}

impl Block for BatteryBlock {
    fn get_output(&self) -> BlockOutput {
        let (rem_percentage_s, rem_percentage): (String, u8) = match get_battery_level(self.index) {
            Ok(l) => (format!("{}", l), l),
            Err(e) => (e.to_string(), 0),
        };
        let bat_state: String = match get_battery_state() {
            Ok(s) => format!("{}", s),
            Err(e) => e.to_string(),
        };
        let rem_time: String = match get_remaining_time(self.index) {
            Ok(t) => format!("{}", t),
            Err(e) => e.to_string(),
        };
        let out = self
            .format
            .replace("{rem_percent}", &rem_percentage_s)
            .replace("{chr_state}", &bat_state)
            .replace("{rem_time}", &rem_time);

        let mut block = BlockOutput::new(&format!("{}", out));
        if rem_percentage <= self.critical_level {
            block.background = Some(self.bg_critical);
        }
        block
    }
}

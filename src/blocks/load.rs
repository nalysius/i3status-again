//! The blocks::datetime module implements the load block.

use crate::bar::BlockOutput;
use crate::blocks::Block;
use crate::config::LoadConfig;
use crate::os::load::*;

pub struct LoadBlock {
    /// The load format string.
    pub format: String,
}

impl LoadBlock {
    pub fn from_config(config: &LoadConfig) -> Self {
        LoadBlock {
            format: config.format.to_string(),
        }
    }
}

impl Block for LoadBlock {
    fn get_output(&mut self) -> BlockOutput {
        let (l1m, l5m, l15m) = match get_load_avg() {
            Ok((l1, l5, l15)) => (l1, l5, l15),
            Err(e) => return BlockOutput::new(&e.to_string()),
        };

        let out = self
            .format
            .replace("{load1m}", &format!("{:.2}", l1m))
            .replace("{load5m}", &format!("{:.2}", l5m))
            .replace("{load15m}", &format!("{:.2}", l15m));

        BlockOutput::new(&out)
    }
}

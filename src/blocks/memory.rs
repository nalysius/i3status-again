//! The blocks::memory module implements the memory block.

use crate::bar::BlockOutput;
use crate::blocks::Block;
use crate::common::MemoryUnit;
use crate::config::MemoryConfig;
use crate::os::memory::*;

pub struct MemoryBlock {
    /// The format string to use.
    format: String,
    /// The unit to use to compute the memory.
    unit: MemoryUnit,
}

impl MemoryBlock {
    pub fn from_config(config: &MemoryConfig) -> Self {
        MemoryBlock {
            format: config.format.to_string(),
            unit: config.unit,
        }
    }
}

impl Block for MemoryBlock {
    fn get_output(&self) -> BlockOutput {
        let (total_memory, used_memory, used_percent) = match get_memory(self.unit) {
            Ok((tm, um, up)) => (tm, um, up),
            Err(e) => return BlockOutput::new(&e.to_string()),
        };

        let out = self
            .format
            .replace("{mem_used_percent}", &format!("{:.1}", used_percent))
            .replace("{mem_total}", &format!("{:.1}", total_memory))
            .replace("{mem_used}", &format!("{:.1}", used_memory))
            .replace("{unit}", &self.unit.to_string());

        BlockOutput::new(&format!("{}", out))
    }
}

//! The blocks::disk_usage module implements the disk_usage block.

use crate::bar::BlockOutput;
use crate::blocks::Block;
use crate::common::MemoryUnit;
use crate::config::DiskUsageConfig;
use crate::os::disk_usage::*;

pub struct DiskUsageBlock {
    /// The format string to use.
    format: String,
    /// The memory to use to compute the available space.
    unit: MemoryUnit,
    /// The mount point of the partition to check.
    mount_point: String,
}

impl DiskUsageBlock {
    pub fn from_config(config: &DiskUsageConfig) -> Self {
        DiskUsageBlock {
            format: config.format.to_string(),
            unit: config.unit,
            mount_point: config.mount_point.clone(),
        }
    }
}

impl Block for DiskUsageBlock {
    fn get_output(&mut self) -> BlockOutput {
        let (total_space, used_space, used_percent, avail_space) =
            match get_disk_usage(&self.mount_point, self.unit) {
                Ok((ts, us, up, asp)) => (ts, us, up, asp),
                Err(e) => return BlockOutput::new(&e.to_string()),
            };

        let out = self
            .format
            .replace("{space_used}", &format!("{:.1}", used_space))
            .replace("{space_total}", &format!("{:.1}", total_space))
            .replace("{space_used_percent}", &format!("{:.1}", used_percent))
            .replace("{space_available}", &format!("{:.1}", avail_space))
            .replace("{unit}", &self.unit.to_string());
        BlockOutput::new(&out)
    }
}

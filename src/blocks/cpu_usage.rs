//! The blocks::cpu_usage module implements the cpu_usage block.

use crate::bar::BlockOutput;
use crate::blocks::Block;
use crate::config::CpuUsageConfig;
use crate::os::cpu_usage::*;

pub struct CpuUsageBlock {
    /// The format string to use.
    format: String,
    /// The 0-based index of the CPU to monitor.
    index: Option<i32>,
    /// The last measured stats.
    /// (used_ticks, idle_ticks)
    last_stats: (u64, u64),
    /// The second to last stats.
    /// (used_ticks, idle_ticks)
    second_to_last_stats: (u64, u64),
}

impl CpuUsageBlock {
    pub fn from_config(config: &CpuUsageConfig) -> Self {
        CpuUsageBlock {
            format: config.format.to_string(),
            index: config.index,
            last_stats: (0, 0),
            second_to_last_stats: (0, 0),
        }
    }
}

impl Block for CpuUsageBlock {
    fn get_output(&mut self) -> BlockOutput {
        let (cur_used_ticks, cur_idle_ticks) = match get_cpu_usage(self.index) {
            Ok((u, i)) => (u, i),
            Err(e) => return BlockOutput::new(&format!("{}", e)),
        };

        self.second_to_last_stats = self.last_stats;
        self.last_stats = (cur_used_ticks, cur_idle_ticks);

        let used_ticks = self.last_stats.0 - self.second_to_last_stats.0;
        let idle_ticks = self.last_stats.1 - self.second_to_last_stats.1;
        let percentage: u64 =
            (used_ticks as f64 * 100.0 / (used_ticks as f64 + idle_ticks as f64)) as u64;

        let out = self
            .format
            .replace("{used_percent}", &format!("{}", percentage));
        BlockOutput::new(&format!("{}", out))
    }
}

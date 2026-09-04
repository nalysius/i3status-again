//! The blocks::cpu_freq module implements the cpu_freq block.

use crate::bar::BlockOutput;
use crate::blocks::Block;
use crate::common::{AggregatUnit, FreqUnit};
use crate::config::CpuFreqConfig;
use crate::os::cpu_freq::*;

pub struct CpuFreqBlock {
    /// The format string to use.
    format: String,
    /// The 0-based index of the CPU to monitor.
    index: Option<u8>,
    /// The unit of the frequency.
    unit: FreqUnit,
    /// How to aggregate the data if there are several CPUs.
    aggregation: AggregatUnit,
}

impl CpuFreqBlock {
    pub fn from_config(config: &CpuFreqConfig) -> Self {
        CpuFreqBlock {
            format: config.format.to_string(),
            index: config.index,
            unit: config.unit,
            aggregation: config.aggregation,
        }
    }
}

impl Block for CpuFreqBlock {
    fn get_output(&self) -> BlockOutput {
        let cpu_freq: String = match get_cpu_freq(self.index, self.unit, self.aggregation) {
            Ok(l) => format!("{:.2}", l),
            Err(e) => e.to_string(),
        };

        let out = self
            .format
            .replace("{freq}", &cpu_freq)
            .replace("{unit}", &self.unit.to_string());
        BlockOutput::new(&format!("{}", out))
    }
}

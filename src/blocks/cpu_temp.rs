//! The blocks::cpu_temp module implements the cpu_temp block.

use crate::bar::BlockOutput;
use crate::blocks::Block;
use crate::common::TempUnit;
use crate::config::CpuTempConfig;
use crate::os::cpu_temp::*;

pub struct CpuTempBlock {
    /// The format string to use.
    format: String,
    /// The 0-based index of the CPU to monitor.
    index: Option<u8>,
    /// The unit of the temperature.
    unit: TempUnit,
}

impl CpuTempBlock {
    pub fn from_config(config: &CpuTempConfig) -> Self {
        CpuTempBlock {
            format: config.format.to_string(),
            index: config.index,
            unit: config.unit,
        }
    }
}

impl Block for CpuTempBlock {
    fn get_output(&self) -> BlockOutput {
        let cpu_temp: String = match get_cpu_temp(self.index, self.unit) {
            Ok(l) => format!("{}", l),
            Err(e) => e.to_string(),
        };

        let out = self
            .format
            .replace("{temp}", &cpu_temp)
            .replace("{unit}", &self.unit.to_string());
        BlockOutput::new(&format!("{}", out))
    }
}

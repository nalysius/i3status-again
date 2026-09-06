//! The os::cpu_freq::notsupported module contains the empty implementation
//! used on operating systems not supporting the block.

use crate::os::cpu_freq::CpuFreqError;

pub fn get_cpu_freq(
    cpu_index: Option<u8>,
    unit: FreqUnit,
    aggregation: AggregatUnit,
) -> Result<f64, CpuFreqError> {
    Err(CpuFreqError::OsNotSupported)
}

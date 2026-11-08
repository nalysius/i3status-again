//! The os::cpu_usage::notsupported module contains the empty implementation
//! used on operating systems not supporting the block.

use crate::usage::cpu_usage::CpuUsageError;

pub fn get_cpu_usage(cpu_index: Option<u32>) -> Result<(u64, u64), CpuUsageError> {
    Err(CpuUsageError::OsNotSupported)
}

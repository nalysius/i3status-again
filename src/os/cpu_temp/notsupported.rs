//! The os::cpu_temp::notsupported module provides an empty implementation for
//! operating systems not supporting the cpu_temp block.

use crate::os::cpu_temp::{CpuTempError, TempUnit};

pub fn get_cpu_temp(cpu_index: Option<u8>, unit: TempUnit) -> Result<u8, CpuTempError> {
    Err(CpuTempError::OsNotSupported)
}

//! The os::disk_usage::notsupported module is the default empty implementation
//! that is used on unsupported operating systems.

use crate::common::MemoryUnit;
use crate::os::disk_usage::DiskUsageError;

pub fn get_disk_usage(unit: MemoryUnit) -> Result<(f64, f64, f64, f64), DiskUsageError> {
    Err(DiskUsageError::OsNotSupported)
}

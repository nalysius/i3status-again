//! The os::disk_usage::openbsd module implements the OpenBSD way to get
//! information about how the disk is used. It provides the public functions to
//! the disk_usage block to easily get access to the disk information.

use crate::common::MemoryUnit;
use crate::os::disk_usage::DiskUsageError;
use crate::sensors::statvfs::openbsd::*;

/// Get the usage information for the given partition.
pub fn get_disk_usage(
    mount_point: &str,
    unit: MemoryUnit,
) -> Result<(f64, f64, f64, f64), DiskUsageError> {
    let (total_space, used_space, used_percent, avail_space) = match statvfs_rs(mount_point) {
        Ok((ts, us, up, asp)) => (ts, us, up, asp),
        Err(e) => return Err(e.into()),
    };

    return match unit {
        // 1048576 = 1024 ^ 2
        MemoryUnit::MebiByte => Ok((
            total_space / 1048576.0,
            used_space / 1048576.0,
            used_percent,
            avail_space / 1048576.0,
        )),
        // 1073741824 = 1024 ^ 3
        MemoryUnit::GibiByte => Ok((
            total_space / 1073741824.0,
            used_space / 1073741824.0,
            used_percent,
            avail_space / 1073741824.0,
        )),
    };
}

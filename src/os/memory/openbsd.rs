//! The os::memory::openbsd module implements the OpenBSD way to get information
//! about the memory. It provides the public functions required to the memory
//! backend to easily get access to the memory information.

use crate::common::MemoryUnit;
use crate::os::memory::MemoryError;
use crate::sensors::sysctl::openbsd::*;

/// Get the information about the memory.
///
/// If Ok, returns (total_memory, used_memory, used_percent).
/// total_memory is the total amount of memory on the machine.
/// used_memory is the used amount of memory.
/// used_percent is the percentage of used memory.
/// total_memory and used_memory are in the unit requested by `unit`.
pub fn get_memory(unit: MemoryUnit) -> Result<(f64, f64, f64), MemoryError> {
    let mem = match sysctl_uvmexp() {
        Ok(v) => v,
        Err(e) => return Err(e.into()),
    };

    // Ensure the data returned by the kernel are consistent
    if mem.pagesize == 0
        || mem.npages == 0
        || mem.free > mem.npages
        || mem.inactive > mem.npages
        || mem.active > mem.npages
        || mem.wired > mem.npages
    {
        return Err(MemoryError::InvalidData);
    }

    let mut total_memory: f64 = (mem.npages as u64 * mem.pagesize as u64) as f64;
    // See docs/decisions.md to know why the used memory is computed this way
    let mut used_memory: f64 =
        ((mem.active as u64 + mem.wired as u64) * mem.pagesize as u64) as f64;
    let used_percent: f64 = used_memory / total_memory * 100.0;

    match unit {
        MemoryUnit::MebiByte => {
            total_memory = total_memory / (1024.0 * 1024.0);
            used_memory = used_memory / (1024.0 * 1024.0);
        }
        MemoryUnit::GibiByte => {
            total_memory = total_memory / (1024.0 * 1024.0 * 1024.0);
            used_memory = used_memory / (1024.0 * 1024.0 * 1024.0);
        }
    }

    Ok((total_memory, used_memory, used_percent))
}

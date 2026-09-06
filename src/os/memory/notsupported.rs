//! The os::memory::notsupported module provides an empty implementation for
//! the operating systems not supporting the block.

use crate::common::MemoryUnit;
use crate::os::memory::MemoryError;

pub fn get_memory(unit: MemoryUnit) -> Result<(f64, f64, f64), MemoryError> {
    Err(MemoryError::OsNotSupported)
}

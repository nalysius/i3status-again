//! The blocks module defines the blocks implementation.
//!
//! Enumerations and traits are defined to make usage of blocks outside
//! of the module easier. For example a Vec<BlockType> can contains several
//! blocks, even if they are different.
//!
//! Blocks are shared between OS, no OS-dependant code can live here. It lives
//! in the os module.

pub mod battery;
pub mod cpu_freq;
pub mod cpu_temp;
pub mod datetime;
pub mod disk_usage;
pub mod load;
pub mod memory;
pub mod volume;

pub use crate::blocks::battery::BatteryBlock;
pub use crate::blocks::cpu_freq::CpuFreqBlock;
pub use crate::blocks::cpu_temp::CpuTempBlock;
pub use crate::blocks::datetime::DateTimeBlock;
pub use crate::blocks::disk_usage::DiskUsageBlock;
pub use crate::blocks::load::LoadBlock;
pub use crate::blocks::memory::MemoryBlock;
pub use crate::blocks::volume::VolumeBlock;

use crate::bar::BlockOutput;

/// BlockType is an enumeration used to represent block types.
///
/// It makes storing different blocks together easier. Instead of a
/// Vec<dyn Block>, store a Vec<BlockType>.
pub enum BlockType {
    Battery(BatteryBlock),
    CpuFreq(CpuFreqBlock),
    CpuTemp(CpuTempBlock),
    DateTime(DateTimeBlock),
    DiskUsage(DiskUsageBlock),
    Load(LoadBlock),
    Memory(MemoryBlock),
    Volume(VolumeBlock),
}

impl BlockType {
    /// A shortcut function to call get_output on the block.
    pub fn get_output(&mut self) -> BlockOutput {
        match self {
            Self::Battery(b) => b.get_output(),
            Self::CpuFreq(c) => c.get_output(),
            Self::CpuTemp(c) => c.get_output(),
            Self::DateTime(d) => d.get_output(),
            Self::DiskUsage(d) => d.get_output(),
            Self::Load(l) => l.get_output(),
            Self::Memory(m) => m.get_output(),
            Self::Volume(v) => v.get_output(),
        }
    }
}

/// A simple trait to enforce some methods in every traits.
pub trait Block {
    /// Main method of a block to generate an output.
    fn get_output(&mut self) -> BlockOutput;
}

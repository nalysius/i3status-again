//! The os::cpu_usage::openbsd module contains the implementation
//! of cpu_usage for OpenBSD.

use crate::os::cpu_usage::CpuUsageError;
use crate::sensors::sysctl::openbsd::{
    CP_IDLE, CPUSTATS_ONLINE, cpustats, sysctl_cpustats, sysctl_int,
};
use libc::{CTL_HW, CTL_KERN, HW_NCPU, KERN_CPUSTATS};

/// Reads the stats of a CPU.
///
/// stats is a &cpustats, the stats about a CPU.
///
/// Returns (used_ticks, idle_ticks).
fn get_ticks(stats: &cpustats) -> (u64, u64) {
    let mut used_ticks = 0;
    let mut idle_ticks = 0;

    for (index, ticks) in stats.cs_time.iter().enumerate() {
        if index == CP_IDLE as usize {
            idle_ticks += ticks;
        } else {
            used_ticks += ticks;
        }
    }

    (used_ticks, idle_ticks)
}

/// Compute the CPU usage.
///
/// cpu_index is the index of the CPU, or None to compute for all the CPUs.
///
/// Returns Result<(used_ticks, idle_ticks), CpuUsageError>
pub fn get_cpu_usage(cpu_index: Option<i32>) -> Result<(u64, u64), CpuUsageError> {
    let ncpu = match sysctl_int(&[CTL_HW, HW_NCPU]) {
        Ok(v) => v,
        Err(e) => return Err(e.into()),
    };

    if ncpu == 0 {
        return Err(CpuUsageError::CpuNotFound);
    }

    if let Some(i) = cpu_index {
        if i < 0 || i > ncpu {
            return Err(CpuUsageError::CpuNotFound);
        }
    }

    if let Some(i) = cpu_index {
        let stats: cpustats = match sysctl_cpustats(&[CTL_KERN, KERN_CPUSTATS, i]) {
            Ok(s) => s,
            Err(e) => return Err(e.into()),
        };
        if stats.cs_flags & CPUSTATS_ONLINE as u64 == 0 {
            return Err(CpuUsageError::CpuDisabled);
        } else {
            let (cur_used_ticks, cur_idle_ticks) = get_ticks(&stats);
            return Ok((cur_used_ticks, cur_idle_ticks));
        }
    }

    let mut used_ticks = 0;
    let mut idle_ticks = 0;

    // Some CPUs can be offline (SMT & lethargic by default), but we don't know
    // their index. An offline CPU can also be reactivated, so it's easier to
    // recheck all CPUs each time.
    for n in 0..ncpu {
        let stats: cpustats = match sysctl_cpustats(&[CTL_KERN, KERN_CPUSTATS, n]) {
            Ok(v) => v,
            Err(e) => return Err(e.into()),
        };

        if stats.cs_flags & CPUSTATS_ONLINE as u64 == 0 {
            continue;
        }

        let (cur_used_ticks, cur_idle_ticks) = get_ticks(&stats);

        used_ticks += cur_used_ticks;
        idle_ticks += cur_idle_ticks;
    }
    Ok((used_ticks, idle_ticks))
}

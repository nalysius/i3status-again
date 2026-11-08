//! The sensors::sysctl::openbsd::sched module contains the constants and
//! structures used in OpenBSD to access scheduling.
//! See /usr/include/sys/sched.h

use libc::c_int;

// CPU states.
pub const CP_USER: c_int = 0;
pub const CP_NICE: c_int = 1;
pub const CP_SYS: c_int = 2;
pub const CP_SPIN: c_int = 3;
pub const CP_INTR: c_int = 4;
pub const CP_IDLE: c_int = 5;
pub const CPUSTATES: c_int = 6;

#[repr(C)]
pub struct cpustats {
    /// CPU state statistics
    pub cs_time: [u64; CPUSTATES as usize],
    pub cs_flags: u64,
}

/// CPU is schedulable
pub const CPUSTATS_ONLINE: c_int = 0x0001;

//! For OpenBSD values like ApmPowerInfo and constants, see
//! See /usr/includes/machine/apmvar.h

use std::fs::File;
use std::io;
use std::os::fd::AsRawFd;


/// The struct returned when reading apm.
#[repr(C)]
#[derive(Debug)]
pub struct ApmPowerInfo {
    pub battery_state: u8,
    pub ac_state: u8,
    pub battery_life: u8,
    pub spare1: u8,
    pub minutes_left: u32,
    pub spare2: [u32;6],
}

pub const APM_IOC_GETPOWER: u32 = 0x40204103;
pub const APM_AC_ON: u32 = 0x01;
pub const APM_BATT_HIGH: u32 = 0x00;
pub const APM_BATT_LOW: u32 = 0x01;
pub const APM_BATT_CHARGING: u32 = 0x03;
pub const APM_BATT_ABSENT: u32 = 0x04;

/// Read info from apm
pub fn read_apm_info() -> io::Result<ApmPowerInfo> {
    let file = File::open("/dev/apm")
        .map_err(|e| io::Error::new(e.kind(), "Cannot open /dev/apm"))?;

    let mut info = std::mem::MaybeUninit::<ApmPowerInfo>::uninit();

    let rc = unsafe {
        libc::ioctl(
            file.as_raw_fd(),
            APM_IOC_GETPOWER as _,
            info.as_mut_ptr() as *mut libc::c_void,
        )
    };

    if rc == -1 {
        return Err(io::Error::last_os_error());
    }

    Ok(unsafe { info.assume_init() })
}

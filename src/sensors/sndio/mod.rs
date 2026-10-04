//! The sensors::sndio module defines a wrapper around sndio.
//! The sndio API should be the same on every OS, so it can be implemented
//! the same way everywhere.
//!
//! It's part of the base system only on OpenBSD, so it's used on OpenBSD by
//! default but is optional on other systems. On FreeBSD, Linux and NetBSD,
//! sndio can be used if i3status-again is compiled with the --features sndio
//! flag and the libsndio library is installed.

pub mod headers;

use crate::sensors::sndio::headers::*;
use libc::{c_int, c_uint, c_void, pollfd};
use std::ffi::CStr;

/// Represents a volume control
struct Control {
    addr: u32,
    /// output.level
    is_level: bool,
    /// output.mute
    is_mute: bool,
    /// Current volume
    cur: u32,
    /// Maximum volume
    max: u32,
}

/// A context for sndio
pub struct SndioCtx {
    hdl: *mut sioctl_hdl,
    controls: Vec<Control>,
    pfds: Vec<pollfd>,
    /// If poll reads POLLHUP, mark the context as deas and recreate it next time
    dead: bool,
}

impl SndioCtx {
    /// Creates a new context and register the callbacks.
    pub fn new() -> Option<Box<Self>> {
        let hdl = unsafe { sioctl_open(SIO_DEVANY.as_ptr(), SIOCTL_READ as c_uint, 0) };
        if hdl.is_null() {
            return None;
        }
        let mut ctx = Box::new(Self {
            hdl,
            controls: Vec::new(),
            pfds: Vec::new(),
            dead: false,
        });

        let arg = (&mut *ctx as *mut Self).cast();
        unsafe {
            sioctl_ondesc(hdl, on_desc, arg);
            sioctl_onval(hdl, on_val, arg);
        }
        Some(ctx)
    }

    /// Polls the events from sndio.
    pub fn poll(&mut self) {
        if self.dead {
            return;
        }
        // Maximum number of pollfd sndio could need
        let nfds = unsafe { sioctl_nfds(self.hdl) } as usize;
        if self.pfds.len() != nfds {
            self.pfds.clear();
            self.pfds
                .resize_with(nfds, || unsafe { std::mem::zeroed() });
        }

        // Fill the buffer, return the number of pollfd to monitor
        let n = unsafe {
            sioctl_pollfd(
                self.hdl,
                self.pfds.as_mut_ptr(),
                libc::POLLIN as libc::c_int,
            )
        };
        if n <= 0 {
            return;
        }

        // Unlock the events and consume them.
        // Callbacks on_desc and on_val  are called during sioctl_revents.
        if unsafe { libc::poll(self.pfds.as_mut_ptr(), n as libc::nfds_t, 0) } <= 0 {
            return;
        }
        let revents = unsafe { sioctl_revents(self.hdl, self.pfds.as_mut_ptr()) };
        // POLLHUP means connection to sndio has been lost.
        if revents & (libc::POLLHUP as libc::c_int) != 0 {
            self.dead = true;
        }
    }

    /// Reads the volume.
    /// Must be called after poll().
    /// If Some(), contains (volume_current, volume_max, is_muted), so it's easy
    /// to compute a percentage.
    pub fn volume(&self) -> Option<(u32, u32, bool)> {
        if self.dead {
            return None;
        }

        let level = self.controls.iter().find(|c| c.is_level)?;
        if let Some(mute) = self.controls.iter().find(|c| c.is_mute) {
            if mute.cur != 0 {
                return Some((0, level.max, true)); // muted
            }
        }
        Some((level.cur, level.max, false))
    }

    pub fn is_dead(&self) -> bool {
        self.dead
    }
}

impl Drop for SndioCtx {
    fn drop(&mut self) {
        unsafe { sioctl_close(self.hdl) };
    }
}

/// Called when discovering the output devices.
/// Allows to get the description and volume, but isn't called again when
/// the volume changes, on_val is needed for this reason.
unsafe extern "C" fn on_desc(arg: *mut c_void, desc: *mut sioctl_desc, val: c_int) {
    if desc.is_null() {
        return;
    }
    let ctx = unsafe { &mut *arg.cast::<SndioCtx>() };
    let desc = unsafe { &*desc };

    // group[0] != 0 : control is in a subgroup, ignore
    if desc.group[0] != 0 {
        return;
    }
    if !carray_eq(&desc.node0.name, b"output") {
        return;
    }

    let is_level = carray_eq(&desc.func, b"level");
    let is_mute = carray_eq(&desc.func, b"mute");
    if !is_level && !is_mute {
        return;
    }

    if let Some(c) = ctx.controls.iter_mut().find(|c| c.addr == desc.addr) {
        c.cur = val as u32;
        c.max = desc.maxval;
    } else {
        ctx.controls.push(Control {
            addr: desc.addr,
            is_level,
            is_mute,
            cur: val as u32,
            max: desc.maxval,
        });
    }
}

/// Called when the volume changes.
unsafe extern "C" fn on_val(arg: *mut c_void, addr: c_uint, val: c_uint) {
    let ctx = unsafe { &mut *arg.cast::<SndioCtx>() };
    if let Some(c) = ctx.controls.iter_mut().find(|c| c.addr == addr) {
        c.cur = val;
    }
}

/// Compares two C arrays and check if they are equal
fn carray_eq(name: &[libc::c_char], s: &[u8]) -> bool {
    let bytes = unsafe { std::slice::from_raw_parts(name.as_ptr().cast(), name.len()) };
    match CStr::from_bytes_until_nul(bytes) {
        Ok(cstr) => cstr.to_bytes() == s,
        Err(_) => false,
    }
}

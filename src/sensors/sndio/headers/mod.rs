#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
//! The module sensors::sndio::headers port defines the FFI required to
//! use sndio from Rust.
//! See /usr/include/sndio.h

use libc::{c_char, c_int, c_uint, c_void, size_t};
use std::ffi::CStr;

// Default audio device and MIDI port
pub const SIO_DEVANY: &CStr = c"default";
pub const MIO_PORTANY: &CStr = c"default";

/// Max name length
pub const SIOCTL_NAMEMAX: c_int = 16;
/// Max display string ength
pub const SIOCTL_DISPLAYMAX: c_int = 32;

// Private "handle" structure
#[repr(C)]
pub struct sioctl_hdl {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sio_hdl {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mio_hdl {
    _private: [u8; 0],
}

/// Pause during xrun
pub const SIO_IGNORE: c_int = 0;
/// Resync after xrun
pub const SIO_SYNC: c_int = 1;
/// Terminate on xrun
pub const SIO_ERROR: c_int = 2;

#[repr(C)]
/// Parameters of a full-duplex stream
pub struct sio_par {
    /// Bits per sample
    pub bits: c_uint,
    /// Bytes per sample
    pub bps: c_uint,
    /// 1 = signed, 0 = unsigned
    pub sig: c_uint,
    /// 1 = LE, 0 = BE byte order
    pub le: c_uint,
    /// 1 = MSB, 0 = LSB aligned
    pub msb: c_uint,
    /// Number channels for recording direction
    pub rchan: c_uint,
    /// Number channels for playback direction
    pub pchan: c_uint,
    /// Frames per seconds
    pub rate: c_uint,
    /// End-to-end buffer size
    pub bufsz: c_uint,
    /// What to do on overruns / underruns
    pub xrun: c_uint,
    /// Optimal bufsz divisor
    pub round: c_uint,
    /// Minimum buffer size
    pub appbufsz: c_uint,
    /// For future use
    pub __pad: [c_int; 3],
    /// For internal / debug purposes only
    pub __magic: c_uint,
}

pub const SIO_NENC: c_int = 8;
pub const SIO_NCHAN: c_int = 8;
pub const SIO_NRATE: c_int = 16;
pub const SIO_NCONF: c_int = 4;

#[repr(C)]
/// Allowed sample encoding
pub struct sio_enc {
    pub bits: c_uint,
    pub bps: c_uint,
    pub sig: c_uint,
    pub le: c_uint,
    pub msb: c_uint,
}

#[repr(C)]
pub struct sio_conf {
    /// mask of enc[] indexes
    pub enc: c_uint,
    /// mask of chan[] indexes (rec)
    pub rchan: c_uint,
    /// mask of chan[] indexes (play)
    pub pchan: c_uint,
    /// mask of rate[] indexes
    pub rate: c_uint,
}

#[repr(C)]
/// Capabilities of a stream
pub struct sio_cap {
    /// Allowed sample encodings
    pub enc: [sio_enc; SIO_NENC as usize],
    /// Allowed value for rchan
    pub rchan: [c_uint; SIO_NCHAN as usize],
    /// Allowed values for pchan
    pub pchan: [c_uint; SIO_NCHAN as usize],
    /// Allowed rates
    pub rate: [c_uint; SIO_NRATE as usize],
    /// For future use
    pub __pad: [c_int; 7],
    /// Number of elements in confs[]
    pub nconf: c_uint,
    pub confs: [sio_conf; SIO_NCONF as usize],
}

pub const SIO_XSTRINGS: [&CStr; 3] = [c"ignore", c"sync", c"error"];

#[repr(C)]
/// Controller component of the device
pub struct sioctl_node {
    /// Ex. "spkr"
    pub name: [c_char; SIOCTL_NAMEMAX as usize],
    /// Optional number or -1
    pub unit: c_int,
}

/// Deleted
pub const SIOCTL_NONE: c_int = 0;
/// Integer in the 0..maxval range
pub const SIOCTL_NUM: c_int = 2;
/// On/off switch (0 or 1)
pub const SIOCTL_SW: c_int = 3;
/// Number, element of vector
pub const SIOCTL_VEC: c_int = 4;
/// Switch, element of a list
pub const SIOCTL_LIST: c_int = 5;
/// Element of a selector
pub const SIOCTL_SEL: c_int = 6;

/// Description of a control (index, value) pair
#[repr(C)]
pub struct sioctl_desc {
    /// Control address
    pub addr: c_uint,
    /// One of SIOCTL_{NONE,NUM,SW,VEC,LIST,SEL}
    pub type_: c_uint,
    /// Max value
    pub maxval: c_uint,
    /// For future use
    pub __pad: [c_int; 3],
    /// Function name, ex. "level"
    pub func: [c_char; SIOCTL_NAMEMAX as usize],
    /// Group this control belongs to
    pub group: [c_char; SIOCTL_NAMEMAX as usize],
    /// Affected node
    pub node0: sioctl_node,
    /// Dito for SIOCTL_{VEC,LIST,SEL}
    pub node1: sioctl_node,
    /// Free-format hint
    pub display: [c_char; SIOCTL_DISPLAYMAX as usize],
}

// Mode bitmap
pub const SIO_PLAY: c_int = 1;
pub const SIO_REC: c_int = 2;
pub const MIO_OUT: c_int = 4;
pub const MIO_IN: c_int = 8;
pub const SIOCTL_READ: c_int = 0x100;
pub const SIOCTL_WRITE: c_int = 0x200;

/// Default bytes per sample for the given bits per sample
#[inline]
pub const fn SIO_BPS(bits: c_int) -> size_t {
    if bits <= 8 {
        1
    } else if bits <= 16 {
        2
    } else {
        4
    }
}

// Default value of "sio_par.le" flag
pub const SIO_LE_NATIVE: c_int = if cfg!(target_endian = "little") { 1 } else { 0 };

/// Maximum value of volume, eg. for sio_setvol()
pub const SIO_MAXVOL: c_int = 127;

#[repr(C)]
pub struct pollfd {
    _private: [u8; 0],
}

pub type SioOnmoveCb = extern "C" fn(arg: *mut c_void, delta: c_int);
pub type SioOnxrunCb = extern "C" fn(arg: *mut c_void);
pub type SioOnvolCb = extern "C" fn(arg: *mut c_void, vol: c_uint);
pub type SioctlOndescCb = extern "C" fn(arg: *mut c_void, desc: *mut sioctl_desc, val: c_int);
pub type SioctlOnvalCb = extern "C" fn(arg: *mut c_void, addr: c_uint, val: c_uint);

unsafe extern "C" {
    pub fn sio_initpar(par: *mut sio_par);
    pub fn sio_open(name: *const c_char, mode: c_uint, nbio_flag: c_int) -> *mut sio_hdl;
    pub fn sio_close(hdl: *mut sio_hdl);
    pub fn sio_setpar(hdl: *mut sio_hdl, par: *mut sio_par) -> c_int;
    pub fn sio_getpar(hdl: *mut sio_hdl, par: *mut sio_par) -> c_int;
    pub fn sio_getcap(hdl: *mut sio_hdl, cap: *mut sio_cap) -> c_int;
    pub fn sio_onmove(hdl: *mut sio_hdl, cb: SioOnmoveCb, arg: *mut c_void);
    pub fn sio_onxrun(hdl: *mut sio_hdl, cb: SioOnxrunCb, arg: *mut c_void);
    pub fn sio_write(hdl: *mut sio_hdl, addr: *const c_void, nbytes: size_t) -> size_t;
    pub fn sio_read(hdl: *mut sio_hdl, addr: *mut c_void, nbytes: size_t) -> size_t;
    pub fn sio_start(hdl: *mut sio_hdl) -> c_int;
    pub fn sio_stop(hdl: *mut sio_hdl) -> c_int;
    pub fn sio_flush(hdl: *mut sio_hdl) -> c_int;
    pub fn sio_nfds(hdl: *mut sio_hdl) -> c_int;
    pub fn sio_pollfd(hdl: *mut sio_hdl, pfd: *mut pollfd, events: c_int) -> c_int;
    pub fn sio_revents(hdl: *mut sio_hdl, pfd: *mut pollfd) -> c_int;
    pub fn sio_eof(hdl: *mut sio_hdl) -> c_int;
    pub fn sio_setvol(hdl: *mut sio_hdl, vol: c_uint) -> c_int;
    pub fn sio_onvol(hdl: *mut sio_hdl, cb: SioOnvolCb, arg: *mut c_void) -> c_int;

    pub fn mio_open(name: *const c_char, mode: c_uint, nbio_flag: c_int) -> *mut mio_hdl;
    pub fn mio_close(hdl: *mut mio_hdl);
    pub fn mio_write(hdl: *mut mio_hdl, addr: *const c_void, nbytes: size_t) -> size_t;
    pub fn mio_read(hdl: *mut mio_hdl, addr: *mut c_void, nbytes: size_t) -> size_t;
    pub fn mio_nfds(hdl: *mut mio_hdl) -> c_int;
    pub fn mio_pollfd(hdl: *mut mio_hdl, pfd: *mut pollfd, events: c_int) -> c_int;
    pub fn mio_revents(hdl: *mut mio_hdl, pfd: *mut pollfd) -> c_int;
    pub fn mio_eof(hdl: *mut mio_hdl) -> c_int;

    pub fn sioctl_open(name: *const c_char, mode: c_uint, flag: c_int) -> *mut sioctl_hdl;
    pub fn sioctl_close(hdl: *mut sioctl_hdl);
    pub fn sioctl_ondesc(hdl: *mut sioctl_hdl, cb: SioctlOndescCb, arg: *mut c_void) -> c_int;
    pub fn sioctl_onval(hdl: *mut sioctl_hdl, cb: SioctlOnvalCb, arg: *mut c_void) -> c_int;
    pub fn sioctl_setval(hdl: *mut sioctl_hdl, addr: c_uint, val: c_uint) -> c_int;
    pub fn sioctl_nfds(hdl: *mut sioctl_hdl) -> c_int;
    pub fn sioctl_pollfd(hdl: *mut sioctl_hdl, pfd: *mut pollfd, events: c_int) -> c_int;
    pub fn sioctl_revents(hdl: *mut sioctl_hdl, pfd: *mut pollfd) -> c_int;
    pub fn sioctl_eof(hdl: *mut sioctl_hdl) -> c_int;

    pub fn mio_rmidi_getfd(stri: *const c_char, mode: c_uint, nbio: c_int) -> c_int;
    pub fn mio_rmidi_fdopen(fd: c_int, mode: c_uint, nbio: c_int) -> *mut mio_hdl;
    pub fn sio_sun_getfd(stri: *const c_char, mode: c_uint, nbio: c_int) -> c_int;
    pub fn sio_sun_fdopen(fd: c_int, mode: c_uint, nbio: c_int) -> *mut sio_hdl;
    pub fn sioctl_sun_getfd(stri: *const c_char, mode: c_uint, nbio: c_int) -> c_int;
    pub fn sioctl_sun_fdopen(fd: c_int, mode: c_uint, nbio: c_int) -> *mut sioctl_hdl;
}

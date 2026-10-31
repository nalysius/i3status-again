//! The sec::openbsd module contains a wrapper calling pledge(2) and unveil(2).
//!
//! https://man.openbsd.org/pledge.2
//! https://man.openbsd.org/unveil.2

use crate::sec::expand_home;
use core::ffi::{c_char, c_int};
use std::ffi::CString;
use std::io::{self, Error};
use std::path::Path;

unsafe extern "C" {
    fn pledge(promises: *const c_char, execpromises: *const c_char) -> c_int;
    fn unveil(path: *const c_char, permissions: *const c_char) -> c_int;
}

/// A wrapper around pledge(2)
fn pledge_rs(promises: &str) -> io::Result<()> {
    let promises_c = CString::new(promises)?;
    let null: *const c_char = core::ptr::null();

    unsafe {
        match pledge(promises_c.as_ptr(), null) {
            0 => Ok(()),
            _ => Err(Error::last_os_error()),
        }
    }
}

/// A wrapper around unveil(2)
fn unveil_rs(path: &str, permissions: &str) -> io::Result<()> {
    let path_c = CString::new(path)?;
    let perm_c = CString::new(permissions)?;

    unsafe {
        match unveil(path_c.as_ptr(), perm_c.as_ptr()) {
            0 => Ok(()),
            _ => Err(Error::last_os_error()),
        }
    }
}

/// A wrapper around unveil(NULL, NULL).
pub fn unveil_disable() -> io::Result<()> {
    let null: *const c_char = core::ptr::null();

    unsafe {
        match unveil(null, null) {
            0 => Ok(()),
            _ => Err(Error::last_os_error()),
        }
    }
}

/// Calls pledge(2) and unveil(2).
pub fn apply_security(config_filepath: &str) {
    let home = expand_home().expect("Cannot read the value of $HOME.");
    // Ensure ~/.sndio exists, required to unveil ~/.sndio/cookie
    let sndio_dir = format!("{}/.sndio", home);
    std::fs::create_dir_all(&sndio_dir).expect("Cannot create ~/.sndio");

    pledge_rs("stdio rpath cpath vminfo unveil unix").expect("Error during pledge.");

    if Path::new(config_filepath).exists() {
        unveil_rs(config_filepath, "r").expect("Error during unveil.");
    }

    unveil_rs(&format!("{}/cookie", sndio_dir), "rwc").expect("Cannot unveil ~/.sndio.");
    unveil_rs("/var/run/sndio", "rw").expect("Cannot unveil /var/run/sndio.");
    unveil_rs("/tmp/sndio", "rw").expect("Cannot unveil /tmp/sndio.");
    unveil_rs("/etc/localtime", "r").expect("Cannot unveil /etc/localtime.");
    unveil_disable().expect("Cannot disable unveil.");
    pledge_rs("stdio rpath vminfo unix").expect("Error during pledge: removing unveil promise.");
}

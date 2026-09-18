//! The sensors::statvfs::openbsd module defines what's needed to use statvfs
//! on OpenBSD.

use libc::statvfs;
use std::error::Error;
use std::ffi::CString;
use std::fmt;
use std::mem::MaybeUninit;

/// An error that can occur using statvfs
#[derive(Debug)]
pub enum StatvfsError {
    /// The mount point associated with the path is not found.
    InvalidMountPoint,
    /// Search permission is denied for a component of the path prefix of path.
    Access,
    Other(std::io::Error),
}

impl fmt::Display for StatvfsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StatvfsError::InvalidMountPoint => write!(f, "Statvfs: invalid mount point."),
            StatvfsError::Access => write!(f, "Statvfs: permission error."),
            StatvfsError::Other(_) => write!(f, "Statvfs: other error."),
        }
    }
}

impl Error for StatvfsError {}

/// A wrapper around statvfs(3)
///
/// In case of success, it returns Ok containing:
/// - total_space: the total space on the disk / partition.
/// - used_space: the used space.
/// - used_percent: the proportion of used space on total_space.
/// - avail_space: the available space.
///
/// They are all in bytes.
///
/// Note: blocks reserved for root count as used, since the user will never
/// have access to it.
pub fn statvfs_rs(mount_point: &str) -> Result<(f64, f64, f64, f64), StatvfsError> {
    let mut buf = MaybeUninit::zeroed();
    let path = CString::new(mount_point).unwrap();
    let ret = unsafe { statvfs(path.as_ptr(), buf.as_mut_ptr() as *mut statvfs) };
    if ret != 0 {
        return match std::io::Error::last_os_error() {
            e if e.raw_os_error() == Some(libc::ENOTDIR) => Err(StatvfsError::InvalidMountPoint),
            e if e.raw_os_error() == Some(libc::ENAMETOOLONG) => {
                Err(StatvfsError::InvalidMountPoint)
            }
            e if e.raw_os_error() == Some(libc::ENOENT) => Err(StatvfsError::InvalidMountPoint),
            e if e.raw_os_error() == Some(libc::EACCES) => Err(StatvfsError::Access),
            e => Err(StatvfsError::Other(e)),
        };
    }

    let data: statvfs = unsafe { buf.assume_init() };

    let total_space: f64 = data.f_frsize as f64 * data.f_blocks as f64;
    let avail_space: f64 = data.f_frsize as f64 * data.f_bavail as f64;
    let used_space: f64 = total_space - avail_space;
    let used_percent: f64 = used_space / total_space * 100.0;
    Ok((total_space, used_space, used_percent, avail_space))
}

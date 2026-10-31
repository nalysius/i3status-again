//! The sec module contains wrappers around security features provided
//! by operating systems, like pledge(2) and unveil(2) for OpenBSD.
//!
//! Each OS has its submodule to keep things clear. As in the os module,
//! the submodule aren't exposed, only crate::sec is. Each OS submodule
//! defines a function named apply_security(&str), so it can be called
//! from the main() function without knowing what OS is used.

use std::env;
use std::io;

#[cfg(target_os = "openbsd")]
mod openbsd;
#[cfg(target_os = "openbsd")]
pub use crate::sec::openbsd::*;

#[cfg(not(any(target_os = "openbsd")))]
pub fn apply_security(config_filepath: &str) {}

/// Gets the value of the $HOME environment variable.
pub fn expand_home() -> io::Result<String> {
    if let Ok(home) = env::var("HOME") {
        return Ok(home);
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "HOME environment variable not set",
    ))
}

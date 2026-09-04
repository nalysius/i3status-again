//! The os module defines code specific for one OS.
//!
//! The submodules defined here, like battery, are used in backends to get
//! information. To continue with the example about battery, each supported OS
//! will define a submodule of battery, and each submodule will expose the same
//! methods. This way, when a backend uses crate::os:battery::*, it works without
//! worrying about the underlying OS.
//! This module provides abstraction.
//!
//! The dependency is as follow: backends can depend on os, which can depend on
//! sensors. Backends don't depend on sensors directly, so os must provide a
//! consistent abstraction.

pub mod battery;
pub mod cpu_freq;
pub mod cpu_temp;

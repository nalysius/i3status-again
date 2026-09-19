//! The sensors::sndio module defines a wrapper around sndio.
//! The sndio API should be the same on every OS, so it can be implemented
//! the same way everywhere.
//!
//! It's part of the base system only on OpenBSD, so it's used on OpenBSD by
//! default but is optional on other systems. On FreeBSD, Linux and NetBSD,
//! sndio can be used if i3status-again is compiled with the --features sndio
//! flag and the libsndio library is installed.

pub mod headers;

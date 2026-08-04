//! The backends::openbsd::datetime module implements the datetime
//! block for OpenBSD.

use crate::blocks::DateTimeBlock;

pub struct DateTimeBackend {
}

/// datetime is common between operating systems, so it's implemented
/// directly in the block.
impl DateTimeBlock for DateTimeBackend {
}

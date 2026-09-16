//! This example shows how to access information about the file system using
//! statvfs on OpenBSD.
//! See https://man.openbsd.org/statvfs.3
//! and https://docs.rs/libc/latest/libc/fn.sysctl.html

use i3status_again::sensors::statvfs::openbsd::*;

fn main() {
    let (_total_space, used_space, _used_percent, avail_space) = statvfs_rs("/home").unwrap();
    println!(
        "Available space: {:.1}GiB",
        avail_space / (1024.0 * 1024.0 * 1024.0)
    );
    println!(
        "Used space: {:.1}GiB",
        used_space / (1024.0 * 1024.0 * 1024.0)
    );
}

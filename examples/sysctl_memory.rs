//! This example shows how to query hw.physmem && hw.usermem using sysctl on
//! OpenBSD.
//! See https://man.openbsd.org/sysctl.2
//! and https://docs.rs/libc/latest/libc/fn.sysctl.html

use i3status_again::sensors::sysctl::openbsd::{CTL_VM, VM_UVMEXP, sysctl_uvmexp, uvmexp};

fn main() {
    let mib = [CTL_VM, VM_UVMEXP];
    let mem: uvmexp = sysctl_uvmexp(&mib).unwrap();

    println!("Page size: {}", mem.pagesize);
    println!("Number of pages: {}", mem.npages);
    println!("Number of free pages: {}", mem.free);
}

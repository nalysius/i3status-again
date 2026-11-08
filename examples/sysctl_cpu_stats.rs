//! This example shows how to loop over the CPU and read the stats
//! for the online ones.
//! See https://man.openbsd.org/sysctl.2
//! and https://docs.rs/libc/latest/libc/fn.sysctl.html

use i3status_again::sensors::sysctl::openbsd::{
    CPUSTATS_ONLINE, cpustats, sysctl_cpustats, sysctl_int,
};
use libc::{CTL_HW, CTL_KERN, HW_NCPU, KERN_CPUSTATS};

/// See the CP_* constants in <sys/sched.h> for the corresponding values.
const CS_NAMES: &[&str] = &["User", "Nice", "Sys", "Spin", "Intr", "Idle"];

fn main() {
    let mib = [CTL_HW, HW_NCPU];
    let ncpu = sysctl_int(&mib).unwrap();
    println!("{} CPU found.", ncpu);
    for n in 0..ncpu {
        let mib = [CTL_KERN, KERN_CPUSTATS, n];
        let stats: cpustats = sysctl_cpustats(&mib).unwrap();
        if stats.cs_flags & CPUSTATS_ONLINE as u64 == 1 {
            println!("CPU {} online.", n);
        } else {
            println!("CPU {} offline", n);
        }
        for (index, time) in stats.cs_time.iter().enumerate() {
            println!("{}: {}", CS_NAMES[index], time);
        }
        println!("-----");
    }
}

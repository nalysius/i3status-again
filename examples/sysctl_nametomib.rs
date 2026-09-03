//! This example shows how to use the sysctlnametomib function on OpenBSD.

use i3status_again::sensors::sysctl::openbsd::{SysctlError, sysctlnametomib};
use libc::c_int;

fn main() {
    let name = "hw.sensors.cpu0.frequency0";
    let mib: Result<Vec<c_int>, SysctlError> = sysctlnametomib(name);
    if let Err(e) = mib {
        println!("Error: {}", e.to_string());
        return;
    }
    let mib = mib.unwrap();
    println!("MIB: {:?}", mib);
}

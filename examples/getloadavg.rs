//! This example shows how to use the getloadavg.

use i3status_again::sensors::load::*;
use libc::c_int;

fn main() {
    let mut loadavg = [0.0, 0.0, 0.0];
    let length = loadavg.len();
    let ret = getloadavg(&mut loadavg, length as c_int);
    if let Some(e) = ret {
        println!("Error: cannot read load. {}", e);
    } else {
        println!("{:?}", loadavg);
    }
}

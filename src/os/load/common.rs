//! The os::load::common module implements a common wrapper around
//! getloadavg.

use crate::os::load::LoadOsError;
use crate::sensors::load::*;
use libc::c_int;

pub fn get_load_avg() -> Result<(f64, f64, f64), LoadOsError> {
    let mut loadavg = [0.0, 0.0, 0.0];
    let length = loadavg.len();
    if let Some(e) = getloadavg(&mut loadavg, length as c_int) {
        return Err(e.into());
    }
    return Ok((loadavg[0], loadavg[1], loadavg[2]));
}

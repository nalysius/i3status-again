//! The openbsd_constants module contains the constants used in OpenBSD to
//! access the sensors data.
//! 

pub const CTL_HW: i32 = 6;
pub const HW_SENSORS: i32 = 11;
pub const SENSOR_WATTHOUR: i32 = 14;

pub const MAXSENSORDEVICES: usize = 256;

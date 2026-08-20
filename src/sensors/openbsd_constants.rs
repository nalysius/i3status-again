//! The openbsd_constants module contains the constants used in OpenBSD to
//! access the sensors data.
//! 

const CTL_HW: i32 = 6;
const HW_SENSORS: i32 = 12;
const SENSOR_WATTHOUR: i32 = 14;

const MAXSENSORDEVICES: usize = 256;

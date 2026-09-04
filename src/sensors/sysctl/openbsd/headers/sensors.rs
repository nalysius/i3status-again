//! The sensors::sysctl::openbsd::sensors module contains the definition of
//! functions and structs used on OpenBSD to request sensors information.
//! See /usr/include/sys/sensors.h.

use libc::{c_int, size_t, timeval};

/// A sensor flag for sensor invalid
/// See /usr/include/sys/sensor.h:120
pub const SENSOR_FINVALID: c_int = 0x0001;
/// A sensor flag for sensor unknown
/// See /usr/include/sys/sensor.h:121
pub const SENSOR_FUNKNOWN: c_int = 0x0002;

// Match the sensor_state enum.
// See /usr/include/sys/sensors.h:100
pub const SENSOR_STATUS_UNSPEC: c_int = 0;
pub const SENSOR_STATUS_OK: c_int = 1;
pub const SENSOR_STATUS_WARN: c_int = 2;
pub const SENSOR_STATUS_CRIT: c_int = 3;
pub const SENSOR_STATUS_UNKNOWN: c_int = 4;

// Match the sensor_type enum
// See /usr/include/sys/sensors.h:33
pub const SENSOR_TYPE_TEMP: c_int = 0;
pub const SENSOR_TYPE_FANRPM: c_int = 1;
pub const SENSOR_TYPE_VOLTSDC: c_int = 2;
pub const SENSOR_TYPE_VOLTSAC: c_int = 3;
pub const SENSOR_TYPE_OHMS: c_int = 4;
pub const SENSOR_TYPE_WATTS: c_int = 5;
pub const SENSOR_TYPE_AMPS: c_int = 6;
pub const SENSOR_TYPE_WATTHOUR: c_int = 7;
pub const SENSOR_TYPE_AMPHOUR: c_int = 8;
pub const SENSOR_TYPE_INDICATOR: c_int = 9;
pub const SENSOR_TYPE_INTEGER: c_int = 10;
pub const SENSOR_TYPE_PERCENT: c_int = 11;
pub const SENSOR_TYPE_LUX: c_int = 12;
pub const SENSOR_TYPE_DRIVE: c_int = 13;
pub const SENSOR_TYPE_TIMEDELTA: c_int = 14;
pub const SENSOR_TYPE_HUMIDITY: c_int = 15;
pub const SENSOR_TYPE_FREQ: c_int = 16;
pub const SENSOR_TYPE_ANGLE: c_int = 17;
pub const SENSOR_TYPE_DISTANCE: c_int = 18;
pub const SENSOR_TYPE_PRESSURE: c_int = 19;
pub const SENSOR_TYPE_ACCEL: c_int = 20;
pub const SENSOR_TYPE_VELOCITY: c_int = 21;
pub const SENSOR_TYPE_ENERGY: c_int = 22;
pub const SENSOR_MAX_TYPES: c_int = 23;

/// A Sensor.
/// See /usr/include/sys/sensors.h:112
#[repr(C)]
#[derive(Clone, Copy)]
pub struct sensor {
    /// The description of the sensor (e.g.: remaining capacity).
    pub desc: [u8; 32],
    /// Datetime when the value was measured.
    pub timeval: timeval,
    /// The measured value.
    pub value: i64,
    /// The type of sensor.
    pub type_: c_int,
    /// The status of the sensor.
    pub status: c_int,
    /// The index of sensor. For example in hw.sensors.acpibat0.watthour3
    /// numt = 3.
    pub numt: c_int,
    /// SENSOR_* flags.
    pub flags: c_int,
}

/// A Sensor Device.
/// See /usr/include/sys/sensors.h:127
#[repr(C)]
#[derive(Clone, Copy)]
pub struct sensordev {
    /// SensorDev number.
    pub num: c_int,
    /// Unix device name.
    pub xname: [u8; 16],
    /// The number of sensors of this device, indexed by type.
    pub max_numt: [i32; SENSOR_MAX_TYPES as size_t],
    pub sensors_count: c_int,
}

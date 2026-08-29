//! The sensors::sysctl::openbsd::sensors module contains the definition of
//! functions and structs used on OpenBSD to request sensors information.
//! See /usr/include/sys/sensors.h.

// percentage = (hw.sensors.acpibat0.watthour3 + hw.sensors.acpibat1.watthour3) / (hw.sensors.acpibat0.watthour1 + hw.sensors.acpibat1.watthour1) * 100

use std::io;
use std::mem;
use std::ptr;

/// A sensor flag for sensor invalid
/// See /usr/include/sys/sensor.h:120
pub const SENSOR_FINVALID: i32 = 0x0001;
/// A sensor flag for sensor unknown
/// See /usr/include/sys/sensor.h:121
pub const SENSOR_FUNKNOWN: i32 = 0x0002;

/// The value of the last SensorType variant.
/// If [variant_count](https://doc.rust-lang.org/std/mem/fn.variant_count.html)
/// becomes stable in the future, use it to be safe in case OpenBSD adds a new
/// type of sensor.
/// See /usr/include/sys/sensor.h:57
const SENSOR_MAX_TYPES: i32 = 23;

/// Sensor states.
/// See /usr/include/sys/sensors.h:100
#[repr(C)]
pub enum SensorStatus {
	SENSOR_S_UNSPEC,
	SENSOR_S_OK,
	SENSOR_S_WARN,
	SENSOR_S_CRIT,
	SENSOR_S_UNKNOWN,
}

/// Sensor types
/// See /usr/include/sys/sensors.h:33
#[repr(C)]
pub enum SensorType {
	SENSOR_TEMP,			/* temperature (uK) */
	SENSOR_FANRPM,			/* fan revolution speed */
	SENSOR_VOLTS_DC,		/* voltage (uV DC) */
	SENSOR_VOLTS_AC,		/* voltage (uV AC) */
	SENSOR_OHMS,			/* resistance */
	SENSOR_WATTS,			/* power (uW) */
	SENSOR_AMPS,			/* current (uA) */
	SENSOR_WATTHOUR,		/* power capacity (uWh) */
	SENSOR_AMPHOUR,			/* power capacity (uAh) */
	SENSOR_INDICATOR,		/* boolean indicator */
	SENSOR_INTEGER,			/* generic integer value */
	SENSOR_PERCENT,			/* percent (m%) */
	SENSOR_LUX,			/* illuminance (ulx) */
	SENSOR_DRIVE,			/* disk */
	SENSOR_TIMEDELTA,		/* system time error (nSec) */
	SENSOR_HUMIDITY,		/* humidity (m%RH) */
	SENSOR_FREQ,			/* frequency (uHz) */
	SENSOR_ANGLE,			/* angle (uDegrees) */
	SENSOR_DISTANCE,		/* distance (uMeter) */
	SENSOR_PRESSURE,		/* pressure (mPa) */
	SENSOR_ACCEL,			/* acceleration (u m/s^2) */
	SENSOR_VELOCITY,		/* velocity (u m/s) */
	SENSOR_ENERGY,			/* energy (uJ) */
	SENSOR_MAX_TYPES
}

/// See /usr/include/sys/sensors.h:112
#[repr(C)]
struct Sensor {
	desc: [u8; 32],
	timeval: libc::timeval,
    value: i64,
	type_: SensorType,
	status: SensorStatus,
	numt: i32
    flags: i32,
}

/// See /usr/include/sys/sensors.h:127
#[repr(C)]
struct SensorDev {
	/// SensorDev number
    num: i32,
	/// Unix device name
    xname: [u8; 16],
    max_numt: [i32; SENSOR_MAX_TYPES],
    sensors_count: i32,
}


/*
/// Represents the quality of the sensor.
pub enum SensorQuality {
    Ok,
    Warning,
    Critical,
    Error,
    Unavailable,
}

/// Represents what kind of sensor has been requested.
pub enum SensorKind {
	/// °C
	Temperature,
	/// V
    Voltage,
	/// A
    Current,
	/// W
    Power,
	/// Wh
    Energy,
	/// Hz
    Frequency,
	/// Rounds / minute (fan)
    RPM,
    Unknown,
}

/// Represents a value read from a sensor.
pub struct SensorValue {
	/// Name of the device (e.g.: acpibat0)
    pub device: String,
	/// Name of the sensor (e.g.: watthour3)
    pub sensor: String,
	/// The kind of sensor (e.g.: SensorKind::Power)
    pub kind: SensorKind,
	/// The unit to display (e.g.: Wh)
    pub unit: &'static str,
	/// The raw value read from the sensor
    pub value: f64,
	/// The quality status
    pub quality: SensorQuality,
}


impl SensorValue {
	/// Get a human-readable label associated with the device.
	/// TODO: match partial strings, like "acpibat*" in case there are more
	/// batteries.
	pub fn get_device_label(&self) -> String {
		match self.device.as_str() {
			"acpibat0" => "BAT 1".to_string(),
			"acpibat1" => "BAT 2".to_string(),
			"acpibat2" => "BAT 3".to_string(),
			"cpu0" => "CPU 1".to_string(),
			"cpu1" => "CPU 2".to_string(),
			"cpu2" => "CPU 3".to_string(),
			"cpu3" => "CPU 4".to_string(),
			"cpu4" => "CPU 5".to_string(),
			"cpu5" => "CPU 6".to_string(),
			"cpu7" => "CPU 7".to_string(),
			"cpu8" => "CPU 8".to_string(),
			"nvme0" => "Disk 1".to_string(),
			"nvme1" => "Disk 2".to_string(),
			"nvme2" => "Disk 3".to_string(),
			_ => "".to_string(),
		}
	}
}



*/

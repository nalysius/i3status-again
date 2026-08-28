//! The sysctl module contains the definition of functions and structs,
//! used on OpenBSD, FreeBSD and NetBSD to request sensors information.

// percentage = (hw.sensors.acpibat0.watthour3 + hw.sensors.acpibat1.watthour3) / (hw.sensors.acpibat0.watthour1 + hw.sensors.acpibat1.watthour1) * 100

use std::io;
use std::mem;
use std::ptr;

/*
struct sensor {
	char desc[32];			/* sensor description, may be empty */
	struct timeval tv;		/* sensor value last change time */
	int64_t value;			/* current value */
	enum sensor_type type;		/* sensor type */
	enum sensor_status status;	/* sensor status */
	int numt;			/* sensor number of .type type */
	int flags;			/* sensor flags */
#define SENSOR_FINVALID		0x0001	/* sensor is invalid */
#define SENSOR_FUNKNOWN		0x0002	/* sensor value is unknown */
};
struct timeval { time_t tv_sec; suseconds_t tv_usec; };
*/

#[repr(C)]
struct Sensor {
	desc: [u8; 32],
	
    value: i64,
	type_: i32,
	
    flags: i32,
}

#[repr(C)]
struct SensorDev {
    num: i32,
    namelen: i32,
    name: [u8; 16], // MAXDRIVERSNAMELEN
    max_types: [i32; 32],
    sensors_count: i32,
}

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




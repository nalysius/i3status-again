//! The sensors::sysctl::openbsd::sensors module contains the definition of
//! functions and structs used on OpenBSD to request sensors information.
//! See /usr/include/sys/sensors.h.

// percentage = (hw.sensors.acpibat0.watthour3 + hw.sensors.acpibat1.watthour3) / (hw.sensors.acpibat0.watthour1 + hw.sensors.acpibat1.watthour1) * 100

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
pub const SENSOR_MAX_TYPES: usize = 23;

/// Sensor states.
/// See /usr/include/sys/sensors.h:100
#[repr(C)]
#[derive(Clone, Copy)]
pub enum SensorStatus {
	SensorSUnspec,
	SensorSOk,
	SensorSWarn,
	SensorSCrit,
	SensorSUnknown,
}

/// Sensor types
/// See /usr/include/sys/sensors.h:33
#[repr(C)]
#[derive(Clone, Copy)]
pub enum SensorType {
	/// Temperature (uK)
	SensorTemp,
	/// Fan revolution speed
	SensorFanrpm,
	/// Voltage (uV DC)
	SensorVoltsDc,
	/// Voltage (uV AC)
	SensorVoltsAc,
	/// Resistance
	SensorOhms,
	/// Power (uW)
	SensorWatts,
	/// Current (uA)
	SensorAmps,
	/// Power capacity (uWh)
	SensorWatthour,
	/// Power capacity (uAh)
	SensorAmphour,
	/// Boolean indicator
	SensorIndicator,
	/// Generic integer value
	SensorInteger,
	/// Percent (m%)
	SensorPercent,
	/// Illuminance (ulx)
	SensorLux,
	/// Disk
	SensorDrive,
	/// System time error (nSec)
	SensorTimedelta,
	/// Humidity (m%RH)
	SensorHumidity,
	/// Frequency (uHz)
	SensorFreq,
	/// Angle (uDegrees)
	SensorAngle,
	/// Distance (uMeter)
	SensorDistance,
	/// Pressure (mPa)
	SensorPressure,
	/// Acceleration (u m/s^2)
	SensorAccel,
	/// Velocity (u m/s)
	SensorVelocity,
	/// Energy
	SensorEnergy,
	SensorMaxType
}

impl ToString for SensorType {
	/// Convert a SensorType to a string
	fn to_string(&self) -> String {
		match &self {
			SensorType::SensorTemp => "temp".to_string(),
			SensorType::SensorFanrpm => "fan".to_string(),
			SensorType::SensorVoltsDc => "volt".to_string(),
			SensorType::SensorVoltsAc => "acvolt".to_string(),
			SensorType::SensorOhms => "resistance".to_string(),
			SensorType::SensorWatts => "power".to_string(),
			SensorType::SensorAmps => "current".to_string(),
			SensorType::SensorWatthour => "watthour".to_string(),
			SensorType::SensorAmphour => "amphour".to_string(),
			SensorType::SensorIndicator => "indicator".to_string(),
			SensorType::SensorInteger => "raw".to_string(),
			SensorType::SensorPercent => "percent".to_string(),
			SensorType::SensorLux => "illuminance".to_string(),
			SensorType::SensorDrive => "drive".to_string(),
			SensorType::SensorTimedelta => "timedelta".to_string(),
			SensorType::SensorHumidity => "humidity".to_string(),
			SensorType::SensorFreq => "frequency".to_string(),
			SensorType::SensorAngle => "angle".to_string(),
			SensorType::SensorDistance => "distance".to_string(),
			SensorType::SensorPressure => "pressure".to_string(),
			SensorType::SensorAccel => "acceleration".to_string(),
			SensorType::SensorVelocity => "velocity".to_string(),
			SensorType::SensorEnergy => "energy".to_string(),
			SensorType::SensorMaxType => "undefined".to_string(),
		}
	}
}

/// A Sensor.
/// See /usr/include/sys/sensors.h:112
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Sensor {
	pub desc: [u8; 32],
	pub timeval: libc::timeval,
    pub value: i64,
	pub type_: SensorType,
	pub status: SensorStatus,
	pub numt: i32,
    pub flags: i32,
}

/// A Sensor Device.
/// See /usr/include/sys/sensors.h:127
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SensorDev {
	/// SensorDev number
    pub num: i32,
	/// Unix device name
    pub xname: [u8; 16],
	/// Max number of sensors for this device
    pub max_numt: [i32; SENSOR_MAX_TYPES],
    pub sensors_count: i32,
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

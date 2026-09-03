//! The sensors::sysctl::openbsd::sensors module contains the definition of
//! functions and structs used on OpenBSD to request sensors information.
//! See /usr/include/sys/sensors.h.

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
    SensorTemp = 0,
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
    SensorMaxType,
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
    /// The description of the sensor (e.g.: remaining capacity).
    pub desc: [u8; 32],
    /// Datetime when the value was measured.
    pub timeval: libc::timeval,
    /// The measured value.
    pub value: i64,
    /// The type of sensor.
    pub type_: SensorType,
    /// The status of the sensor.
    pub status: SensorStatus,
    /// The index of sensor. For example in hw.sensors.acpibat0.watthour3
    /// numt = 3.
    pub numt: i32,
    /// SENSOR_* flags.
    pub flags: i32,
}

impl Sensor {
    /// Get the sensor description as a string.
    pub fn get_desc(&self) -> String {
        String::from_utf8(self.desc.to_vec())
            .unwrap()
            .trim_matches(char::from(0))
            .to_string()
    }
}

/// A Sensor Device.
/// See /usr/include/sys/sensors.h:127
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SensorDev {
    /// SensorDev number.
    pub num: i32,
    /// Unix device name.
    pub xname: [u8; 16],
    /// The number of sensors of this device, indexed by type.
    pub max_numt: [i32; SENSOR_MAX_TYPES],
    pub sensors_count: i32,
}

impl SensorDev {
    /// Get the identifier of the device.
    /// Example: for acpibat0 it returns 0.
    /// Default to u8::MAX
    pub fn get_id(&self) -> u8 {
        self.get_name()
            .chars()
            .rev()
            .take_while(|c| c.is_ascii_digit())
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect::<String>()
            .parse()
            .unwrap_or(u8::MAX)
    }

    /// Get the device name as a String
    pub fn get_name(&self) -> String {
        String::from_utf8(self.xname.to_vec())
            .unwrap()
            .trim_end_matches(char::from(0))
            .to_string()
    }
}

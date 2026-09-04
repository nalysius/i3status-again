//! The sensors::sysctl::openbsd module defines what's needed to use sysctl
//! on OpenBSD.

use libc::{c_int, c_void, size_t, sysctl};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt;
use std::mem::MaybeUninit;
use std::string::ToString;

pub mod headers;

pub use crate::sensors::sysctl::openbsd::headers::sensors::*;
pub use crate::sensors::sysctl::openbsd::headers::sysctl::*;
pub use crate::sensors::sysctl::openbsd::headers::uvmexp::*;

/// An error that can occurs using sysctl
#[derive(Debug)]
pub enum SysctlError {
    /// The item associated with the MIB is not found.
    NotFound,
    /// The address of name, oldp, newp, or length pointer oldlenp is invalid.
    InvalidAddress,
    /// The MIB is less than two or greater than CTL_MAXNAME. Or
    /// a non-null newp pointer is given and its specified length in newlen is
    /// too large or too small.
    InvalidName,
    /// The size of the data is invalid, either too long or too short.
    /// Contains the expected size and the actual size, in this order.
    InvalidSize(usize, usize),
    Other(std::io::Error),
}

impl fmt::Display for SysctlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            SysctlError::NotFound => write!(f, "Sysctl: name not found."),
            SysctlError::InvalidAddress => write!(f, "Sysctl: invalid address."),
            SysctlError::InvalidName => write!(f, "Sysctl: invalid name."),
            SysctlError::InvalidSize(_, _) => {
                write!(f, "Sysctl: sensor or sensordev has an invalid size.")
            }
            SysctlError::Other(_) => write!(f, "Sysctl: other error."),
        }
    }
}

impl Error for SysctlError {}

/// Sensor device types
/// Not from sensors.h.
#[derive(PartialEq)]
pub enum SensorDevType {
    /// The device is a battery.
    SensorDevBattery,
    /// The device is a CPU.
    SensorDevCpu,
}

impl<'a> TryFrom<&str> for SensorDevType {
    type Error = String;

    /// Try to convert a string to a SensorDevType
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.starts_with("acpibat") {
            return Ok(Self::SensorDevBattery);
        } else if value.starts_with("cpu") {
            return Ok(Self::SensorDevCpu);
        }
        Err("No match".to_string())
    }
}

/// A wrapper around sysctl to get all sensors maching the types.
///
/// To filter the returned sensors, one can use the sensor.desc field.
/// For example when requesting the watthours of the batteries, instead
/// of depending on the index (e.g.: watthour3), using the description
/// "remaining capacity" should be more flexible.
///
/// ## Examples
///
/// To get all the watthours of all batteries:
///   let sensors_res = sysctl_sensors(SensorDevType::SensorDevBattery, SENSOR_TYPE_WATTHOUR);
///
/// To get all the frequencies of all CPUs:
///   let sensors_res = sysctl_sensors(SensorDevType::SensorDevCpu, SENSOR_TYPE_FREQ);
pub fn sysctl_sensors(
    device_type: SensorDevType,
    sensor_type: c_int,
) -> Result<Vec<(sensordev, sensor)>, SysctlError> {
    let mut sensors: Vec<(sensordev, sensor)> = Vec::new();
    // Loop over all the sensor devices
    let mut device_id = 0;
    loop {
        let mib = [CTL_HW, HW_SENSORS, device_id];
        let device: sensordev = match sysctl_sensordev(&mib) {
            Err(SysctlError::NotFound) => return Ok(sensors),
            Err(x) => return Err(x),
            Ok(d) => d,
        };

        let device_name = get_sensordev_name(&device);
        let found_device_t = SensorDevType::try_from(device_name.as_str());

        // No need to enumerate the sensors if the device doesn't have the
        // right type.
        if found_device_t.is_err() || found_device_t.unwrap() != device_type {
            device_id += 1;
            continue;
        }

        // Loop over the device' sensors for the right type
        let sensor_number = device.max_numt[sensor_type as usize];
        for sensor_id in 0..sensor_number {
            let mib = [CTL_HW, HW_SENSORS, device_id, sensor_type, sensor_id];
            let sensor: sensor = sysctl_sensor(&mib)?;
            sensors.push((device, sensor));
        }
        device_id += 1;
    }
}

/// A wrapper around sysctl to get a sensor.
pub fn sysctl_sensor(mib: &[c_int]) -> Result<sensor, SysctlError> {
    sysctl_fixed(mib)
}

/// A wrapper around sysctl to get a sensordev.
pub fn sysctl_sensordev(mib: &[c_int]) -> Result<sensordev, SysctlError> {
    sysctl_fixed(mib)
}

/// A wrapper around sysctl to get a uvmexp.
pub fn sysctl_uvmexp(mib: &[c_int]) -> Result<uvmexp, SysctlError> {
    sysctl_fixed(mib)
}

/// A wrapper around sysctl.
///
/// Works for any type with a fixed size, like sensor or sensordev.
/// DON'T use it to query a String or any type with a dynamic size.
fn sysctl_fixed<T: Copy>(mib: &[c_int]) -> Result<T, SysctlError> {
    let mut size = size_of::<T>();
    let mut buf = MaybeUninit::<T>::uninit();
    let ret: c_int;
    unsafe {
        ret = sysctl(
            mib.as_ptr(),                    // name
            mib.len() as u32,                // namelen
            buf.as_mut_ptr() as *mut c_void, // oldp
            &mut size,                       //oldlenp
            std::ptr::null_mut(),            // newp
            0 as size_t,                     // newlen
        );
    }

    if ret != 0 {
        return match std::io::Error::last_os_error() {
            e if e.raw_os_error() == Some(libc::ENOENT) => Err(SysctlError::NotFound),
            e if e.raw_os_error() == Some(libc::EFAULT) => Err(SysctlError::InvalidAddress),
            e if e.raw_os_error() == Some(libc::EINVAL) => Err(SysctlError::InvalidName),
            e => Err(SysctlError::Other(e)),
        };
    }

    if size != size_of::<T>() {
        return Err(SysctlError::InvalidSize(size_of::<T>(), size));
    }

    Ok(unsafe { buf.assume_init() })
}

/// Get a sysctl MIB from a name.
///
/// Works only for hw.sensors.*, we shouldn't need anothing else in this
/// project.
///
/// sysctl cannot be used with "hw.sensors.cpu0.temp0", it required an MIB.
/// this function searches in the list to find the corresponding one.
pub fn sysctlnametomib(name: &str) -> Result<Vec<c_int>, SysctlError> {
    // Loop over all the sensor devices
    let mut device_id = 0;
    loop {
        let mib = [CTL_HW, HW_SENSORS, device_id];

        let device: sensordev = sysctl_sensordev(&mib)?;
        // The device name has a length of 16, so the null chars used to fill the
        // string need to be trimmed.
        let device_name: String = get_sensordev_name(&device);

        // Loop over the device' sensors
        // sensordev.max_numt is index by type of sensor.
        // See sensors::sysctl::openbsd::headers::SENSOR_TYPE_* constants.
        for sensor_type_id in 0..SENSOR_MAX_TYPES {
            let sensor_number = device.max_numt[sensor_type_id as usize];
            for sensor_id in 0..sensor_number {
                let mib = [
                    CTL_HW,
                    HW_SENSORS,
                    device_id,
                    sensor_type_id.try_into().unwrap(),
                    sensor_id,
                ];
                let _sensor: sensor = sysctl_sensor(&mib)?;
                // The values are raw. Example: hw.sensors.cpu0.temp is in
                // micro Kelvin, not Celsius.
                let sensor_name = sensor_type_tostring(sensor_type_id.try_into().unwrap());
                let found_name =
                    &format!("hw.sensors.{}.{}{}", device_name, sensor_name, sensor_id);
                if found_name == name {
                    return Ok(vec![
                        CTL_HW,
                        HW_SENSORS,
                        device_id,
                        sensor_type_id.try_into().unwrap(),
                        sensor_id,
                    ]);
                }
            }
        }
        device_id += 1;
    }
}

/// Get the string representation of a sensor type.
pub fn sensor_type_tostring(sensor_type: c_int) -> String {
    match sensor_type {
        SENSOR_TYPE_TEMP => "temp".to_string(),
        SENSOR_TYPE_FANRPM => "fan".to_string(),
        SENSOR_TYPE_VOLTSDC => "volt".to_string(),
        SENSOR_TYPE_VOLTSAC => "acvolt".to_string(),
        SENSOR_TYPE_OHMS => "resistance".to_string(),
        SENSOR_TYPE_WATTS => "power".to_string(),
        SENSOR_TYPE_AMPS => "current".to_string(),
        SENSOR_TYPE_WATTHOUR => "watthour".to_string(),
        SENSOR_TYPE_AMPHOUR => "amphour".to_string(),
        SENSOR_TYPE_INDICATOR => "indicator".to_string(),
        SENSOR_TYPE_INTEGER => "raw".to_string(),
        SENSOR_TYPE_PERCENT => "percent".to_string(),
        SENSOR_TYPE_LUX => "illuminance".to_string(),
        SENSOR_TYPE_DRIVE => "drive".to_string(),
        SENSOR_TYPE_TIMEDELTA => "timedelta".to_string(),
        SENSOR_TYPE_HUMIDITY => "humidity".to_string(),
        SENSOR_TYPE_FREQ => "frequency".to_string(),
        SENSOR_TYPE_ANGLE => "angle".to_string(),
        SENSOR_TYPE_DISTANCE => "distance".to_string(),
        SENSOR_TYPE_PRESSURE => "pressure".to_string(),
        SENSOR_TYPE_ACCEL => "acceleration".to_string(),
        SENSOR_TYPE_VELOCITY => "velocity".to_string(),
        SENSOR_TYPE_ENERGY => "energy".to_string(),
        _ => "undefined".to_string(),
    }
}

/// Get the description of a sensor as a String.
pub fn get_sensor_desc(sensor: &sensor) -> String {
    String::from_utf8(sensor.desc.to_vec())
        .unwrap()
        .trim_matches(char::from(0))
        .to_string()
}

/// Get the name of the sensordev.
pub fn get_sensordev_name(sensordev: &sensordev) -> String {
    String::from_utf8(sensordev.xname.to_vec())
        .unwrap()
        .trim_end_matches(char::from(0))
        .to_string()
}

/// Get the id of the sensordev, extracted from the name.
pub fn get_sensordev_id(sensordev: &sensordev) -> u8 {
    get_sensordev_name(&sensordev)
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

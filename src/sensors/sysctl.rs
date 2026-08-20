//! The sysctl module contains the definition of functions and structs,
//! used on OpenBSD, FreeBSD and NetBSD to request sensors information.

// percentage = (hw.sensors.acpibat0.watthour3 + hw.sensors.acpibat1.watthour3) / (hw.sensors.acpibat0.watthour1 + hw.sensors.acpibat1.watthour1) * 100

use std::io;
use std::mem;
use std::ptr;
use super::openbsd_constants::*;


#[repr(C)]
struct Sensor {
    value: i64,
    warning: i64,
    critical: i64,
    type_: i32,
    flags: i32,
    desc: [u8; 32],
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
			"cpu7" => "CPU 8".to_string(),
			"nvme0" => "Disk 1".to_string(),
			"nvme1" => "Disk 2".to_string(),
			"nvme2" => "Disk 3".to_string(),
			_ => "".to_string(),
		}
	}
}

/// Calls sysctl(2) with the given MIB and returns
/// the raw bytes of the result.
///
/// `mib` is a slice of integers representing the MIB path,
/// e.g. &[libc::CTL_HW, libc::HW_SENSORS].
pub fn sysctl_raw(mib: &[i32]) -> io::Result<Vec<u8>> {
    // Query the size of the data.
    let mut len: libc::size_t = 0;
    let rc = unsafe {
        libc::sysctl(
            mib.as_ptr() as *mut libc::c_int,
            mib.len() as libc::c_uint,
            ptr::null_mut(),
            &mut len,
            ptr::null(),
            0,
        )
    };
    if rc != 0 {
        return Err(io::Error::last_os_error());
    }

    if len == 0 {
        return Ok(Vec::new());
    }

    // Allocate a buffer of the right size.
    let mut buf = vec![0u8; len];

    // Query the actual data.
    let rc = unsafe {
        libc::sysctl(
            mib.as_ptr() as *mut libc::c_int,
            mib.len() as libc::c_uint,
            buf.as_mut_ptr() as *mut libc::c_void,
            &mut len,
            ptr::null(),
            0,
        )
    };
    if rc != 0 {
        return Err(io::Error::last_os_error());
    }

    // The kernel may have written fewer bytes than requested.
    buf.truncate(len);
    Ok(buf)
}

/// Sysctl wrapper that returns a String.
pub fn sysctl_string(mib: &[i32]) -> io::Result<String> {
    let buf = sysctl_raw(mib)?;
    // sysctl strings are null-terminated
    let end = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
    String::from_utf8(buf[..end].to_vec())
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

/// Sysctl wrapper that returns an integer
pub fn sysctl_int(mib: &[i32]) -> io::Result<i32> {
    let buf = sysctl_raw(mib)?;
    if buf.len() < mem::size_of::<i32>() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "buffer too small for i32",
        ));
    }
    let arr: [u8; 4] = buf[..4].try_into().unwrap();
    Ok(i32::from_ne_bytes(arr))
}

pub fn sysctl_f64(mib: &[i32]) -> io::Result<f64> {
    let buf = sysctl_raw(mib)?;

    if buf.len() < mem::size_of::<Sensor>() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "buffer too small for struct sensor",
        ));
    }

    let sensor = unsafe {
        ptr::read_unaligned(buf.as_ptr() as *const Sensor)
    };

    // OpenBSD stores values in micro-unots (1/1_000_000)
    Ok(sensor.value as f64 / 1_000_000.0)
}


fn find_device(name: &str) -> io::Result<i32> {
    for i in 0..MAXSENSORDEVICES as i32 {
        let mib = [CTL_HW, HW_SENSORS, i];
        match sysctl_raw(&mib) {
            Ok(buf) => {
                if buf.len() < mem::size_of::<SensorDev>() {
                    continue;
                }
                let dev = unsafe {
                    ptr::read_unaligned(buf.as_ptr() as *const SensorDev)
                };
                let dev_name = std::str::from_utf8(
                    &dev.name[..dev.namelen as usize]
                ).unwrap_or("");
                if dev_name == name {
                    return Ok(dev.num);
                }
            }
            Err(_) => continue,
        }
    }
    Err(io::Error::new(
        io::ErrorKind::NotFound,
        format!("sensor device '{}' not found", name),
    ))
}

/// Read hw.sensors.acpibatX.watthour3
fn read_battery_remaining(dev_name: &str) -> io::Result<f64> {
    let dev_num = find_device(dev_name)?;

    // MIB: [CTL_HW, HW_SENSORS, dev_num, SENSOR_WATTHOUR, 3]
    let mib = [CTL_HW, HW_SENSORS, dev_num, SENSOR_WATTHOUR, 3];
    sysctl_f64(&mib)
}

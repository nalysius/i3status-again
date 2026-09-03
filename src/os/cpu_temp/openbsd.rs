//! The os::cpu_temp::openbsd module implements the OpenBSD way to get the
//! temperature of the CPU. It provides the public functions required by the
//! cpu_temp.

use crate::common::TempUnit;
use crate::os::cpu_temp::*;
use crate::sensors::sysctl::openbsd::*;

/// Get the temperature of the CPU.
///
/// cpu_index is the index of the CPU to monitor. If None, all the CPUs
/// found are used to be displayed in a single block.
/// To monitor only cpu0 for example, give Some(0).
/// If you don't select a CPU or if you select one that has several temperature
/// sensors, the maximum temperature is returned.
///
/// unit is the target unit of temperature.
pub fn get_cpu_temp(cpu_index: Option<u8>, unit: TempUnit) -> Result<u8, CpuError> {
    let sensors = match sysctl_sensors(SensorDevType::SensorDevCpu, SensorType::SensorTemp) {
        Ok(v) => v,
        Err(e) => return Err(e.into()),
    };

    let mut max_temp: u8 = 0;
    let cpu_id: u8 = cpu_index.unwrap_or(u8::MAX);
    for (device, sensor) in sensors {
        let device_id = device.get_id();
        if cpu_id == device_id || cpu_index.is_none() {
            let mut temp: u8 = kelvin_to_celsius((sensor.value / 1_000_000) as u32);
            if unit == TempUnit::Fahrenheit {
                temp = celsius_to_fahrenheit(temp);
            }

            if temp > max_temp {
                max_temp = temp;
            }
        }
    }
    Ok(max_temp)
}

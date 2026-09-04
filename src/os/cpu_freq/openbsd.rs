//! The os::cpu_freq::openbsd module implements the OpenBSD way to get the
//! frequency of the CPU. If provides the public functions required by the
//! cpu_freq block.

use crate::common::{AggregatUnit, FreqUnit};
use crate::os::cpu_freq::*;
use crate::sensors::sysctl::openbsd::*;

pub fn get_cpu_freq(
    cpu_index: Option<u8>,
    unit: FreqUnit,
    aggregation: AggregatUnit,
) -> Result<f64, CpuFreqError> {
    let sensors = match sysctl_sensors(SensorDevType::SensorDevCpu, SENSOR_TYPE_FREQ) {
        Ok(v) => v,
        Err(e) => return Err(e.into()),
    };

    let mut frequencies: Vec<i64> = Vec::new();
    let cpu_id: u8 = cpu_index.unwrap_or(u8::MAX);
    for (device, sensor) in sensors {
        let device_id = get_sensordev_id(&device);
        if cpu_id == device_id || cpu_index.is_none() {
            frequencies.push(sensor.value);
        }
    }

    if frequencies.len() == 0 {
        return Err(CpuFreqError::CpuNotFound);
    }

    // Frequency in uHz
    let freq: f64 = match aggregation {
        AggregatUnit::Average => {
            let sum: i64 = frequencies.iter().sum();
            sum as f64 / (frequencies.len() as f64)
        }
        AggregatUnit::Maximum => *frequencies.iter().max().unwrap() as f64,
    };

    return match unit {
        FreqUnit::MHz => Ok(freq / 1_000_000.0 / 1_000_000.0),
        FreqUnit::GHz => Ok(freq / 1_000_000.0 / 1_000_000.0 / 1000.0),
    };
}

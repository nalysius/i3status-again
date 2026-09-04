use i3status_again::sensors::sysctl::openbsd::*;

fn main() {
    let sensors = sysctl_sensors(SensorDevType::SensorDevBattery, SENSOR_TYPE_WATTHOUR).unwrap();
    for (device, sensor) in sensors {
        let device_name: String = get_sensordev_name(&device);
        let sensor_name = sensor_type_tostring(SENSOR_TYPE_WATTHOUR);
        let sensor_desc: String = get_sensor_desc(&sensor);
        println!(
            "{}.{}{}: {} ({})",
            device_name, sensor_name, sensor.numt, sensor.value, sensor_desc
        );
    }
}

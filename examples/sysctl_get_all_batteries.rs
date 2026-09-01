use i3status_again::sensors::sysctl::openbsd::*;

fn main() {
	let sensors = sysctl_sensors(SensorDevType::SensorDevBattery, SensorType::SensorWatthour).unwrap();
	for (device, sensor) in sensors {
		let device_name: String = String::from_utf8(device.xname.to_vec())
			.unwrap()
			.trim_matches(char::from(0))
			.to_string();
		let sensor_name = sensor.type_.to_string();
		let sensor_desc: String = String::from_utf8(sensor.desc.to_vec())
			.unwrap();
		println!("{}.{}{} ({})", device_name, sensor_name, sensor.numt, sensor_desc);
	}
}

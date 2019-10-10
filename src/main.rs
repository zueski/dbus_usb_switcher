mod serial;
mod dbus;




fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("Hello, world!");
	dbus::enable_callback(
		move || { serial::disable("/dev/ttyUSB0".to_string())?; Ok(()) }, 
		move || { serial::enable("/dev/ttyUSB0".to_string())?; Ok(()) }
	)?;
	Ok(())
}


extern crate dbus;

use std::io;
use dbus::blocking::Connection;
use dbus::Error;
use std::time::Duration;

//andy@triton ~/Code/rustdbus $ /home/andy/.cargo/bin/dbus-codegen-rust -d org.xfce.ScreenSaver -p /org/xfce/ScreenSaver -m None
// This code was autogenerated with dbus-codegen-rust, see https://github.com/diwic/dbus-rs
use dbus::arg;

pub struct OrgXfceScreenSaverActiveChanged {
	pub new_value: bool,
}

impl arg::AppendAll for OrgXfceScreenSaverActiveChanged {
	fn append(&self, i: &mut arg::IterAppend) {
		arg::RefArg::append(&self.new_value, i);
	}
}

impl arg::ReadAll for OrgXfceScreenSaverActiveChanged {
	fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
		Ok(OrgXfceScreenSaverActiveChanged {
			new_value: i.read()?,
		})
	}
}

impl dbus::message::SignalArgs for OrgXfceScreenSaverActiveChanged {
	const NAME: &'static str = "ActiveChanged";
	const INTERFACE: &'static str = "org.xfce.ScreenSaver";
}

pub fn enable_callback(
		signal_enabled: fn() -> io::Result<()>,
		signal_disabled: fn() -> io::Result<()>
	) -> Result<(), Box<dyn std::error::Error>> {
	// First open up a connection to the session bus.
	let mut conn = Connection::new_session()?;
	
	{
		let proxy = conn.with_proxy("org.xfce.ScreenSaver", "/org/xfce/ScreenSaver", Duration::from_millis(5000));
		// Let's start listening to signals.
		
		let sub_id: Result<u32, Error> = proxy.match_signal(move |h: OrgXfceScreenSaverActiveChanged, _: &Connection| {
			println!("Activity changed: {}", h.new_value);
			if h.new_value
			{
				let r = signal_enabled();
				return r.is_ok()
			} else {
				let r = signal_disabled();
				return r.is_ok()
			}
//			true
		});
		match sub_id {
			Ok(id) => println!("Subscribed with id {}", id),
			Err(e) => println!("Error subscribing: {}", e),
		}
	}

	// Listen to incoming signals forever.
	loop {
		conn.process(Duration::from_millis(1000))?; 
	}
}
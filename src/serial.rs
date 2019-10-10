extern crate serial;

use std::io;
use std::time::Duration;
use std::io::Write;
use crate::serial::serial::SerialPort;

const COMMAND_BYTE_LOGO: &u8 = &0xA0u8;
const COMMAND_BYTE_ON: &u8 = &0x01u8;
const COMMAND_BYTE_OFF: &u8 = &0x00u8;

const COMMAND_ON: &[u8; 4] = &[0xA0u8,0x01u8,0x01u8,0xA2u8];
const COMMAND_OFF: &[u8; 4] = &[0xA0u8,0x01u8,0x00u8,0xA1u8];

const SETTINGS: serial::PortSettings = serial::PortSettings {
	baud_rate:    serial::Baud9600,
	char_size:    serial::Bits8,
	parity:       serial::ParityNone,
	stop_bits:    serial::Stop1,
	flow_control: serial::FlowNone,
};

pub fn enable(port_name: String) -> io::Result<()> {
	println!("Enabling {}", port_name);
	//write_command(port_name, &build_command(1, true))?;
	write_command(port_name, COMMAND_ON)?;
	Ok(())
}

pub fn disable(port_name: String) -> io::Result<()> {
	println!("disabling {}", port_name);
	//write_command(port_name, &build_command(1, false))?;
	write_command(port_name, COMMAND_OFF)?;
	Ok(())
}

fn write_command(port_name: String, command: &[u8]) -> io::Result<()> {
	let mut port = serial::open(&port_name).unwrap();
	port.configure(&SETTINGS)?;
	port.set_timeout(Duration::from_secs(1))?;
	port.write_all(command)?;
	Ok(())
}

fn build_command(switch: u8, state: bool) -> [u8; 4] {
	let state_byte = match state {
		true => *COMMAND_BYTE_ON,
		false => *COMMAND_BYTE_OFF,
	};
	let command_buf = [*COMMAND_BYTE_LOGO, switch, state_byte, 0xA2u8];
	return command_buf;
}

#![feature(macro_rules)]


use std::io::net::ip::Port;
use std::rand::random;


pub use self::process::Process;

pub mod macros;
pub mod process;


pub fn random_port(min: Port, max: Port) -> Port {
	random::<Port>() % (max - min) + min
}

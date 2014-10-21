use std::io::net::ip::Port;
use std::os;

use getopts::{
	getopts,
	optopt,
};


pub struct Args {
	pub root_path: Path,
	pub port     : Port,
}

impl Args {
	pub fn parse() -> Args {
		let args = os::args();

		let options = [
			optopt("r", "root", "the root directory", ""),
			optopt("p", "port", "the port to listen on", "80"),
		];

		let matches = match getopts(args.tail(), options) {
			Ok(matches) => matches,
			Err(error)  => fail!("Error parsing arguments: {}", error),
		};

		let root_path = match matches.opt_str("r") {
			Some(root_path) => Path::new(root_path),
			None =>
				fail!("You need to specific the root path with --root"),
		};

		let port = match matches.opt_str("p") {
			Some(port) => match from_str(port.as_slice()) {
				Some(port) => port,
				None =>
					fail!("Invalid value for port: {}", port),
			},
			None =>
				fail!("You need to specify the port with --port"),
		};

		Args {
			root_path: root_path,
			port     : port,
		}
	}
}

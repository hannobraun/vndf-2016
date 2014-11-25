use getopts::{
	getopts,
	optflag,
	optopt,
};
use std::io::net::ip::Port;


pub struct Args {
	pub headless: bool,
	pub port    : Port,
}

impl Args {
	pub fn parse(args: &[String]) -> Args {
		let opts = &[
			optflag("h", "headless", "enable headless mode"),
			optopt("p", "server-port", "server port to connect to", "34481"),
		];
		let matches = match getopts(args.tail(), opts) {
			Ok(matches) => matches,
			Err(error)  => panic!("Error parsing arguments: {}", error),
		};

		let port = match matches.opt_str("p") {
			Some(port) => from_str(port.as_slice()).unwrap(),
			None       => 34481,
		};

		Args {
			headless: matches.opt_present("h"),
			port    : port,
		}
	}
}

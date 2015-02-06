use std::old_io::net::ip::Port;

use getopts::Options;


pub struct Args {
	pub port: Port,
}

impl Args {
	pub fn parse<I>(cli_args: I) -> Args where I: Iterator<Item=String> {
		let mut args = Args {
			port: 34481,
		};

		let mut options = Options::new();
		options.optopt(
				"p",
				"port",
				"port to listen on",
				args.port.to_string().as_slice(),
			);

		let matches = match options.parse(cli_args) {
			Ok(matches) => matches,
			Err(error)  => panic!("Error parsing arguments: {}", error),
		};

		args.port = match matches.opt_str("p") {
			Some(port) => port.parse().unwrap(),
			None       => args.port,
		};

		args
	}
}
use std::old_io::net::ip::Port;

use getopts::Options;


pub struct Args {
	pub port: Port,
}

impl Args {
	pub fn parse(cli_args: &[String]) -> Args {
		let mut args = Args {
			port: 34481,
		};

		let mut opts = Options::new();
		opts.optopt(
				"p",
				"port",
				"port to listen on",
				args.port.to_string().as_slice(),
			);

		let matches = match opts.parse(cli_args.tail()) {
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
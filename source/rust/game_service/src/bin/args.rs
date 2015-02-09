use std::old_io::net::ip::Port;
use std::ffi::AsOsStr;

use getopts::Options;


pub struct Args {
	pub port: Port,
}

impl Args {
	pub fn parse<I>(cli_args: I) -> Args
		where
			I: Iterator,
			<I as Iterator>::Item: AsOsStr,
	{
		let mut args = Args {
			port: 34481,
		};

		let mut options = Options::new();
		options.optopt(
				"",
				"port",
				"port to listen on",
				args.port.to_string().as_slice(),
			);

		let matches = match options.parse(cli_args) {
			Ok(matches) => matches,
			Err(error)  => panic!("Error parsing arguments: {}", error),
		};

		args.port = match matches.opt_str("port") {
			Some(port) => port.parse().unwrap(),
			None       => args.port,
		};

		args
	}
}
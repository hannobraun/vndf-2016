use std::ffi::AsOsStr;

use getopts::Options;


pub struct Args {
	pub port            : u16,
	pub client_timeout_s: f64,
}

impl Args {
	pub fn parse<I>(cli_args: I) -> Args
		where
			I: Iterator,
			<I as Iterator>::Item: AsOsStr,
	{
		let mut args = Args {
			port            : 34481,
			client_timeout_s: 5.0,
		};

		let mut options = Options::new();
		options.optopt(
			"",
			"port",
			"port to listen on",
			args.port.to_string().as_slice(),
		);
		options.optopt(
			"",
			"client-timeout",
			"timeout after which a client is considered inactive (in seconds)",
			args.client_timeout_s.to_string().as_slice()
		);

		let matches = match options.parse(cli_args) {
			Ok(matches) => matches,
			Err(error)  => panic!("Error parsing arguments: {}", error),
		};

		args.port = match matches.opt_str("port") {
			Some(port) => port.parse().unwrap(),
			None       => args.port,
		};
		args.client_timeout_s = match matches.opt_str("client-timeout") {
			Some(timeout_s) => timeout_s.parse().unwrap(),
			None            => args.client_timeout_s,
		};

		args
	}
}
use std::env;

use getopts::Options;


pub struct Args {
	pub port            : u16,
	pub client_timeout_s: f64,
	pub sleep_ms        : u32,
}

impl Args {
	pub fn parse(cli_args: env::Args) -> Args {
		let mut args = Args {
			port            : 34481,
			client_timeout_s: 5.0,
			sleep_ms        : 500,
		};

		let mut options = Options::new();
		options.optopt(
			"",
			"port",
			"port to listen on",
			args.port.to_string().as_ref(),
		);
		options.optopt(
			"",
			"client-timeout",
			"timeout after which a client is considered inactive (in seconds)",
			args.client_timeout_s.to_string().as_ref()
		);
		options.optopt(
			"",
			"sleep-duration",
			"Length of the sleep in the main loop (in milliseconds)",
			args.sleep_ms.to_string().as_ref(),
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
		args.sleep_ms = match matches.opt_str("sleep-duration") {
			Some(duration) => duration.parse().unwrap(),
			None           => args.sleep_ms,
		};

		args
	}
}
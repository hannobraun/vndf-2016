use std::ffi::AsOsStr;
use std::time::Duration;

use getopts::Options;


pub struct Args {
	pub port            : u16,
	pub client_timeout_s: f64,
	pub sleep_duration  : Duration,
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
			sleep_duration  : Duration::milliseconds(500),
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
			args.sleep_duration.num_milliseconds().to_string().as_ref(),
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
		args.sleep_duration = match matches.opt_str("sleep-duration") {
			Some(duration) => Duration::milliseconds(duration.parse().unwrap()),
			None           => args.sleep_duration,
		};

		args
	}
}
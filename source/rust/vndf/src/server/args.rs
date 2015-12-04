use std::env;

use getopts::Options;


pub struct Args {
	pub port            : u16,
	pub client_timeout_s: f64,
	pub sleep_ms        : u64,
	pub initial_state   : String,
}

impl Args {
	pub fn default() -> Self {
		Args {
			port            : 34481,
			client_timeout_s: 5.0,
			sleep_ms        : 500,
			initial_state   : "".to_string(),
		}
	}

	pub fn parse(cli_args: env::Args) -> Args {
		let mut args = Args::default();

		let mut options = Options::new();
		options.optopt(
			"",
			"port",
			"port to listen on",
			&format!("{}", args.port),
		);
		options.optopt(
			"",
			"client-timeout",
			"timeout after which a client is considered inactive (in seconds)",
			&format!("{}", args.client_timeout_s),
		);
		options.optopt(
			"",
			"sleep-duration",
			"Length of the sleep in the main loop (in milliseconds)",
			&format!("{}", args.sleep_ms),
		);
		options.optopt(
			"",
			"initial-state",
			"Path of the initial state file to load",
			&args.initial_state,
		);

		let matches = match options.parse(cli_args) {
			Ok(matches) => matches,
			Err(error)  => panic!("Error parsing arguments: {}", error),
		};

		if let Some(port) = matches.opt_str("port") {
			args.port = port.parse().unwrap();
		}
		if let Some(timeout_s) = matches.opt_str("client-timeout") {
			args.client_timeout_s = timeout_s.parse().unwrap();
		}
		if let Some(duration) = matches.opt_str("sleep-duration") {
			args.sleep_ms = duration.parse().unwrap();
		}
		if let Some(initial_state) = matches.opt_str("initial-state") {
			args.initial_state = initial_state;
		}

		args
	}
}
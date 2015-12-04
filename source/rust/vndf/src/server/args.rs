use std::env;
use std::fmt::Debug;
use std::str::FromStr;

use getopts::{
	Matches,
	Options,
};


pub struct Args {
	pub port            : u16,
	pub client_timeout_s: f64,
	pub sleep_ms        : u64,
	pub initial_state   : Option<String>,
}

impl Args {
	pub fn default() -> Self {
		Args {
			port            : 34481,
			client_timeout_s: 5.0,
			sleep_ms        : 500,
			initial_state   : None,
		}
	}

	pub fn parse(cli_args: env::Args) -> Result<Self, String> {
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
			"PATH",
		);

		let matches = match options.parse(cli_args) {
			Ok(matches) => matches,
			Err(error)  => panic!("Error parsing arguments: {}", error),
		};

		parse_arg("port"          , &mut args.port            , &matches);
		parse_arg("client-timeout", &mut args.client_timeout_s, &matches);
		parse_arg("sleep-duration", &mut args.sleep_ms        , &matches);

		if let Some(initial_state) = matches.opt_str("initial-state") {
			args.initial_state = Some(initial_state);
		}

		Ok(args)
	}
}


fn parse_arg<T>(name: &str, target: &mut T, matches: &Matches)
	where
		T     : FromStr,
		T::Err: Debug,
{
	if let Some(arg) = matches.opt_str(name) {
		*target = arg.parse().unwrap();
	}
}

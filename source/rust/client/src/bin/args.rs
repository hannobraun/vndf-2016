use std::env;
use std::net::{
	SocketAddr,
	ToSocketAddrs,
};

use getopts::Options;


pub struct Args {
	pub headless      : bool,
	pub cli           : bool,
	pub server        : SocketAddr,
	pub net_timeout_s : f64,
	pub sleep_ms      : u32,
}

impl Args {
	pub fn parse(args: env::Args) -> Args {
		let mut options = Options::new();
		options.optflag(
			"",
			"headless",
			"enable headless mode",
		);
		options.optflag(
			"",
			"cli",
			"enable CLI mode",
		);
		options.optopt(
			"",
			"server-host",
			"server host to connect to",
			"localhost",
		);
		options.optopt(
			"",
			"server-port",
			"server port to connect to",
			"34481",
		);
		options.optopt(
			"",
			"network-timeout",
			"network timeout in seconds",
			"0.5"
		);
		options.optopt(
			"",
			"sleep-duration",
			"duration of main loop sleep in milliseconds",
			"20",
		);

		let matches = match options.parse(args) {
			Ok(matches) => matches,
			Err(error)  => panic!("Error parsing arguments: {}", error),
		};

		let host = match matches.opt_str("server-host") {
			Some(host) => host,
			None       => "localhost".to_string(),
		};
		let port: u16 = match matches.opt_str("server-port") {
			Some(port) => port.parse().unwrap(),
			None       => 34481,
		};
		let net_timeout_s = match matches.opt_str("network-timeout") {
			Some(timeout_s) => timeout_s.parse().unwrap(),
			None            => 5.0,
		};
		let sleep_duration_ms = match matches.opt_str("sleep-duration") {
			Some(duration_ms) => duration_ms.parse().unwrap(),
			None              => 20,
		};

		let server_address = (host.as_ref(), port);
		let server_address = match server_address.to_socket_addrs() {
			Ok(mut addresses) => match addresses.next() {
				Some(address) =>
					address,
				None =>
					panic!("Expected server address ({:?})", server_address),
			},
			Err(error) =>
				panic!(
					"Error parsing server address ({:?}): {}",
					server_address, error,
				),
		};

		Args {
			headless      : matches.opt_present("headless"),
			cli           : matches.opt_present("cli"),
			server        : server_address,
			net_timeout_s : net_timeout_s,
			sleep_ms      : sleep_duration_ms,
		}
	}
}

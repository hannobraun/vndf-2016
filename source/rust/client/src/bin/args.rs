use std::ffi::AsOsStr;
use std::old_io::net::ip::{
	Port,
	SocketAddr,
	ToSocketAddr,
};

use getopts::Options;


pub struct Args {
	pub headless     : bool,
	pub server       : SocketAddr,
	pub net_timeout_s: f64,
}

impl Args {
	pub fn parse<I>(args: I) -> Args
		where
			I: Iterator,
			<I as Iterator>::Item: AsOsStr,
	{
		let mut options = Options::new();
		options.optflag(
			"",
			"headless",
			"enable headless mode",
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

		let matches = match options.parse(args) {
			Ok(matches) => matches,
			Err(error)  => panic!("Error parsing arguments: {}", error),
		};

		let host = match matches.opt_str("server-host") {
			Some(host) => host,
			None       => "localhost".to_string(),
		};
		let port: Port = match matches.opt_str("server-port") {
			Some(port) => port.parse().unwrap(),
			None       => 34481,
		};
		let net_timeout_s = match matches.opt_str("network-timeout") {
			Some(timeout_s) => timeout_s.parse().unwrap(),
			None            => 5.0,
		};

		Args {
			headless     : matches.opt_present("headless"),
			server       : (host.as_slice(), port).to_socket_addr().unwrap(),
			net_timeout_s: net_timeout_s,
		}
	}
}

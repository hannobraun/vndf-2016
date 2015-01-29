use std::old_io::net::ip::{
	Port,
	SocketAddr,
	ToSocketAddr,
};

use getopts::Options;


pub struct Args {
	pub headless: bool,
	pub server  : SocketAddr,
}

impl Args {
	pub fn parse(args: &[String]) -> Args {
		let mut opts = Options::new();
		opts.optflag("h", "headless", "enable headless mode");
		opts.optopt(
				"h",
				"server-host",
				"server host to connect to",
				"localhost",
			);
		opts.optopt("p", "server-port", "server port to connect to", "34481");

		let matches = match opts.parse(args.tail()) {
			Ok(matches) => matches,
			Err(error)  => panic!("Error parsing arguments: {}", error),
		};

		let host = match matches.opt_str("h") {
			Some(host) => host,
			None       => "localhost".to_string(),
		};
		let port: Port = match matches.opt_str("p") {
			Some(port) => port.parse().unwrap(),
			None       => 34481,
		};

		Args {
			headless: matches.opt_present("h"),
			server  : (host.as_slice(), port).to_socket_addr().unwrap(),
		}
	}
}

use std::io;
use std::os;
use std::path;
use std::str;

use net;
use protocol;


pub struct Core {
	connection: protocol::Connection
}

impl Core {
	pub fn start() -> ~Core {
		let server_address = get_server_address();
		let socket_fd      = net::connect(server_address, ~"34481");

		~Core {
			connection: protocol::init(socket_fd) }
	}
}


fn get_server_address() -> ~str {
	let args = os::args();

	if args.len() > 2 {
		fail!("Usage: {:s} <server_address>\n", args[0]);
	}

	if args.len() == 2 {
		args[1]
	}
	else {
		let mut file = match io::File::open(&path::posix::Path::new("server")) {
			Ok(file) => file,
			Err(e)   => {
				print!("ERROR {}\n", e);
				fail!();
			}
		};

		let contents = match file.read_to_end() {
			Ok(contents) => contents,
			Err(e)       => {
				print!("ERROR {}\n", e);
				fail!();
			}
		};

		str::from_utf8(contents).unwrap_or_else(|| { fail!() }).to_owned()
	}
}

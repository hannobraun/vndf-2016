use std::io;
use std::os;
use std::path;
use std::str;


pub struct Core {
	server_address: ~str
}

impl Core {
	pub fn start() -> ~Core {
		~Core {
			server_address: get_server_address() }
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

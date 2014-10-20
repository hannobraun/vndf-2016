extern crate getopts;
extern crate time;

extern crate iron;
extern crate static_file;


use std::io::net::ip::Ipv4Addr;
use std::os;

use getopts::{
	getopts,
	optopt,
};
use iron::Iron;

use handler::RocksHandler;


mod handler;


fn main() {
	let args = os::args();

	let options = [
		optopt("r", "root", "the root directory", ""),
	];

	let matches = match getopts(args.tail(), options) {
		Ok(matches) => matches,
		Err(error)  => fail!("Error parsing arguments: {}", error),
	};

	let root_path = match matches.opt_str("r") {
		Some(root_path) => root_path,
		None =>
			fail!("You need to specific the root path with --root"),
	};


	Iron::new(
		RocksHandler::new(Path::new(root_path))
	)
	.listen(Ipv4Addr(127, 0, 0, 1), 3000);

	print!("Listening on port 3000\n");
}

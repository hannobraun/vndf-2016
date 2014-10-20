use std::os;

use getopts::{
	getopts,
	optopt,
};


pub fn parse() -> String {
	let args = os::args();

	let options = [
		optopt("r", "root", "the root directory", ""),
	];

	let matches = match getopts(args.tail(), options) {
		Ok(matches) => matches,
		Err(error)  => fail!("Error parsing arguments: {}", error),
	};

	match matches.opt_str("r") {
		Some(root_path) => root_path,
		None =>
			fail!("You need to specific the root path with --root"),
	}
}

use std::os;

use getopts::{
	getopts,
	optopt,
};


pub struct Args {
	pub root_path: String,
}

pub fn parse() -> Args {
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

	Args {
		root_path: root_path
	}
}

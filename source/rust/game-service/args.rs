use getopts::{
	getopts,
	optopt,
	usage
};
use std::os;


pub struct Args {
	pub port: ~str
}


pub fn parse() -> Option<Args> {
	let mut args = Args {
		port: ~"34481"
	};

	let args_as_strs = os::args();

	let options = [
		optopt("p", "port", "port to listen on", args.port)
	];

	let usage = usage(format!("{} [OPTIONS]", args_as_strs[0]), options);

	let matches = match getopts(args_as_strs.tail(), options) {
		Ok(matches) => matches,
		Err(fail)   => {
			print!("{}\n", fail.to_err_msg());
			print!("{}", usage);

			return None
		}
	};

	match matches.opt_str("p") {
		Some(port) => args.port = port,
		None       => ()
	}

	Some(args)
}

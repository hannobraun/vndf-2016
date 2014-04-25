use getopts::{
	getopts,
	optopt,
	usage
};
use std::os;


static default_port: &'static str = "34481";


pub fn port() -> Option<~str> {
	let args = os::args();

	let options = [
		optopt("p", "port", "port to listen on", default_port)
	];

	let usage = usage(format!("{} [OPTIONS]", args[0]), options);

	let matches = match getopts(args.tail(), options) {
		Ok(matches) => matches,
		Err(fail)   => {
			print!("{}\n", fail.to_err_msg());
			print!("{}", usage);

			return None
		}
	};

	match matches.opt_str("p") {
		Some(port) => Some(port),
		None       => Some(default_port.to_owned())
	}
}

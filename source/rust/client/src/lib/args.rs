use getopts::{
	getopts,
	optflag,
	optopt,
	usage
};
use std::os;


pub struct Args {
	pub address : String,
	pub port    : String,
	pub period  : u16,
	pub headless: bool
}


pub fn parse() -> Option<Args> {
	let mut args = Args {
		address : "localhost".to_string(),
		port    : "34481".to_string(),
		period  : 1000,
		headless: false
	};

	let args_as_strs: Vec<String> =
		os::args()
		.iter()
		.map(|s| s.to_string())
		.collect();

	let options = [
		optopt("a", "address", "server address"     , args.address.as_slice()),
		optopt("p", "port"   , "server port"        , args.port.as_slice()),
		optopt("f", "period" , "action period in ms", args.period.to_string().as_slice()),

		optflag("h", "headless", "start in headless mode")
	];

	let usage = usage(
		format!("{} [OPTIONS]", args_as_strs[0]).as_slice(),
		&options,
	);

	let matches = match getopts(args_as_strs.tail(), &options) {
		Ok(matches) => matches,
		Err(fail)   => {
			print!("{}\n", fail);
			print!("{}", usage);

			return None
		}
	};

	match matches.opt_str("a") {
		Some(address) => args.address = address,
		None          => ()
	}

	match matches.opt_str("p") {
		Some(port) => args.port = port,
		None       => ()
	}

	match matches.opt_str("f") {
		Some(period_str) => match from_str(period_str.as_slice()) {
			Some(period) => args.period = period,
			None         => return None
		},
		None => ()
	}

	args.headless = matches.opt_present("h");

	Some(args)
}

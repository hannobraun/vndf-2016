use getopts::{
	getopts,
	optopt,
	usage
};
use std::os;


pub struct Args {
	pub port         : String,
	pub frame_time   : u32,
	pub initial_state: String,
}


pub fn parse() -> Option<Args> {
	let mut args = Args {
		port         : "34481".to_string(),
		frame_time   : 1000,
		initial_state: "initial-state.json".to_string(),
	};

	let args_as_strs: Vec<String> = os::args().iter().map(|s| s.to_string()).collect();

	let options = [
		optopt("p", "port", "port to listen on", args.port.as_slice()),
		optopt("f", "frame-time", "frame time in ms", args.frame_time.to_string().as_slice()),
		optopt("s", "initial-state", "initial game state", args.initial_state.to_string().as_slice()),
	];

	let usage = usage(format!("{} [OPTIONS]", args_as_strs[0]).as_slice(), &options);

	let matches = match getopts(args_as_strs.as_slice(), &options) {
		Ok(matches) => matches,
		Err(fail)   => {
			print!("{}\n", fail);
			print!("{}", usage);

			return None
		}
	};

	match matches.opt_str("p") {
		Some(port) => args.port = port,
		None       => ()
	}

	match matches.opt_str("f") {
		Some(frame_time_as_str) => match from_str(frame_time_as_str.as_slice()) {
			Some(frame_time) => args.frame_time = frame_time,
			None             => {
				print!("{}\n", "Frame time must be a number");
				print!("{}", usage);
			}
		},

		None => ()
	}

	match matches.opt_str("s") {
		Some(initial_state) => args.initial_state = initial_state,
		None => (),
	}

	Some(args)
}

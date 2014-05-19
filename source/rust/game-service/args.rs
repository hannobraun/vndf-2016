use getopts::{
	getopts,
	optopt,
	usage
};
use std::os;


pub struct Args {
	pub port      : StrBuf,
	pub frame_time: u32
}


pub fn parse() -> Option<Args> {
	let mut args = Args {
		port      : "34481".to_strbuf(),
		frame_time: 1000
	};

	let args_as_strs: Vec<StrBuf> = os::args().iter().map(|s| s.to_strbuf()).collect();

	let options = [
		optopt("p", "port", "port to listen on", args.port.as_slice()),
		optopt("f", "frame-time", "frame time in ms", args.frame_time.to_str())
	];

	let usage = usage(format!("{} [OPTIONS]", args_as_strs.get(0)), options);

	let matches = match getopts(args_as_strs.as_slice(), options) {
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

	Some(args)
}

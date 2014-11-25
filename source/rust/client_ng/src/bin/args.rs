use getopts::{
	getopts,
	optflag,
};


pub struct Args {
	pub headless: bool,
}

impl Args {
	pub fn parse(args: &[String]) -> Args {
		let opts = &[
			optflag("h", "headless", "enable headless mode"),
		];
		let matches = match getopts(args.tail(), opts) {
			Ok(matches) => matches,
			Err(error)  => panic!("Error parsing arguments: {}", error),
		};

		Args {
			headless: matches.opt_present("h"),
		}
	}
}

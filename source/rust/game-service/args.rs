use std::os;


pub fn port() -> ~str {
	let args = os::args();

	if args.len() != 2 {
		fail!("Usage: {:s} <port>\n", args[0]);
	}

	args[1]
}

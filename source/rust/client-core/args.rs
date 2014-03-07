use std::os;


pub fn get_server_address() -> ~str {
	let args = os::args();

	if args.len() != 2 {
		fail!("Usage: {:s} <server_address>\n", args[0]);
	}

	args[1]
}

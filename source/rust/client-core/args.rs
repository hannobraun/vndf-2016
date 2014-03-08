use std::os;


pub fn address_and_port() -> (~str, ~str) {
	let args = os::args();

	if args.len() != 3 {
		fail!("Usage: {:s} <server_address> <port>\n", args[0]);
	}

	(args[1].clone(), args[2].clone())
}

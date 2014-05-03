use libc;
use std::io;


pub fn exit(message: &str) -> ! {
	match write!(&mut io::stderr(), "Fatal Error: {}\n", message) {
		Ok(_)  => (),
		Err(_) => ()
	}

	unsafe { libc::exit(1) };
}

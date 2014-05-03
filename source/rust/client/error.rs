use libc;

pub fn exit(message: &str) {
	print!("Fatal Error: {}\n", message);
	unsafe { libc::exit(1) };
}

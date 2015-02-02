use std::old_io::{
	Append,
	File,
	Write,
};


pub fn writer() -> File {
	File::open_mode(
		&Path::new("debug.out"),
		Append,
		Write,
	)
	.unwrap()
}

use std::fs::{
	File,
	OpenOptions,
};


pub fn writer() -> File {
	OpenOptions::new()
		.write(true)
		.append(true)
		.open(&Path::new("debug.out"))
		.unwrap()
}

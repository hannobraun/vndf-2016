extern crate nalgebra;
extern crate time;

extern crate client;
extern crate common;
extern crate test_suite;


mod component {
	mod client {
		mod input;
		mod protocol;
	}
	mod server {
		mod protocol;
	}
}
mod acceptance {
	mod basic;
	mod navigation;
}

extern crate nalgebra;
extern crate time;

extern crate shared;
extern crate tests;


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

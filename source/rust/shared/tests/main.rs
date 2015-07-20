extern crate nalgebra;
extern crate time;

extern crate shared;


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

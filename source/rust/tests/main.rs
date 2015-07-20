extern crate nalgebra;
extern crate time;

extern crate vndf;


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

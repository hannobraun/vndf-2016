extern crate nalgebra;
extern crate time;

extern crate vndf;


mod unit {
	mod server {
		mod game;
	}
}

mod integration {
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

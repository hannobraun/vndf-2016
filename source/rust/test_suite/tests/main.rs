extern crate time;

extern crate client;
extern crate common;
extern crate test_suite;


mod component {
	mod client {
		mod input;
		mod protocol;
	}
	mod game_service {
		mod protocol;
	}
}
mod acceptance {
	mod basic;
	mod navigation;
}

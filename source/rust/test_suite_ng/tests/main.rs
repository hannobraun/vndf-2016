extern crate acpe;
extern crate time;

extern crate client;
extern crate common;
extern crate test_tools;


mod component {
	mod client {
		mod input;
		mod protocol;
	}
	mod game_service {
		mod protocol;
		mod robustness;
	}
}
mod acceptance {
	mod basic;
}

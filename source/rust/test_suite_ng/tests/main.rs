#![feature(phase)]


extern crate acpe;
extern crate time;

extern crate client_ng;
extern crate common;
extern crate game_service_ng;
extern crate test_tools_ng;


mod unit {
	mod client {
		mod server;
	}
	mod game_service {
		mod socket;
	}
}
mod component {
	mod client {
		mod protocol;
	}
	mod game_service {
		mod protocol;
	}
}
mod acceptance {
	mod basic;
}

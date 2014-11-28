#![feature(phase)]


extern crate client_ng;
extern crate game_service_ng;
extern crate protocol_ng;
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
	mod server {
		mod protocol;
	}
	mod client {
		mod protocol;
	}
}
mod acceptance {
	mod basic;
}

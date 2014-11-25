#![feature(phase)]


extern crate collections;
extern crate serialize;
extern crate time;

#[phase(plugin)] extern crate acceptance;
extern crate cgmath;

extern crate client;
extern crate game;
extern crate game_service;
extern crate platform;
extern crate protocol;
extern crate rustecs;
#[phase(plugin)] extern crate rustecs_macros;
extern crate test_tools;


mod unit {
	mod client_test {
		mod receiver;
	}

	mod common_test {
		mod protocol;
	}
}

mod component {
	mod client;
}

mod acceptance {
	mod basic;
	mod ship;
}

mod acceptance_ng {
	mod basic;
}

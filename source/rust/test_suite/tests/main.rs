#![feature(collections, core)]


extern crate time;

extern crate client;
extern crate common;
extern crate test_suite;


mod component {
	mod client {
		mod input;
	}
	mod game_service {
		mod protocol;
	}
}
mod acceptance {
	mod basic;
}

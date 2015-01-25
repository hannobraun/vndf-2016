#![allow(unstable)]


extern crate acpe;
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
		mod robustness;
	}
}
mod acceptance {
	mod basic;
}

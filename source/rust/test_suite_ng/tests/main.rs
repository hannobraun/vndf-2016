#![feature(phase)]


extern crate acpe;
extern crate time;

extern crate client;
extern crate common;
extern crate test_tools;


mod unit {
	mod acpe {
		mod socket;
	}
}
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

#![feature(phase)]


extern crate collections;

extern crate rustecs;
#[phase(syntax)] extern crate rustecs_macros;
extern crate common;


mod unit {
	mod common_test {
		mod angle;
		mod vec;
	}

	mod ecs_test {
		mod entity;
	}
}

mod component {
	mod client;
}

mod acceptance {
	mod basic;
	mod ship;
}

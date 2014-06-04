#![feature(phase)]


extern crate collections;

extern crate common;
#[phase(syntax)] extern crate ecs_macros;


mod unit {
	mod common_test {
		mod angle;
		mod vec;
		mod ecs;
	}
}

mod component {
	mod client;
}

mod acceptance {
	mod basic;
	mod ship;
}

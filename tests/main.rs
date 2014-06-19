#![feature(macro_rules)]
#![feature(phase)]


extern crate collections;
extern crate serialize;
extern crate time;

extern crate rustecs;
#[phase(plugin)]
extern crate rustecs_macros;
extern crate common;


mod unit {
	mod common_test {
		mod angle;
		mod protocol;
		mod vec;
	}

	mod ecs_test {
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

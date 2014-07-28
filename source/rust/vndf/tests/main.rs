#![feature(phase)]


extern crate collections;
extern crate serialize;
extern crate time;

extern crate rustecs;
#[phase(plugin)]
extern crate rustecs_macros;
#[phase(plugin, link)]
extern crate vndf;


mod unit {
	mod client_test {
		mod receiver;
	}

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

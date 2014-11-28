#![feature(slicing_syntax)]


extern crate serialize;


pub use action::{
	Action,
	Step,
};
pub use perception::{
	Perception,
	PerceptionEnc,
};


mod action;
mod perception;


pub struct Encoder;

impl Encoder {
	pub fn new() -> Encoder {
		Encoder
	}

	pub fn perception(&mut self, last_action: u64) -> PerceptionEnc {
		PerceptionEnc::new(last_action)
	}
}

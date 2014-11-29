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


pub struct Encoder {
	buffer: [u8, ..512],
}

impl Encoder {
	pub fn new() -> Encoder {
		Encoder {
			buffer: [0, ..512],
		}
	}

	pub fn perception(&mut self, last_action: u64) -> PerceptionEnc {
		PerceptionEnc::new(&mut self.buffer, last_action)
	}
}

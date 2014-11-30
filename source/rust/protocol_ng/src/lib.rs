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


pub const MAX_PACKET_SIZE: uint = 512;


pub type Seq = u64;


pub struct Encoder {
	buffer: [u8, ..MAX_PACKET_SIZE],
}

impl Encoder {
	pub fn new() -> Encoder {
		Encoder {
			buffer: [0, ..MAX_PACKET_SIZE],
		}
	}

	pub fn perception(&mut self, last_action: Seq) -> PerceptionEnc {
		PerceptionEnc::new(&mut self.buffer, last_action)
	}
}

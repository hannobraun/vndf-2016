use std::old_io::IoResult;
use std::vec::Drain;

use client::interface::{
	Frame,
	InputEvent,
};


pub struct Cli {
	events: Vec<InputEvent>,
}

impl Cli {
	pub fn new() -> Cli {
		Cli {
			events: Vec::new(),
		}
	}

	pub fn update(&mut self, _: &Frame) -> IoResult<Drain<InputEvent>> {
		// TODO: Implement
		Ok(self.events.drain())
	}
}

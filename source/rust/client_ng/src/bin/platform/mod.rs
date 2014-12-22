use std::io::IoResult;

use client::platform::{
	Frame,
	Input,
};

use self::input::{
	InputReader,
	ReadInput,
};
use self::render::{
	Render,
	Renderer,
};


pub mod input;
pub mod render;


// TODO: Merge ReadInput and Render into PlatformIo
// TODO: Add constructor method
pub trait PlatformIo: ReadInput + Render {}


pub struct PlayerIo {
	input_reader: InputReader,
	renderer    : Renderer,
}

impl PlayerIo {
	pub fn new() -> IoResult<PlayerIo> {
		let input_reader = InputReader::new();
		let renderer = match Renderer::new() {
			Ok(renderer) => renderer,
			Err(error)   => return Err(error),
		};

		Ok(PlayerIo {
			input_reader: input_reader,
			renderer    : renderer,
		})
	}
}

impl ReadInput for PlayerIo {
	fn input(&mut self) -> Input {
		self.input_reader.input()
	}
}

impl Render for PlayerIo {
	fn render(&mut self, frame: &Frame) -> IoResult<()> {
		self.renderer.render(frame)
	}
}

impl PlatformIo for PlayerIo {}

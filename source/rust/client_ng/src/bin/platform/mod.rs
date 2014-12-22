use std::io::IoResult;

use client::platform::{
	Frame,
	Input,
};

use self::input::{
	HeadlessInputReader,
	InputReader,
	ReadInput,
};
use self::render::{
	HeadlessRenderer,
	Render,
	Renderer,
};


pub mod input;
pub mod render;


// TODO: Merge ReadInput and Render into PlatformIo
pub trait PlatformIo: ReadInput + Render {
	fn new() -> IoResult<Self>;
}


pub struct PlayerIo {
	input_reader: InputReader,
	renderer    : Renderer,
}

impl PlatformIo for PlayerIo {
	fn new() -> IoResult<PlayerIo> {
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


// TODO: Inline HeadlessInputRender and HeadlessRenderer
pub struct HeadlessIo {
	input_reader: HeadlessInputReader,
	renderer    : HeadlessRenderer,
}

impl PlatformIo for HeadlessIo {
	fn new() -> IoResult<HeadlessIo> {
		let input_reader = HeadlessInputReader::new();
		let renderer = match HeadlessRenderer::new() {
			Ok(renderer) => renderer,
			Err(error)   => return Err(error),
		};

		Ok(HeadlessIo {
			input_reader: input_reader,
			renderer    : renderer,
		})
	}
}

impl ReadInput for HeadlessIo {
	fn input(&mut self) -> Input {
		self.input_reader.input()
	}
}

impl Render for HeadlessIo {
	fn render(&mut self, frame: &Frame) -> IoResult<()> {
		self.renderer.render(frame)
	}
}

use common::io::Frame;

use io;


pub struct Renderer;

impl Renderer {
	pub fn new() -> Renderer {
		Renderer
	}
}

impl io::Renderer for Renderer {
	fn render(&self, frame: &Frame) {
		print!("{}\n", frame.to_json());
	}
}

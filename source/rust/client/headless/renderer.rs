use common::io;
use common::io::Frame;


pub struct Renderer;

impl Renderer {
	pub fn new() -> Renderer {
		Renderer
	}
}

impl io::Renderer for Renderer {
	fn render(&mut self, frame: &Frame) {
		print!("{}\n", frame.to_json());
	}
}

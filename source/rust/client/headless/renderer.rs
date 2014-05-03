use common::headless::Frame;

use components::Control;
use entities::Components;
use io;


pub struct Renderer;

impl Renderer {
	pub fn new() -> Renderer {
		Renderer
	}
}

impl io::Renderer for Renderer {
	fn render(&self, frame: &Frame,
		_: &Components<Control>) {

		print!("{}\n", frame.to_json());
	}
}

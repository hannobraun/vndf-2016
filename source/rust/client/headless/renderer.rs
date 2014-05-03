use common::headless::Frame;
use common::physics::{
	Body,
	Vec2
};

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
		_: Vec2,
		_: &Components<Control>,
		_: &Components<Body>) {

		print!("{}\n", frame.to_json());
	}
}

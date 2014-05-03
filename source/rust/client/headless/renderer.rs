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
	fn render(&self, _: &Frame,
		camera: Vec2,
		_     : &Components<Control>,
		bodies: &Components<Body>) {

		let frame = Frame {
			camera: camera,
			ships : bodies.values().map(|&x| x).collect()
		};

		print!("{}\n", frame.to_json());
	}
}

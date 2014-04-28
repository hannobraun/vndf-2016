use common::physics::{
	Body,
	Vec2
};

use components::{
	Control,
	Visual
};
use entities::Components;
use io;


pub struct Renderer;

impl Renderer {
	pub fn new() -> Renderer {
		Renderer
	}
}

impl io::Renderer for Renderer {
	fn render(&self,
		_: Vec2,
		_: &Components<Control>,
		_: &Components<Body>,
		_: &Components<Visual>) {

		print!("Yeah, fake rendering!\n");
	}
}

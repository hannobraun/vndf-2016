use common::physics::{
	Body,
	Vec2
};

use components::{
	Control,
	Visual
};
use entities::Components;


pub trait Input {
	fn apply(&self, controls: &mut Components<Control>);
}

pub trait Renderer {
	fn render(&self,
		camera  : Vec2,
		controls: &Components<Control>,
		bodies  : &Components<Body>,
		visuals : &Components<Visual>);
}

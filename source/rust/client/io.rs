use common::physics::{
	Body,
	Vec2
};

use components::Control;
use entities::Components;


pub trait Input {
	fn apply(&self, controls: &mut Components<Control>) -> bool;
}

pub trait Renderer {
	fn render(&self,
		camera  : Vec2,
		controls: &Components<Control>,
		bodies  : &Components<Body>);
}

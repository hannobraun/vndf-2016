use common::io::Frame;

use components::Control;
use entities::Components;


pub trait Input {
	fn apply(&self, controls: &mut Components<Control>) -> bool;
}

pub trait Renderer {
	fn render(&self, frame: &Frame,
		controls: &Components<Control>);
}

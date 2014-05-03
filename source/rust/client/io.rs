use common::io::Frame;
use common::io::Input;

use components::Control;
use entities::Components;


pub trait InputHandler {
	fn apply(&mut self, controls: &mut Components<Control>) -> Input;
}

pub trait Renderer {
	fn render(&self, frame: &Frame,
		controls: &Components<Control>);
}

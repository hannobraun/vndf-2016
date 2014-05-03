use common::io::Frame;
use common::io::Input;

pub trait InputHandler {
	fn apply(&mut self) -> Input;
}

pub trait Renderer {
	fn render(&self, frame: &Frame);
}

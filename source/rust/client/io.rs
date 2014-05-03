use common::io::Frame;

pub trait Renderer {
	fn render(&self, frame: &Frame);
}

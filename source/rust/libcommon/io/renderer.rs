use io::Frame;

pub trait Renderer {
	fn render(&mut self, frame: &Frame);
}

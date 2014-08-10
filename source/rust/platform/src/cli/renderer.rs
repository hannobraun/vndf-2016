use platform::Frame;


pub struct Renderer;

impl Renderer {
	pub fn new() -> Renderer {
		Renderer
	}

	pub fn render(&mut self, frame: &Frame) {
		print!("{}\n", frame.to_json());
	}
}

extern crate sync;

extern crate glfw;
extern crate glfw_platform;

extern crate platform;


use platform::{
	Frame,
	Input,
	Platform,
};


struct DesktopPlatform {
	glfw  : glfw::Glfw,
	window: glfw::Window,
}

impl Platform for DesktopPlatform {
	fn input(&mut self) -> Result<Input, String> {
		self.glfw.poll_events();

		let mut input = Input::default();
		input.exit = self.window.should_close();

		Ok(input)
	}

	fn render(&mut self, frame: &Frame) {}
}


pub fn init() -> Box<Platform> {
	let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

	let (mut window, events) = glfw_platform::WindowBuilder::new(&glfw)
		.title("Von Neumann Defense Force *EARLY PROTOTYPE*")
		.size(800, 600)
		.try_modern_context_hints()
		.create()
		.expect("failed to create window");

	box
		DesktopPlatform {
			glfw  : glfw,
			window: window,
		}
	as Box<Platform>
}

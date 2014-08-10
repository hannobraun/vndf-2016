extern crate glfw;
extern crate glfw_platform;

extern crate platform;


use platform::{
	Frame,
	Input,
	Platform,
};


struct DesktopPlatform;

impl Platform for DesktopPlatform {
	fn input(&mut self) -> Result<Input, String> {
		Ok(Input::default())
	}

	fn render(&mut self, frame: &Frame) {}
}


pub fn init() -> Box<Platform> {
	let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

	let (mut window, events) = glfw_platform::WindowBuilder::new(&glfw)
		.title("Von Neumann Defense Force *EARLY PROTOTYPE*")
		.try_modern_context_hints()
		.create()
		.expect("failed to create window");

	print!("!\n");
	box DesktopPlatform as Box<Platform>
}

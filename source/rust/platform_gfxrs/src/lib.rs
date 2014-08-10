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
	box DesktopPlatform as Box<Platform>
}

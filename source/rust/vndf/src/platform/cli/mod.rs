use platform::{
	Frame,
	Input,
	Platform,
};

use self::inputhandler::InputHandler;
use self::renderer::Renderer;


mod inputhandler;
mod renderer;


struct CliPlatform {
	input_handler: InputHandler,
	renderer     : Renderer,
}

impl Platform for CliPlatform {
	fn input(&mut self) -> Result<Input, String> {
		Ok(self.input_handler.input())
	}

	fn render(&mut self, frame: &Frame) {
		self.renderer.render(frame)
	}

	fn get_error(&mut self) -> Result<(), String> {
		Ok(())
	}
}


pub fn init() -> Box<Platform> {
	box
		CliPlatform {
			input_handler: InputHandler::new(),
			renderer     : Renderer::new(),
		}
	as Box<Platform>
}

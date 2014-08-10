use std::rc::Rc;

use io;
use io::{
	Frame,
	Input,
	Platform,
};

use self::inputhandler::InputHandler;
use self::renderer::Renderer;


pub use self::font::Font;
pub use self::shaders::Shaders;
pub use self::textures::{Texture, Textures};
pub use self::window::Window;


mod font;
mod images;
mod inputhandler;
mod renderer;
mod shaders;
mod textures;
mod window;


struct DesktopPlatform {
	input_handler: InputHandler,
	renderer     : Renderer,
}

impl Platform for DesktopPlatform {
	fn input(&mut self) -> Input {
		self.input_handler.input()
	}

	fn render(&mut self, frame: &io::Frame) {
		self.renderer.render(frame)
	}
}


pub fn init() -> Box<Platform> {
	let screen_width  = 800;
	let screen_height = 600;

	let     window   = Rc::new(Window::create(screen_width, screen_height));
	let     shaders  = Shaders::new(&*window);
	let mut textures = Textures::init(&*window);
	let     font     = Font::load(&mut textures);

	images::load(&mut textures);

	box
		DesktopPlatform {
			input_handler: InputHandler::new(window.clone()),
			renderer     : Renderer::new(
				window.clone(),
				shaders,
				textures,
				font
			)
		}
	as Box<Platform>
}

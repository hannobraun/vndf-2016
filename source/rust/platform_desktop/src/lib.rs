extern crate libc;

extern crate freetype;
extern crate gl;
extern crate glfw;
extern crate stb_image;

extern crate physics;
extern crate platform;


use std::rc::Rc;

use platform::{
	Frame,
	Input,
	Platform,
};

use self::inputhandler::InputHandler;
use self::renderer::Renderer;


pub use self::font::{
	Font,
	Glyph,
};
pub use self::shaders::{
	Program,
	Shaders,
};
pub use self::textures::{
	Name,
	Texture,
	Textures
};
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
	fn input(&mut self) -> Result<Input, String> {
		Ok(self.input_handler.input())
	}

	fn render(&mut self, frame: &Frame) {
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

#![feature(phase)]


extern crate cgmath;
extern crate freetype;
extern crate gfx;
#[phase(plugin)] extern crate gfx_macros;
extern crate glfw;
extern crate stb_image;

extern crate game;
extern crate platform;


use std::rc::Rc;

use inputhandler::InputHandler;
use platform::{
	Frame,
	Input,
	Platform,
};
use render::renderer::Renderer;
use window::Window;


mod font;
mod images;
mod inputhandler;
mod render;
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
		self.renderer.render(frame);
	}
}


pub fn init() -> Box<Platform + 'static> {
	let window = Rc::new(Window::create(800, 600));
	let font   = font::load();
	let images = images::load();

	let input_handler = InputHandler::new(window.clone());
	let renderer      = Renderer::new(window.clone(), images, font);

	box
		DesktopPlatform {
			input_handler: input_handler,
			renderer     : renderer,
		}
	as Box<Platform>
}

#![feature(phase)]


extern crate sync;

extern crate gfx;
#[phase(plugin)] extern crate gfx_macros;
extern crate glfw;
extern crate stb_image;

extern crate physics;
extern crate platform;


use std::rc::Rc;

use gfx::{
	Device,
	DeviceHelper,
};
use glfw::Context;

use inputhandler::InputHandler;
use platform::{
	Frame,
	Input,
	Platform,
};
use renderer::Renderer;
use window::Window;


mod images;
mod inputhandler;
mod renderer;
mod window;


struct DesktopPlatform {
	input_handler: InputHandler,
	renderer     : Renderer,
	window       : Rc<Window>,
}

impl Platform for DesktopPlatform {
	fn input(&mut self) -> Result<Input, String> {
		Ok(self.input_handler.input())
	}

	fn render(&mut self, frame: &Frame) {
		self.renderer.render(frame);
	}
}


pub fn init() -> Box<Platform> {
	let window = Rc::new(Window::create(800, 600));

	let input_handler = InputHandler::new(window.clone());
	let renderer      = Renderer::new(window.clone());

	box
		DesktopPlatform {
			window  : window,

			input_handler: input_handler,
			renderer     : renderer,
		}
	as Box<Platform>
}

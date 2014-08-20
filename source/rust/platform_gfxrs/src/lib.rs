#![feature(phase)]


extern crate sync;

extern crate gfx;
#[phase(plugin)] extern crate gfx_macros;
extern crate glfw;

extern crate platform;


use std::rc::Rc;

use gfx::{
	Device,
	DeviceHelper,
};
use glfw::Context;

use platform::{
	Frame,
	Input,
	Platform,
};
use renderer::Renderer;
use window::Window;


mod renderer;
mod window;


struct DesktopPlatform {
	renderer: Renderer,
	window  : Rc<Window>,
}

impl Platform for DesktopPlatform {
	fn input(&mut self) -> Result<Input, String> {
		self.window.poll_events();

		let mut input = Input::default();
		input.exit = self.window.should_close();

		Ok(input)
	}

	fn render(&mut self, frame: &Frame) {
		self.renderer.render();
	}
}


pub fn init() -> Box<Platform> {
	let window   = Rc::new(Window::create(800, 600));
	let renderer = Renderer::new(window.clone());

	box
		DesktopPlatform {
			window  : window,
			renderer: renderer,
		}
	as Box<Platform>
}

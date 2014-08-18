#![feature(unsafe_destructor)]


extern crate sync;

extern crate device;
extern crate gfx;
extern crate glfw;

extern crate platform;


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


struct DesktopPlatform {
	glfw  : glfw::Glfw,
	window: glfw::Window,
	events: sync::comm::Receiver<(f64,glfw::WindowEvent)>,
	device: device::gl::GlDevice,
}

impl Platform for DesktopPlatform {
	fn input(&mut self) -> Result<Input, String> {
		self.glfw.poll_events();

		let mut input = Input::default();
		input.exit = self.window.should_close();

		for (_, event) in glfw::flush_messages(&self.events) {
			match event {
				glfw::KeyEvent(glfw::KeyEscape, _, glfw::Press, _) =>
					input.exit = true,

				_ => {},
			}
		}

		Ok(input)
	}

	fn render(&mut self, frame: &Frame) {
		let (width, height) = self.window.get_framebuffer_size();

		let frame = gfx::Frame::new(width as u16, height as u16);
		let mut list = self.device.create_draw_list();

		list.clear(
			gfx::ClearData {
				color: Some(gfx::Color([0.3, 0.3, 0.3, 1.0])),
				depth: None,
				stencil: None,
			},
			&frame
		);

		self.device.submit(list.as_slice());
        self.window.swap_buffers();
	}
}

// This is a workaround for https://github.com/gfx-rs/gfx-rs/issues/204. It
// should be possible to remove this pretty soon.
#[unsafe_destructor]
impl Drop for DesktopPlatform {
	fn drop(&mut self) {}
}


pub fn init() -> Box<Platform> {
	let width  = 800;
	let height = 600;

	let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

	let (window, events) = glfw
		.create_window(
			width,
			height,
			"Von Neumann Defense Force *EARLY PROTOTYPE*",
			glfw::Windowed)
		.expect("failed to create window");

	window.make_current();
	window.set_key_polling(true);

	let mut device = gfx::GlDevice::new(|s| glfw.get_proc_address(s));

	box
		DesktopPlatform {
			glfw  : glfw,
			window: window,
			events: events,
			device: device,
		}
	as Box<Platform>
}

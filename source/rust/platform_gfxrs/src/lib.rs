#![feature(unsafe_destructor)]


extern crate sync;

extern crate device;
extern crate gfx;
extern crate glfw;
extern crate glfw_platform;
extern crate render;

extern crate platform;


use glfw_platform::BuilderExtension;

use platform::{
	Frame,
	Input,
	Platform,
};


struct DesktopPlatform {
	glfw  : glfw::Glfw,
	window: glfw::Window,
	events: sync::comm::Receiver<(f64,glfw::WindowEvent)>,
	device: device::Device<render::resource::handle::Handle,device::gl::GlBackEnd,glfw_platform::Platform<glfw::RenderContext>>,
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
		self.device.update();
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

	let (mut window, events) = glfw_platform::WindowBuilder::new(&glfw)
		.title("Von Neumann Defense Force *EARLY PROTOTYPE*")
		.size(width, height)
		.try_modern_context_hints()
		.create()
		.expect("failed to create window");

	window.set_key_polling(true);

	let mut device = gfx::build()
		.with_glfw_window(&mut window)
		.with_queue_size(1)
		.spawn(proc(r) render(r, width as u16, height as u16))
		.unwrap();

	box
		DesktopPlatform {
			glfw  : glfw,
			window: window,
			events: events,
			device: device,
		}
	as Box<Platform>
}

fn render(mut renderer: gfx::Renderer, width: u16, height: u16) {
	let frame = gfx::Frame::new(width, height);

	let clear = gfx::ClearData {
		color: Some(gfx::Color([0.3, 0.3, 0.3, 1.0])),
		depth: None,
		stencil: None,
	};

	while !renderer.should_finish() {
		renderer.clear(clear, frame);
		renderer.end_frame();
		for err in renderer.errors() {
			println!("Renderer error: {}", err);
		}
	}
}

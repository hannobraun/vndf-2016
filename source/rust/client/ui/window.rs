use gl;
use glfw;
use glfw::Context;

use error::exit;


pub struct Window {
	pub width : u32,
	pub height: u32,

	glfw       : glfw::Glfw,
	glfw_window: glfw::Window
}

impl Window {
	pub fn create(width: u32, height: u32) -> Window {
		let glfw = match glfw::init(glfw::FAIL_ON_ERRORS) {
			Ok(glfw)   => glfw,
			Err(error) => exit(format!("{}", error))
		};

		glfw.window_hint(glfw::ContextVersion(3, 0));

		let (window, _) = glfw.create_window(
			width, height,
			"Von Neumann Defense Force",
			glfw::Windowed)
			.expect("failed to create window");

		glfw.make_context_current(Some(&window));
		gl::load_with(|proc_name| { glfw.get_proc_address(proc_name) });

		Window {
			width  : width,
			height: height,

			glfw       : glfw,
			glfw_window: window
		}
	}

	pub fn key_pressed(&self, key: glfw::Key) -> bool {
		self.glfw_window.get_key(key) == glfw::Press
	}

	pub fn should_close(&self) -> bool {
		self.glfw_window.should_close() ||
			self.glfw_window.get_key(glfw::KeyEscape) == glfw::Press
	}

	pub fn swap_buffers(&self) {
		self.glfw_window.swap_buffers()
	}

	pub fn poll_events(&self) {
		self.glfw.poll_events();
	}
}

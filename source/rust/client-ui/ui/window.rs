use gl;
use glfw;


pub struct Window {
	width : u32,
	height: u32,

	glfw       : glfw::Glfw,
	glfw_window: glfw::Window
}

impl Window {
	pub fn create(width: u32, height: u32) -> ~Window {
		let (glfw, errors) = glfw::init().unwrap();
		glfw::fail_on_error(&errors);

		let (window, _) = glfw.create_window(
			width, height,
			"Von Neumann Defense Force",
			glfw::Windowed)
			.expect("failed to create window");

		window.make_context_current();
		gl::load_with(|proc_name| { glfw.get_proc_address(proc_name) });

		~Window {
			width  : width,
			height: height,

			glfw       : glfw,
			glfw_window: window
		}
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

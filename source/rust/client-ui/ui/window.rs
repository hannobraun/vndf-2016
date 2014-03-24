use glfw;


pub struct Window {
	width : u32,
	height: u32,

	glfw_window: glfw::Window
}

impl Window {
	pub fn create(width: u32, height: u32) -> ~Window {
		match glfw::init() {
			Err(_) => fail!("failed to initialize GLFW"),
			_      => ()
		}

		let window_opt = glfw::Window::create(
			width, height,
			"Von Neumann Defense Force",
			glfw::Windowed);

		let window = match window_opt {
			Some(window) => window,
			None         => fail!("failed to create window")
		};

		window.make_context_current();

		~Window {
			width  : width,
			height: height,

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
}

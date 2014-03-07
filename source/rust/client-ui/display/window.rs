use glfw;


pub struct Window {
	glfw_window: glfw::Window
}

impl Window {
	pub fn create(width: u32, height: u32) -> glfw::Window {
		let window_opt = glfw::Window::create(
			width, height,
			"Von Neumann Defense Force",
			glfw::Windowed);

		let window = match window_opt {
			Some(window) => window,
			None         => fail!("Failed to create window.")
		};

		window.make_context_current();

		window
	}

	pub fn should_close(&self) -> bool {
		self.glfw_window.should_close() ||
			self.glfw_window.get_key(glfw::KeyEscape) == glfw::Press
	}
}

use glfw;


pub struct Window {
	glfw_window: glfw::Window
}

impl Window {
	pub fn should_close(&self) -> bool {
		self.glfw_window.should_close() ||
			self.glfw_window.get_key(glfw::KeyEscape) == glfw::Press
	}
}

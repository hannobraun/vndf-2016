use gfx;
use glfw;
use glfw::Context;


pub struct Window {
	pub width : u16,
	pub height: u16,

	glfw       : glfw::Glfw,
	glfw_window: glfw::Window
}

impl Window {
	pub fn create(width: u16, height: u16) -> Window {
		let glfw = match glfw::init(glfw::FAIL_ON_ERRORS) {
			Ok(glfw)   => glfw,
			Err(error) => fail!(format!("{}", error))
		};

		glfw.window_hint(glfw::ContextVersion(3, 2));
		glfw.window_hint(glfw::OpenglForwardCompat(true));
		glfw.window_hint(glfw::OpenglProfile(glfw::OpenGlCoreProfile));
		glfw.window_hint(glfw::Samples(16));

		let (window, _) =
			glfw.create_window(
				width as u32,
				height as u32,
				"Von Neumann Defense Force *EARLY PROTOTYPE*",
				glfw::Windowed
			)
			.expect("failed to create window");

		window.make_current();

		Window {
			width : width,
			height: height,

			glfw       : glfw,
			glfw_window: window
		}
	}

	pub fn new_device(&self) -> gfx::GlDevice {
		gfx::GlDevice::new(|s| self.glfw_window.get_proc_address(s))
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

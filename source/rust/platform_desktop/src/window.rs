use cgmath::Vector2;
use gfx;
use glfw::{
	mod,
	Action,
	Context,
	Key,
	OpenGlProfileHint,
	WindowHint,
	WindowMode,
};


pub struct Window {
	pub width : u16,
	pub height: u16,
	pub size  : Vector2<f32>,

	glfw       : glfw::Glfw,
	glfw_window: glfw::Window
}

impl Window {
	pub fn create(width: u16, height: u16) -> Window {
		let glfw = match glfw::init(glfw::FAIL_ON_ERRORS) {
			Ok(glfw)   => glfw,
			Err(error) => panic!(format!("{}", error))
		};

		glfw.window_hint(WindowHint::ContextVersion(3, 2));
		glfw.window_hint(WindowHint::OpenglForwardCompat(true));
		glfw.window_hint(WindowHint::OpenglProfile(OpenGlProfileHint::Core));
		glfw.window_hint(WindowHint::Samples(16));

		let (window, _) =
			glfw.create_window(
				width as u32,
				height as u32,
				"Von Neumann Defense Force *EARLY PROTOTYPE*",
				WindowMode::Windowed
			)
			.expect("failed to create window");

		window.make_current();

		Window {
			width : width,
			height: height,
			size  : Vector2::new(width as f32, height as f32),

			glfw       : glfw,
			glfw_window: window
		}
	}

	pub fn new_device(&self) -> gfx::GlDevice {
		gfx::GlDevice::new(|s| self.glfw_window.get_proc_address(s))
	}

	pub fn key_pressed(&self, key: Key) -> bool {
		self.glfw_window.get_key(key) == Action::Press
	}

	pub fn should_close(&self) -> bool {
		self.glfw_window.should_close() ||
			self.glfw_window.get_key(Key::Escape) == Action::Press
	}

	pub fn swap_buffers(&self) {
		self.glfw_window.swap_buffers()
	}

	pub fn poll_events(&self) {
		self.glfw.poll_events();
	}
}

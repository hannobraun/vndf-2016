use glutin;

use render::base::Graphics;


pub struct Window {
	inner: glutin::Window,

	width : u32,
	height: u32,
}

impl Window {
	pub fn new() -> Window {
		let width  = 800;
		let height = 600;

		let window = glutin::WindowBuilder::new()
			.with_title("Von Neumann Defense Force - PREVIEW VERSION".to_string())
			.with_dimensions(width, height)
			.with_vsync()
			.build_strict()
			.unwrap_or_else(|e| panic!("Error creating window: {}", e));

		unsafe { window.make_current() };

		Window {
			inner: window,

			width : width,
			height: height,
		}
	}

	pub fn width(&self) -> u32 {
		self.width
	}

	pub fn height(&self) -> u32 {
		self.height
	}

	pub fn create_graphics(&self) -> Graphics {
		Graphics::new(
			|s| self.inner.get_proc_address(s),
			(self.width as u16, self.height as u16),
		)
	}

	pub fn poll_events(&self) -> glutin::PollEventsIterator {
		self.inner.poll_events()
	}

	pub fn swap_buffers(&self) {
		self.inner.swap_buffers()
	}
}

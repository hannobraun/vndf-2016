use gfx_device_gl::GlDevice;
use glutin;


pub struct Window {
	inner: glutin::Window,

	width : u32,
	height: u32,
}

impl Window {
	pub fn new() -> Window {
		let window = glutin::WindowBuilder::new()
			.with_title("Von Neumann Defense Force - PREVIEW VERSION".to_string())
			.with_vsync()
			.build_strict()
			.unwrap_or_else(|e| panic!("Error creating window: {}", e));

		unsafe { window.make_current() };

		let (width, height) = match window.get_inner_size() {
			Some(size) => size,
			None       => panic!("Failed to determine window size"),
		};

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

	pub fn new_device(&self) -> GlDevice {
		GlDevice::new(|s| self.inner.get_proc_address(s))
	}

	pub fn is_closed(&self) -> bool {
		self.inner.is_closed()
	}

	pub fn poll_events(&self) -> glutin::PollEventsIterator {
		self.inner.poll_events()
	}

	pub fn swap_buffers(&self) {
		self.inner.swap_buffers()
	}
}

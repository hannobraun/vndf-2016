use gfx;
use gfx_device_gl::{
	GlDevice,
	GlResources,
};


pub struct Renderer {
	graphics: gfx::Graphics<GlDevice>,
	frame   : gfx::Frame<GlResources>,
}

impl Renderer {
	pub fn new(device: GlDevice, width: u32, height: u32) -> Renderer {
		let graphics = gfx::Graphics::new(device);
		let frame    = gfx::Frame::new(width as u16, height as u16);

		Renderer {
			graphics: graphics,
			frame   : frame,
		}
	}

	pub fn render(&mut self) {
		self.graphics.clear(
			gfx::ClearData {
				color  : [0.0, 0.0, 0.25, 1.0],
				depth  : 1.0,
				stencil: 0,
			},
			gfx::COLOR,
			&self.frame,
		);

		self.graphics.end_frame();
	}
}

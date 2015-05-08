use libc::c_void;

use gfx;
use gfx::traits::*;
use gfx_device_gl as gl;

use render::base::Batch;


pub struct Graphics {
	pub graphics: gfx::Graphics<gl::Device, gl::Factory>,
	pub output  : gl::Output,
}

impl Graphics {
	pub fn new<F>(get_proc_address: F, size: (u16, u16)) -> Graphics
		where F: FnMut(&str) -> *const c_void
	{
		let graphics = gl::create(get_proc_address).into_graphics();
		let output   = graphics.factory.make_fake_output(size.0, size.1);

		Graphics {
			graphics: graphics,
			output  : output,
		}
	}

	pub fn clear(&mut self) {
		self.graphics.clear(
			gfx::ClearData {
				color  : [0.0, 0.0, 0.25, 1.0],
				depth  : 1.0,
				stencil: 0,
			},
			gfx::COLOR,
			&self.output,
		);
	}

	pub fn draw<P>(
		&mut self,
		batch : &Batch<P>,
		params: &P,
	)
		where P: gfx::render::shade::ShaderParam<Resources=gl::Resources>,
	{
		self.graphics
			.draw_core(
				&batch.batch,
				&batch.slice,
				params,
				&self.output,
			)
			.unwrap_or_else(|e| panic!("Error drawing graphics: {:?}", e));
	}
}

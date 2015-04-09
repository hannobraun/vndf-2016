use libc::c_void;

use gfx;
use gfx::traits::*;
use gfx_device_gl as gl;


#[shader_param]
pub struct Params<R: gfx::Resources> {
	pub transform: [[f32; 4]; 4],

	pub width : f32,
	pub height: f32,

	pub color: gfx::shade::TextureParam<R>,
}


pub struct Graphics {
	pub graphics: gfx::Graphics<gl::Device, gl::Factory>,
	pub frame   : gfx::Frame<gl::Resources>,
}

impl Graphics {
	pub fn new<F>(get_proc_address: F, size: (u16, u16)) -> Graphics
		where F: FnMut(&str) -> *const c_void
	{
		Graphics {
			graphics: gl::create(get_proc_address).into_graphics(),
			frame   : gfx::Frame::new(size.0, size.1),
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
			&self.frame,
		);
	}

	pub fn draw<P>(
		&mut self,
		batch : &gfx::batch::CoreBatch<P>,
		slice : &gfx::Slice<gl::Resources>,
		params: &P,
	)
		where P: gfx::render::shade::ShaderParam<Resources=gl::Resources>,
	{
		self.graphics
			.draw_core(
				batch,
				slice,
				params,
				&self.frame,
			)
			.unwrap_or_else(|e| panic!("Error drawing graphics: {:?}", e));
	}
}

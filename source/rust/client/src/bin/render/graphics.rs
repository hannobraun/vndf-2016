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
}

impl Graphics {
	pub fn new<F>(get_proc_address: F) -> Graphics
		where F: FnMut(&str) -> *const c_void
	{
		let gfx_graphics = gl::create(get_proc_address)
			.into_graphics();

		Graphics {
			graphics: gfx_graphics,
		}
	}
}

use gfx;
use gfx_device_gl as gl;


pub struct Graphics {
	pub graphics: gfx::Graphics<gl::Device, gl::Factory>,
}

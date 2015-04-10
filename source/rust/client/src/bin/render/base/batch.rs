use gfx;
use gfx_device_gl as gl;


pub struct Batch<P: gfx::render::shade::ShaderParam> {
	pub batch: gfx::batch::CoreBatch<P>,
	pub slice: gfx::Slice<gl::Resources>,
}

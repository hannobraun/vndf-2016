use libc::c_void;

use gfx;
use gfx::traits::*;
use gfx_device_gl as gl;

use render::base::Batch;


pub struct Graphics {
	pub device : gl::Device,
	pub factory: gl::Factory,
	pub stream : gfx::OwnedStream<gl::Device, gl::Output>,
}

impl Graphics {
	pub fn new<F>(get_proc_address: F, size: (u16, u16)) -> Graphics
		where F: FnMut(&str) -> *const c_void
	{
		let (device, mut factory) = gl::create(get_proc_address);

		let output  = factory.make_fake_output(size.0, size.1);
		let stream  = factory.create_stream(output);

		Graphics {
			device : device,
			factory: factory,
			stream : stream,
		}
	}

	pub fn clear(&mut self) {
		self.stream.clear(
			gfx::ClearData {
				color  : [0.0, 0.0, 0.25, 1.0],
				depth  : 1.0,
				stencil: 0,
			}
		);
	}

	pub fn draw<P>(
		&mut self,
		batch : &Batch<P>,
		params: &P,
	)
		where P: gfx::render::shade::ShaderParam<Resources=gl::Resources>,
	{
		self.stream
			.draw(&batch.batch.with(
				&batch.slice,
				params,
				&gfx::DrawState::new().blend(gfx::BlendPreset::Alpha),
			))
			.unwrap_or_else(|e| panic!("Error drawing graphics: {:?}", e));
	}

	pub fn flush(&mut self) {
		self.stream.flush(&mut self.device);
	}
}

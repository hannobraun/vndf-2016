use std::marker::PhantomData;

use gfx::{
	self,
	DeviceExt,
	ToSlice,
};
use gfx_device_gl::{
	GlDevice,
	GlResources,
};
use nalgebra::{
	Eye,
	Mat4,
};


#[vertex_format]
#[derive(Copy)]
struct Vertex {
	pos: [f32; 2],
}


#[shader_param]
struct Params<R: gfx::Resources> {
	transform: [[f32; 4]; 4],

	_marker: PhantomData<R>,
}


static VERTEX_SRC: &'static [u8] = b"
	#version 120

	attribute vec2 pos;

	uniform mat4 transform;

	void main() {
		gl_Position = transform * vec4(pos, 0.0, 1.0);
	}
";

static FRAGMENT_SRC: &'static [u8] = b"
	#version 120

	void main() {
		gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0);
	}
";


pub struct Renderer {
	graphics: gfx::Graphics<GlDevice>,
	frame   : gfx::Frame<GlResources>,
	batch   : gfx::batch::RefBatch<Params<GlResources>>,
}

impl Renderer {
	pub fn new(mut device: GlDevice, width: u32, height: u32) -> Renderer {
		let program = device
			.link_program(VERTEX_SRC, FRAGMENT_SRC)
			.unwrap_or_else(|e| panic!("Error linking program: {:?}", e));

		let mesh = device.create_mesh(&[
			Vertex { pos: [ -0.5,  0.5 ] },
			Vertex { pos: [ -0.5, -0.5 ] },
			Vertex { pos: [  0.5,  0.5 ] },
			Vertex { pos: [  0.5, -0.5 ] },
		]);

		let mut graphics = gfx::Graphics::new(device);
		let     frame    = gfx::Frame::new(width as u16, height as u16);

		let slice = mesh.to_slice(gfx::PrimitiveType::TriangleStrip);

		let batch = graphics
			.make_batch(
				&program,
				&mesh,
				slice,
				&gfx::DrawState::new(),
			)
			.unwrap_or_else(|e| panic!("Error making batch: {:?}", e));

		Renderer {
			graphics: graphics,
			frame   : frame,
			batch   : batch,
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

		let transform: Mat4<f32> = Eye::new_identity(4);
		let params = Params {
			transform: *transform.as_array(),
			_marker  : PhantomData,
		};

		self.graphics
			.draw(&self.batch, &params, &self.frame)
			.unwrap_or_else(|e| panic!("Error drawing graphics: {:?}", e));

		self.graphics.end_frame();
	}
}

use std::marker::PhantomData;

use gfx;
use gfx::traits::*;
use gfx_device_gl as gl;

use nalgebra::Mat4;

use render::Graphics;


static VERTEX_SRC: &'static [u8] = b"
	#version 120

	attribute vec2 pos;

	uniform mat4 transform;

	uniform vec2 size;

	void main() {
		gl_Position = transform * vec4(pos * size, 0.0, 1.0);
	}
";

static FRAGMENT_SRC: &'static [u8] = b"
	#version 120

	void main() {
		gl_FragColor = vec4(0.0, 0.0, 1.0, 1.0);
	}
";


#[vertex_format]
#[derive(Clone, Copy)]
struct Vertex {
	pos: [f32; 2],
}


#[shader_param]
pub struct Params<R: gfx::Resources> {
	pub transform: [[f32; 4]; 4],
	pub size     : [f32; 2],
	pub _marker  : PhantomData<R>,
}



pub struct ShipDrawer {
	batch: gfx::batch::CoreBatch<Params<gl::Resources>>,
	slice: gfx::Slice<gl::Resources>,
}

impl ShipDrawer {
	pub fn new(graphics: &mut Graphics) -> ShipDrawer {
		let program = graphics.graphics.factory
			.link_program(VERTEX_SRC, FRAGMENT_SRC)
			.unwrap_or_else(|e| panic!("Error linking program: {:?}", e));

		let mesh = graphics.graphics.factory.create_mesh(&[
			Vertex { pos: [ -0.5, -0.5 ] },
			Vertex { pos: [  0.5, -0.5 ] },
			Vertex { pos: [  0.0,  0.5 ] },
		]);

		let batch = graphics.graphics
			.make_core(
				&program,
				&mesh,
				&gfx::DrawState::new().blend(gfx::BlendPreset::Alpha),
			)
			.unwrap_or_else(|e| panic!("Error making batch: {:?}", e));

		let slice = mesh.to_slice(gfx::PrimitiveType::TriangleStrip);

		ShipDrawer {
			batch: batch,
			slice: slice,
		}
	}

	pub fn draw(&mut self, transform: &Mat4<f32>, graphics: &mut Graphics) {
		let params = Params {
			transform: *transform.as_array(),

			size: [30.0, 30.0],

			_marker: PhantomData,
		};

		graphics.draw(
			&self.batch,
			&self.slice,
			&params,
		);
	}
}

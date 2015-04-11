use std::marker::PhantomData;

use gfx;
use gfx_device_gl as gl;

use nalgebra::Mat4;

use render::base::{
	Batch,
	Graphics,
};


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
	batch    : Batch<Params<gl::Resources>>,
	transform: Mat4<f32>,
}

impl ShipDrawer {
	pub fn new(graphics: &mut Graphics, transform: Mat4<f32>) -> ShipDrawer {
		let batch = Batch::new(
			graphics,
			VERTEX_SRC, FRAGMENT_SRC,
			&[
				Vertex { pos: [ -0.5, -0.5 ] },
				Vertex { pos: [  0.5, -0.5 ] },
				Vertex { pos: [  0.0,  0.5 ] },
			]
		);

		ShipDrawer {
			batch    : batch,
			transform: transform,
		}
	}

	pub fn draw(&mut self, graphics: &mut Graphics) {
		let params = Params {
			transform: *self.transform.as_array(),
			size     : [30.0, 30.0],
			_marker  : PhantomData,
		};

		graphics.draw(
			&self.batch,
			&params,
		);
	}
}

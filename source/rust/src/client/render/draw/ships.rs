use std::marker::PhantomData;

use gfx_device_gl as gl;

use nalgebra::{
	Iso3,
	Mat4,
	ToHomogeneous,
	Vec2,
	Vec3,
};

use client::render::base::{
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


gfx_vertex!(Vertex {
	pos@ pos: [f32; 2],
});


gfx_parameters!(Params {
	transform@ transform: [[f32; 4]; 4],
	size     @ size     : [f32; 2],
});


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

	pub fn draw(&mut self, graphics: &mut Graphics, ship: &Ship) {
		let translation = Iso3::new(
			Vec3::new(ship.x, ship.y, 0.0),
			Vec3::new(0.0, 0.0, 0.0),
		);
		let transform = self.transform * translation.to_homogeneous();

		let params = Params {
			transform: *transform.as_array(),
			size     : [30.0, 30.0],
			_r       : PhantomData,
		};

		graphics.draw(
			&self.batch,
			&params,
		);
	}
}


pub type Ship = Vec2<f32>;

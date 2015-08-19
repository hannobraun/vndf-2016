use std::marker::PhantomData;

use gfx_device_gl as gl;

use nalgebra::{
	Iso3,
	Mat4,
	ToHomogeneous,
	Vec3,
};

use client::render::base::{
	Batch,
	Graphics,
	color,
	Shape,
};


static VERTEX_SRC: &'static [u8] = b"
	#version 120

	attribute vec2 pos;

	uniform mat4 transform;
	uniform vec2 size;
	uniform vec3 color;

	varying vec3 v_color;

	void main() {
		gl_Position = transform * vec4(pos * size, 0.0, 1.0);
		v_color = color;
	}
";

static FRAGMENT_SRC: &'static [u8] = b"
	#version 120

	varying vec3 v_color;

	void main() {
		gl_FragColor = vec4(v_color, 1.0);
	}
";


gfx_vertex!(Vertex {
	pos@ pos: [f32; 2],
});


gfx_parameters!(Params {
	transform@ transform: [[f32; 4]; 4],
	size     @ size     : [f32; 2],
	color	 @ color    : [f32; 3],
});


pub struct ShapeDrawer {
	batch    : Batch<Params<gl::Resources>>,
}

impl ShapeDrawer {
	pub fn new(graphics: &mut Graphics, shape: &Shape) -> ShapeDrawer {
		let batch = Batch::new(
			graphics,
			VERTEX_SRC, FRAGMENT_SRC,
			&get_vertices(shape),
		);

		ShapeDrawer {
			batch    : batch,
		}
	}
	
	pub fn draw(
		&mut self,
		pos      : [f32;2],
		size     : [f32;2],
		color    : color::Color,
		transform: Mat4<f32>,
		graphics : &mut Graphics,
	) {
		let translation = Iso3::new(
			Vec3::new(pos[0], pos[1], 0.0),
			Vec3::new(0.0, 0.0, 0.0),
		);
		let transform = transform * translation.to_homogeneous();

		let params = Params {
			transform: *transform.as_array(),
			size     : size,
			color    : color,
			_r       : PhantomData,
		};

		graphics.draw(
			&self.batch,
			&params,
		);
	}
}

fn get_vertices(shape: &Shape) -> Vec<Vertex> {
	let mut verts: Vec<Vertex> = vec!();
	for n in shape.get_points() {
		verts.push(Vertex { pos: [n.0,n.1] });
	}
	verts
}

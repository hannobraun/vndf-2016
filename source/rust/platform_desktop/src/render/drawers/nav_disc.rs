use cgmath::FixedArray;
use gfx::{
	mod,
	DeviceHelper,
	DrawState,
	Frame,
	ToSlice,
};

use render::{
	shaders,
	Graphics,
	Transform,
	Vertex,
};

use super::Draw;


#[shader_param(Batch)]
struct Params {
	radius   : f32,
	transform: [[f32, ..4], ..4],
}


pub struct NavDiscDrawer {
	batch: Batch,
}

impl NavDiscDrawer {
	pub fn new(graphics: &mut Graphics, draw_state: &DrawState) -> NavDiscDrawer {
		let vertices = [
			Vertex::new([ -1.0, -1.0, 0.0 ], [ 0.0, 1.0 ]),
			Vertex::new([  1.0, -1.0, 0.0 ], [ 1.0, 1.0 ]),
			Vertex::new([ -1.0,  1.0, 0.0 ], [ 0.0, 0.0 ]),
			Vertex::new([  1.0,  1.0, 0.0 ], [ 1.0, 0.0 ]),
		];

		let mesh  = graphics.device.create_mesh(vertices);
		let slice = mesh.to_slice(gfx::TriangleStrip);

		let program = graphics.device
			.link_program(
				shaders::vertex::RINGS.clone(),
				shaders::fragment::RINGS.clone()
			)
			.unwrap_or_else(|error| panic!("error linking program: {}", error));

		let batch = graphics
			.make_batch(
				&program,
				&mesh,
				slice,
				draw_state,
			)
			.unwrap();

		NavDiscDrawer {
			batch: batch,
		}
	}

	pub fn draw(&self, graphics: &mut Graphics, frame: &Frame, nav_disc: &NavDisc) {
		graphics.draw(
			&self.batch,
			&nav_disc.to_params(),
			frame,
		);

		// It's possible to add more draw calls here to draw rotated rings for
		// additional planes. I tried it and the result was extreme visual
		// overload. I'll try to make do with the one ring for now.
	}
}


pub struct NavDisc {
	pub radius   : f32,
	pub transform: Transform,
}

impl Draw<Params> for NavDisc {
	fn to_params(&self) -> Params {
		Params {
			radius   : self.radius,
			transform: self.transform.into_fixed()
		}
	}
}

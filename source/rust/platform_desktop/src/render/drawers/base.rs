use cgmath::{
	FixedArray,
	Vector3,
};
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


#[shader_param(Batch)]
struct Params {
	center   : [f32, ..3],
	position : [f32, ..3],
	transform: [[f32, ..4], ..4],
}


pub struct BaseDrawer {
	pub batch: Batch,
}

impl BaseDrawer {
	pub fn new(graphics: &mut Graphics, draw_state: &DrawState) -> BaseDrawer {
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
				shaders::vertex::BASE.clone(),
				shaders::fragment::BASE.clone()
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

		BaseDrawer {
			batch: batch,
		}
	}

	pub fn draw(&self, graphics: &mut Graphics, frame: &Frame, base: &Base) {
		graphics.draw(
			&self.batch,
			&base.to_params(),
			frame
		);
	}
}


pub struct Base {
	pub center   : Vector3<f32>,
	pub position : Vector3<f32>,
	pub transform: Transform,
}

impl Base {
	fn to_params(&self) -> Params {
		Params {
			center   : self.center.into_fixed(),
			position : self.position.into_fixed(),
			transform: self.transform.into_fixed(),
		}
	}
}

use cgmath::{
	FixedArray,
	Vector3,
};
use gfx::{
	mod,
	DeviceHelper,
	Frame,
	ToSlice,
};

use super::{
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
	pub fn new(graphics: &mut Graphics, draw_state: &gfx::DrawState) -> BaseDrawer {
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
			.unwrap_or_else(|error| fail!("error linking program: {}", error));

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

	pub fn draw(
		&self,
		graphics : &mut Graphics,
		frame    : &Frame,
		center   : &Vector3<f32>,
		position : &Vector3<f32>,
		transform: &Transform,
	) {
		let params = Params {
			center   : center.into_fixed(),
			position : position.into_fixed(),
			transform: transform.into_fixed(),
		};

		graphics.draw(
			&self.batch,
			&params,
			frame
		);
	}
}

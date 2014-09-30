use cgmath::FixedArray;
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
	transform  : [[f32, ..4], ..4],
}


pub struct Rings {
	batch: Batch,
}

impl Rings {
	pub fn new(
		graphics  : &mut Graphics,
		draw_state: &gfx::DrawState,
	) -> Rings {
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
			.unwrap_or_else(|error| fail!("error linking program: {}", error));

		let batch = graphics
			.make_batch(
				&program,
				&mesh,
				slice,
				draw_state,
			)
			.unwrap();

		Rings {
			batch: batch,
		}
	}

	pub fn draw(
		&self,
		graphics : &mut Graphics,
		frame    : &Frame,
		transform: &Transform,
	) {
		let params = Params {
			transform: transform.into_fixed(),
		};

		graphics.draw(
			&self.batch,
			&params,
			frame
		);
	}
}

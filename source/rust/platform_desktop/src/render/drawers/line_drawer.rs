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

use render::{
	shaders,
	Drawer,
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


pub struct LineDrawer {
	pub batch: Batch,
}

impl Drawer<Line> for LineDrawer {
	fn new(graphics: &mut Graphics, draw_state: &gfx::DrawState) -> LineDrawer {
		let vertices = [
			Vertex::new([ 0.0, 0.0, 0.0 ], [ 0.0, 0.0 ]),
			Vertex::new([ 0.0, 0.0, 1.0 ], [ 0.0, 0.0 ]),
		];

		let mesh  = graphics.device.create_mesh(vertices);
		let slice = mesh.to_slice(gfx::Line);

		let program = graphics.device
			.link_program(
				shaders::vertex::LINE.clone(),
				shaders::fragment::SIMPLE.clone()
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

		LineDrawer {
			batch: batch,
		}
	}

	fn draw(
		&self,
		graphics: &mut Graphics,
		frame   : &Frame,
		line    : &Line,
	) {
		let params = Params {
			center   : line.center.into_fixed(),
			position : line.position.into_fixed(),
			transform: line.transform.into_fixed(),
		};

		graphics.draw(
			&self.batch,
			&params,
			frame
		);
	}
}


pub struct Line {
	pub center   : Vector3<f32>,
	pub position : Vector3<f32>,
	pub transform: Transform,
}

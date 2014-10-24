use cgmath::FixedArray;
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
	radius   : f32,
	transform: [[f32, ..4], ..4],
}


pub struct NavDiscDrawer {
	batch: Batch,
}

impl Drawer<NavDisc> for NavDiscDrawer {
	fn new(
		graphics  : &mut Graphics,
		draw_state: &gfx::DrawState,
	) -> NavDiscDrawer {
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

		NavDiscDrawer {
			batch: batch,
		}
	}

	fn draw(
		&self,
		graphics: &mut Graphics,
		frame   : &Frame,
		nav_disc: &NavDisc,
	) {
		graphics.draw(
			&self.batch,
			&Params {
				radius   : nav_disc.radius,
				transform: nav_disc.transform.into_fixed()
			},
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

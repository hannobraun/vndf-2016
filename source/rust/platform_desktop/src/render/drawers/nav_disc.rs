use cgmath::FixedArray;
use gfx::{
	mod,
	DrawState,
};

use render::{
	shaders,
	Graphics,
	Transform,
	Vertex,
};

use super::{
	Draw,
	Drawer,
};


#[shader_param(Batch)]
struct Params {
	radius   : f32,
	transform: [[f32, ..4], ..4],
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


pub type NavDiscDrawer = Drawer<_ParamsLink, Params>;

pub fn new_drawer(
	graphics: &mut Graphics,
	draw_state: &DrawState
) -> NavDiscDrawer {
	let vertices = [
		Vertex::new([ -1.0, -1.0, 0.0 ], [ 0.0, 1.0 ]),
		Vertex::new([  1.0, -1.0, 0.0 ], [ 1.0, 1.0 ]),
		Vertex::new([ -1.0,  1.0, 0.0 ], [ 0.0, 0.0 ]),
		Vertex::new([  1.0,  1.0, 0.0 ], [ 1.0, 0.0 ]),
	];

	Drawer::new(
		graphics,
		draw_state,
		vertices,
		gfx::TriangleStrip,
		shaders::vertex::NAV_DISC.clone(),
		shaders::fragment::NAV_DISC.clone(),
	)
}

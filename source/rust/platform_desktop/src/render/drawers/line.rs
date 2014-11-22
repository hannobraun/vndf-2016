use cgmath::{
	FixedArray,
	Vector3,
};
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
	center   : [f32, ..3],
	position : [f32, ..3],
	transform: [[f32, ..4], ..4],
}


pub struct Line {
	pub center   : Vector3<f32>,
	pub position : Vector3<f32>,
	pub transform: Transform,
}

impl Draw<Params> for Line {
	fn to_params(&self) -> Params {
		Params {
			center   : self.center.into_fixed(),
			position : self.position.into_fixed(),
			transform: self.transform.into_fixed(),
		}
	}
}


pub type LineDrawer = Drawer<_ParamsLink, Params>;

pub fn new_drawer(
	graphics: &mut Graphics,
	draw_state: &DrawState
) -> LineDrawer {
	let vertices = [
		Vertex::new([ 0.0, 0.0, 0.0 ], [ 0.0, 0.0 ]),
		Vertex::new([ 0.0, 0.0, 1.0 ], [ 0.0, 0.0 ]),
	];

	Drawer::new(
		graphics,
		draw_state,
		&vertices,
		gfx::Line,
		shaders::vertex::LINE.clone(),
		shaders::fragment::SIMPLE.clone(),
	)
}

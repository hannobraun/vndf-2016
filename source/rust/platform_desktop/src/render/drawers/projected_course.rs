use cgmath::{
	FixedArray,
	Vector,
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
	position : [f32, ..3],
	velocity : [f32, ..3],
	transform: [[f32, ..4], ..4],
}


pub struct ProjectedCourse {
	pub position : Vector3<f32>,
	pub velocity : Vector3<f32>,
	pub transform: Transform,
}

impl Draw<Params> for ProjectedCourse {
	fn to_params(&self) -> Params {
		let velocity = self.velocity
			.mul_s(self.transform[3][3])
			.mul_s(0.005);

		Params {
			position : self.position.into_fixed(),
			velocity : velocity.into_fixed(),
			transform: self.transform.into_fixed()
		}
	}
}


pub type ProjectedCourseDrawer = Drawer<_ParamsLink, Params>;

pub fn new_drawer(
	graphics: &mut Graphics,
	draw_state: &DrawState
) -> ProjectedCourseDrawer {
	let vertices = [
		Vertex::new([ 0.0, 0.0, 0.0 ], [ 0.0, 0.0 ]),
		Vertex::new([ 1.0, 0.0, 0.0 ], [ 0.0, 0.0 ]),
	];

	Drawer::new(
		graphics,
		draw_state,
		&vertices,
		gfx::LineStrip,
		shaders::vertex::PROJECTED_COURSE.clone(),
		shaders::fragment::SIMPLE.clone(),
	)
}

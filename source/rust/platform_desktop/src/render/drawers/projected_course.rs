use cgmath::{
	FixedArray,
	Vector,
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
	position : [f32, ..3],
	velocity : [f32, ..3],
	transform: [[f32, ..4], ..4],
}


pub struct ProjectedCourseDrawer {
	batch: Batch,
}

impl ProjectedCourseDrawer {
	pub fn new(
		graphics  : &mut Graphics,
		draw_state: &DrawState
	) -> ProjectedCourseDrawer {
		let vertices = [
			Vertex::new([ 0.0, 0.0, 0.0 ], [ 0.0, 0.0 ]),
			Vertex::new([ 1.0, 0.0, 0.0 ], [ 0.0, 0.0 ]),
		];

		let mesh  = graphics.device.create_mesh(vertices);
		let slice = mesh.to_slice(gfx::LineStrip);

		let program = graphics.device
			.link_program(
				shaders::vertex::PROJECTED_COURSE.clone(),
				shaders::fragment::SIMPLE.clone()
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

		ProjectedCourseDrawer {
			batch: batch,
		}
	}

	pub fn draw(
		&self,
		graphics        : &mut Graphics,
		frame           : &Frame,
		projected_course: &ProjectedCourse
	) {
		graphics.draw(
			&self.batch,
			&projected_course.to_params(),
			frame,
		);
	}
}


pub struct ProjectedCourse {
	pub position : Vector3<f32>,
	pub velocity : Vector3<f32>,
	pub transform: Transform,
}

impl ProjectedCourse {
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

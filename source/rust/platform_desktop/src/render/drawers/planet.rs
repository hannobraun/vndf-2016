use cgmath::{
	EuclideanVector,
	FixedArray,
	Vector3,
};
use gfx::{
	mod,
	DrawState,
};

use platform::Camera;

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
	position  : [f32, ..3],
	radius    : f32,
	base_color: [f32, ..3],
	projection: [[f32, ..4], ..4],
	transform : [[f32, ..4], ..4],

	distance_to_eye   : f32,
	camera_right_world: [f32, ..3],
	camera_up_world   : [f32, ..3],
}


pub struct Planet {
	pub position  : Vector3<f32>,
	pub radius    : f32,
	pub color     : Vector3<f32>,
	pub projection: Transform,
	pub camera    : Camera,
}

impl Draw<Params> for Planet {
	fn to_params(&self) -> Params {
		let view = self.camera.to_transform();

		let camera_right_world =
			Vector3::new(
				view[0][0],
				view[1][0],
				view[2][0],
			)
			.normalize();
		let camera_up_world =
			Vector3::new(
				view[0][1],
				view[1][1],
				view[2][1],
			)
			.normalize();

		let transform = self.projection.mul(&view);

		let eye = Vector3::new(
			self.camera.eye().x as f32,
			self.camera.eye().y as f32,
			self.camera.eye().z as f32,
		);

		Params {
			position  : self.position.into_fixed(),
			radius    : self.radius,
			base_color: self.color.into_fixed(),
			projection: self.projection.into_fixed(),
			transform : transform.into_fixed(),

			distance_to_eye   : (eye - self.position).length(),
			camera_right_world: camera_right_world.into_fixed(),
			camera_up_world   : camera_up_world.into_fixed(),
		}
	}
}


pub type PlanetDrawer = Drawer<_ParamsLink, Params>;

pub fn new_drawer(
	graphics: &mut Graphics,
	draw_state: &DrawState
) -> PlanetDrawer {
	let vertices = [
		Vertex::new([ -1.0, -1.0, 0.0 ], [ 0.0, 1.0 ]),
		Vertex::new([  1.0, -1.0, 0.0 ], [ 1.0, 1.0 ]),
		Vertex::new([ -1.0,  1.0, 0.0 ], [ 0.0, 0.0 ]),
		Vertex::new([  1.0,  1.0, 0.0 ], [ 1.0, 0.0 ]),
	];

	Drawer::new(
		graphics,
		draw_state,
		&vertices,
		gfx::TriangleStrip,
		shaders::vertex::SCALED_BILLBOARD.clone(),
		shaders::fragment::PLANET.clone(),
	)
}

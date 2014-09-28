use cgmath::{
	EuclideanVector,
	FixedArray,
	Vector3,
};
use gfx::{
	mod,
	DeviceHelper,
	Frame,
	ToSlice,
};

use platform::Camera;

use super::{
	shaders,
	Graphics,
	Transform,
	Vertex,
};


#[shader_param(Batch)]
struct Params {
	radius    : f32,
	projection: [[f32, ..4], ..4],
	transform : [[f32, ..4], ..4],

	distance_to_eye   : f32,
	camera_right_world: [f32, ..3],
	camera_up_world   : [f32, ..3],
}


pub struct Planet {
	batch : Batch,
	radius: f32,
}

impl Planet {
	pub fn new(
		graphics  : &mut Graphics,
		draw_state: &gfx::DrawState,
		radius    : f32,
	) -> Planet {
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
				shaders::vertex::PLANET.clone(),
				shaders::fragment::PLANET.clone()
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

		Planet {
			batch : batch,
			radius: radius,
		}
	}

	pub fn draw(
		&self,
		graphics  : &mut Graphics,
		frame     : &Frame,
		projection: Transform,
		camera    : &Camera,
	) {
		let view = camera.to_transform();

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

		let transform = projection.mul(&view);

		let params = Params {
			radius    : self.radius,
			projection: projection.into_fixed(),
			transform : transform.into_fixed(),

			distance_to_eye   : camera.eye().length() as f32,
			camera_right_world: camera_right_world.into_fixed(),
			camera_up_world   : camera_up_world.into_fixed(),
		};

		graphics.draw(
			&self.batch,
			&params,
			frame
		);
	}
}

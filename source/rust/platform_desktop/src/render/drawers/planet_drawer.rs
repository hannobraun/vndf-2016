use cgmath::{
	EuclideanVector,
	FixedArray,
	Vector3,
};
use gfx::{
	mod,
	DeviceHelper,
	DrawState,
	Frame,
	ToSlice,
};

use platform::Camera;

use render::{
	shaders,
	Graphics,
	Transform,
	Vertex,
};

use super::Drawer;


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


pub struct PlanetDrawer {
	batch : Batch,
}

impl Drawer<Planet> for PlanetDrawer {
	fn new(graphics: &mut Graphics, draw_state: &DrawState) -> PlanetDrawer {
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
				shaders::vertex::SCALED_BILLBOARD.clone(),
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

		PlanetDrawer {
			batch : batch,
		}
	}

	fn draw(&self, graphics: &mut Graphics, frame: &Frame, planet: &Planet) {
		let view = planet.camera.to_transform();

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

		let transform = planet.projection.mul(&view);

		let eye = Vector3::new(
			planet.camera.eye().x as f32,
			planet.camera.eye().y as f32,
			planet.camera.eye().z as f32,
		);

		let params = Params {
			position  : planet.position.into_fixed(),
			radius    : planet.radius,
			base_color: planet.color.into_fixed(),
			projection: planet.projection.into_fixed(),
			transform : transform.into_fixed(),

			distance_to_eye   : (eye - planet.position).length(),
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


pub struct Planet {
	pub position  : Vector3<f32>,
	pub radius    : f32,
	pub color     : Vector3<f32>,
	pub projection: Transform,
	pub camera    : Camera,
}

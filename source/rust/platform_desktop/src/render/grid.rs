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

use platform::Camera;

use super::{
	shaders,
	Graphics,
	Transform,
	Vertex,
};


#[shader_param(GridBatch)]
struct Params {
	transform: [[f32, ..4], ..4],
}


pub struct Grid {
	batch: GridBatch,
}

impl Grid {
	pub fn new(graphics: &mut Graphics, draw_state: &gfx::DrawState) -> Grid {
		let grid_data = [
			Vertex::new([ -9000.0, -9000.0, 0.0 ], [0.0, 0.0]),
			Vertex::new([ -9000.0,  9000.0, 0.0 ], [0.0, 0.0]),
			Vertex::new([ -7000.0, -9000.0, 0.0 ], [0.0, 0.0]),
			Vertex::new([ -7000.0,  9000.0, 0.0 ], [0.0, 0.0]),
			Vertex::new([ -5000.0, -9000.0, 0.0 ], [0.0, 0.0]),
			Vertex::new([ -5000.0,  9000.0, 0.0 ], [0.0, 0.0]),
			Vertex::new([ -3000.0, -9000.0, 0.0 ], [0.0, 0.0]),
			Vertex::new([ -3000.0,  9000.0, 0.0 ], [0.0, 0.0]),
			Vertex::new([ -1000.0, -9000.0, 0.0 ], [0.0, 0.0]),
			Vertex::new([ -1000.0,  9000.0, 0.0 ], [0.0, 0.0]),
			Vertex::new([  1000.0, -9000.0, 0.0 ], [0.0, 0.0]),
			Vertex::new([  1000.0,  9000.0, 0.0 ], [0.0, 0.0]),
			Vertex::new([  3000.0, -9000.0, 0.0 ], [0.0, 0.0]),
			Vertex::new([  3000.0,  9000.0, 0.0 ], [0.0, 0.0]),
			Vertex::new([  5000.0, -9000.0, 0.0 ], [0.0, 0.0]),
			Vertex::new([  5000.0,  9000.0, 0.0 ], [0.0, 0.0]),
			Vertex::new([  7000.0, -9000.0, 0.0 ], [0.0, 0.0]),
			Vertex::new([  7000.0,  9000.0, 0.0 ], [0.0, 0.0]),
			Vertex::new([  9000.0, -9000.0, 0.0 ], [0.0, 0.0]),
			Vertex::new([  9000.0,  9000.0, 0.0 ], [0.0, 0.0]),

			Vertex::new([ -9000.0, -9000.0, 0.0 ], [0.0, 0.0]),
			Vertex::new([  9000.0, -9000.0, 0.0 ], [0.0, 0.0]),
			Vertex::new([ -9000.0, -7000.0, 0.0 ], [0.0, 0.0]),
			Vertex::new([  9000.0, -7000.0, 0.0 ], [0.0, 0.0]),
			Vertex::new([ -9000.0, -5000.0, 0.0 ], [0.0, 0.0]),
			Vertex::new([  9000.0, -5000.0, 0.0 ], [0.0, 0.0]),
			Vertex::new([ -9000.0, -3000.0, 0.0 ], [0.0, 0.0]),
			Vertex::new([  9000.0, -3000.0, 0.0 ], [0.0, 0.0]),
			Vertex::new([ -9000.0, -1000.0, 0.0 ], [0.0, 0.0]),
			Vertex::new([  9000.0, -1000.0, 0.0 ], [0.0, 0.0]),
			Vertex::new([ -9000.0,  1000.0, 0.0 ], [0.0, 0.0]),
			Vertex::new([  9000.0,  1000.0, 0.0 ], [0.0, 0.0]),
			Vertex::new([ -9000.0,  3000.0, 0.0 ], [0.0, 0.0]),
			Vertex::new([  9000.0,  3000.0, 0.0 ], [0.0, 0.0]),
			Vertex::new([ -9000.0,  5000.0, 0.0 ], [0.0, 0.0]),
			Vertex::new([  9000.0,  5000.0, 0.0 ], [0.0, 0.0]),
			Vertex::new([ -9000.0,  7000.0, 0.0 ], [0.0, 0.0]),
			Vertex::new([  9000.0,  7000.0, 0.0 ], [0.0, 0.0]),
			Vertex::new([ -9000.0,  9000.0, 0.0 ], [0.0, 0.0]),
			Vertex::new([  9000.0,  9000.0, 0.0 ], [0.0, 0.0]),
		];

		let mesh  = graphics.device.create_mesh(grid_data);
		let slice = mesh.to_slice(gfx::Line);

		let program = graphics.device
			.link_program(
				shaders::vertex::SIMPLE.clone(),
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

		Grid {
			batch: batch,
		}
	}

	pub fn draw(
		&self,
		graphics  : &mut Graphics,
		frame     : &Frame,
		camera    : &Camera,
		projection: Transform,
	) {
		let grid_camera = Camera {
			center: Vector3::new(
				camera.center[0] % 2000.0,
				camera.center[1] % 2000.0,
				camera.center[2],
			),

			perspective: camera.perspective,
			distance   : camera.distance,
		};

		let params = Params {
			transform: projection.mul(&grid_camera.to_transform()).into_fixed(),
		};

		graphics.draw(
			&self.batch,
			&params,
			frame
		);
	}
}

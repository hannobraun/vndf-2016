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
	Graphics,
	Transform,
	Vertex,
};


static VERTEX_SHADER: gfx::ShaderSource = shaders! {
	GLSL_150: b"
		#version 150 core

		uniform mat4 transform;

		in vec3 position;

		void main() {
			gl_Position = transform * vec4(position, 1.0);
		}
	"
};

static FRAGMENT_SHADER: gfx::ShaderSource = shaders! {
	GLSL_150: b"
		#version 150 core

		out vec4 out_color;

		void main() {
			out_color = vec4(1.0, 1.0, 1.0, 1.0);
		}
	"
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
			Vertex::without_tex([ -9000.0, -9000.0, 0.0 ]),
			Vertex::without_tex([ -9000.0,  9000.0, 0.0 ]),
			Vertex::without_tex([ -7000.0, -9000.0, 0.0 ]),
			Vertex::without_tex([ -7000.0,  9000.0, 0.0 ]),
			Vertex::without_tex([ -5000.0, -9000.0, 0.0 ]),
			Vertex::without_tex([ -5000.0,  9000.0, 0.0 ]),
			Vertex::without_tex([ -3000.0, -9000.0, 0.0 ]),
			Vertex::without_tex([ -3000.0,  9000.0, 0.0 ]),
			Vertex::without_tex([ -1000.0, -9000.0, 0.0 ]),
			Vertex::without_tex([ -1000.0,  9000.0, 0.0 ]),
			Vertex::without_tex([  1000.0, -9000.0, 0.0 ]),
			Vertex::without_tex([  1000.0,  9000.0, 0.0 ]),
			Vertex::without_tex([  3000.0, -9000.0, 0.0 ]),
			Vertex::without_tex([  3000.0,  9000.0, 0.0 ]),
			Vertex::without_tex([  5000.0, -9000.0, 0.0 ]),
			Vertex::without_tex([  5000.0,  9000.0, 0.0 ]),
			Vertex::without_tex([  7000.0, -9000.0, 0.0 ]),
			Vertex::without_tex([  7000.0,  9000.0, 0.0 ]),
			Vertex::without_tex([  9000.0, -9000.0, 0.0 ]),
			Vertex::without_tex([  9000.0,  9000.0, 0.0 ]),

			Vertex::without_tex([ -9000.0, -9000.0, 0.0 ]),
			Vertex::without_tex([  9000.0, -9000.0, 0.0 ]),
			Vertex::without_tex([ -9000.0, -7000.0, 0.0 ]),
			Vertex::without_tex([  9000.0, -7000.0, 0.0 ]),
			Vertex::without_tex([ -9000.0, -5000.0, 0.0 ]),
			Vertex::without_tex([  9000.0, -5000.0, 0.0 ]),
			Vertex::without_tex([ -9000.0, -3000.0, 0.0 ]),
			Vertex::without_tex([  9000.0, -3000.0, 0.0 ]),
			Vertex::without_tex([ -9000.0, -1000.0, 0.0 ]),
			Vertex::without_tex([  9000.0, -1000.0, 0.0 ]),
			Vertex::without_tex([ -9000.0,  1000.0, 0.0 ]),
			Vertex::without_tex([  9000.0,  1000.0, 0.0 ]),
			Vertex::without_tex([ -9000.0,  3000.0, 0.0 ]),
			Vertex::without_tex([  9000.0,  3000.0, 0.0 ]),
			Vertex::without_tex([ -9000.0,  5000.0, 0.0 ]),
			Vertex::without_tex([  9000.0,  5000.0, 0.0 ]),
			Vertex::without_tex([ -9000.0,  7000.0, 0.0 ]),
			Vertex::without_tex([  9000.0,  7000.0, 0.0 ]),
			Vertex::without_tex([ -9000.0,  9000.0, 0.0 ]),
			Vertex::without_tex([  9000.0,  9000.0, 0.0 ]),
		];

		let mesh  = graphics.device.create_mesh(grid_data);
		let slice = mesh.to_slice(gfx::Line);

		let program = graphics.device
			.link_program(
				VERTEX_SHADER.clone(),
				FRAGMENT_SHADER.clone()
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

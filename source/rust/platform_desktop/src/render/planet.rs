use cgmath::{
	EuclideanVector,
	FixedArray,
	Vector2,
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

		uniform vec2 size;
		uniform mat4 transform;
		uniform vec3 camera_right_world;
		uniform vec3 camera_up_world;

		in vec3 position;

		void main() {
			vec3 position2 =
				vec3(0.0, 0.0, 0.0)
				+ camera_right_world * position.x * size.x
				+ camera_up_world * position.y * size.y;

			gl_Position = transform * vec4(position2, 1.0);
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


#[shader_param(Batch)]
struct Params {
	size     : [f32, ..2],
	transform: [[f32, ..4], ..4],

	camera_right_world: [f32, ..3],
	camera_up_world   : [f32, ..3],
}


pub struct Planet {
	batch : Batch,
	size  : Vector2<f32>,
}

impl Planet {
	pub fn new(graphics: &mut Graphics, width: f32, height: f32) -> Planet {
		let vertices = [
			Vertex::new([ -0.5, -0.5, 0.0 ], [ 0.0, 1.0 ]),
			Vertex::new([  0.5, -0.5, 0.0 ], [ 1.0, 1.0 ]),
			Vertex::new([ -0.5,  0.5, 0.0 ], [ 0.0, 0.0 ]),
			Vertex::new([  0.5,  0.5, 0.0 ], [ 1.0, 0.0 ]),
		];

		let mesh  = graphics.device.create_mesh(vertices);
		let slice = mesh.to_slice(gfx::TriangleStrip);

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
				&gfx::DrawState::new().blend(gfx::BlendAlpha)
			)
			.unwrap();

		Planet {
			batch : batch,
			size  : Vector2::new(width, height),
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
			size     : self.size.into_fixed(),
			transform: transform.into_fixed(),

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

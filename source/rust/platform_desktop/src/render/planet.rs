use cgmath::{
	FixedArray,
	Matrix4,
	Vector,
	Vector2,
};
use gfx::{
	mod,
	DeviceHelper,
	Frame,
	ToSlice,
};

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


#[shader_param(Batch)]
struct Params {
	transform: [[f32, ..4], ..4],
}


pub struct Planet {
	batch : Batch,
	offset: Vector2<f32>,
}

impl Planet {
	pub fn new(graphics: &mut Graphics, width: f32, height: f32) -> Planet {
		let vertices = [
			Vertex::new([   0.0,    0.0, 0.0 ], [ 0.0, 1.0 ]),
			Vertex::new([ width,    0.0, 0.0 ], [ 1.0, 1.0 ]),
			Vertex::new([   0.0, height, 0.0 ], [ 0.0, 0.0 ]),
			Vertex::new([ width, height, 0.0 ], [ 1.0, 0.0 ]),
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
			offset: Vector2::new(width, height).mul_s(-0.5),
		}
	}

	pub fn draw(
		&self,
		graphics : &mut Graphics,
		frame    : &Frame,
		transform: Transform,
	) {
		let params = Params {
			transform: transform.mul(&Matrix4::from_translation(&self.offset.extend(0.0))).into_fixed(),
		};

		graphics.draw(
			&self.batch,
			&params,
			frame
		);
	}
}

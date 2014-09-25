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
	Graphics,
	Transform,
	Vertex,
};


static VERTEX_SHADER: gfx::ShaderSource = shaders! {
	GLSL_150: b"
		#version 150 core

		uniform float radius;
		uniform mat4  transform;
		uniform vec3  camera_right_world;
		uniform vec3  camera_up_world;

		in vec3 position;

		out vec2 point;

		void main() {
			vec3 position2 =
				vec3(0.0, 0.0, 0.0)
				+ camera_right_world * position.x * radius
				+ camera_up_world * position.y * radius;

			gl_Position = transform * vec4(position2, 1.0);
			point = position.xy;
		}
	"
};

static FRAGMENT_SHADER: gfx::ShaderSource = shaders! {
	GLSL_150: b"
		#version 150 core

		uniform mat4  projection;
		uniform float distance_to_eye;
		uniform float radius;

		in vec2 point;

		out vec4 out_color;

		void main() {
			float atmosphere_height = 0.06;

			float r2 = 1.0;
			float r1 = r2 - atmosphere_height;

			float r = length(point);

			// Lighten the circle a bit towards the center to give the
			// perception of roundness and depth.
			vec3 base_color = vec3(0.0, 0.45, 0.0);
			vec3 color      = base_color * (1.0 - r * 0.5);

			// At the edges, the planet should not be completely solid, to give
			// the appearance of an atmosphere.
			float a = 1.0 - (1.0 / (r2 - r1) * (r - r1));

			// The final color is just composed from the two previous
			// computations.
			out_color = vec4(color, a);

			// If we're outside the circle, we're done. Otherwise, the following
			// will mess up the depth buffer.
			if (r > r2) {
				return;
			}

			// Since this is a billboard, we need a bit of math to set the depth
			// buffer value as if it were a sphere.
			float depth  = distance_to_eye - sqrt(r2*r2 - r*r) * radius;
			float A      = projection[2].z;
			float B      = projection[3].z;
			gl_FragDepth = 0.5*(-A*depth + B) / depth + 0.5;
		}
	"
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

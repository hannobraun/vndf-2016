use gfx::{
	mod,
	DeviceHelper,
	ToSlice,
};

use super::{
	Graphics,
	Vertex,
};


#[shader_param(GridBatch)]
pub struct GridParams {
	pub transform: [[f32, ..4], ..4],
}


static GRID_VERTEX_SHADER: gfx::ShaderSource = shaders! {
	GLSL_150: b"
		#version 150 core

		uniform mat4 transform;

		in vec3 position;

		void main() {
			gl_Position = transform * vec4(position, 1.0);
		}
	"
};

static GRID_FRAGMENT_SHADER: gfx::ShaderSource = shaders! {
	GLSL_150: b"
		#version 150 core

		out vec4 out_color;

		void main() {
			out_color = vec4(1.0, 1.0, 1.0, 1.0);
		}
	"
};


pub struct Grid {
	pub batch: GridBatch,
}

impl Grid {
	pub fn new(graphics: &mut Graphics) -> Grid {
		let grid_data = [
			Vertex::for_grid([ -700.0, -600.0, 0.0 ]),
			Vertex::for_grid([ -700.0,  600.0, 0.0 ]),
			Vertex::for_grid([ -500.0, -600.0, 0.0 ]),
			Vertex::for_grid([ -500.0,  600.0, 0.0 ]),
			Vertex::for_grid([ -300.0, -600.0, 0.0 ]),
			Vertex::for_grid([ -300.0,  600.0, 0.0 ]),
			Vertex::for_grid([ -100.0, -600.0, 0.0 ]),
			Vertex::for_grid([ -100.0,  600.0, 0.0 ]),
			Vertex::for_grid([  100.0, -600.0, 0.0 ]),
			Vertex::for_grid([  100.0,  600.0, 0.0 ]),
			Vertex::for_grid([  300.0, -600.0, 0.0 ]),
			Vertex::for_grid([  300.0,  600.0, 0.0 ]),
			Vertex::for_grid([  500.0, -600.0, 0.0 ]),
			Vertex::for_grid([  500.0,  600.0, 0.0 ]),
			Vertex::for_grid([  700.0, -600.0, 0.0 ]),
			Vertex::for_grid([  700.0,  600.0, 0.0 ]),

			Vertex::for_grid([ -700.0, -600.0, 0.0 ]),
			Vertex::for_grid([  700.0, -600.0, 0.0 ]),
			Vertex::for_grid([ -700.0, -400.0, 0.0 ]),
			Vertex::for_grid([  700.0, -400.0, 0.0 ]),
			Vertex::for_grid([ -700.0, -200.0, 0.0 ]),
			Vertex::for_grid([  700.0, -200.0, 0.0 ]),
			Vertex::for_grid([ -700.0,    0.0, 0.0 ]),
			Vertex::for_grid([  700.0,    0.0, 0.0 ]),
			Vertex::for_grid([ -700.0,  200.0, 0.0 ]),
			Vertex::for_grid([  700.0,  200.0, 0.0 ]),
			Vertex::for_grid([ -700.0,  400.0, 0.0 ]),
			Vertex::for_grid([  700.0,  400.0, 0.0 ]),
			Vertex::for_grid([ -700.0,  600.0, 0.0 ]),
			Vertex::for_grid([  700.0,  600.0, 0.0 ]),
		];

		let mesh  = graphics.device.create_mesh(grid_data);
		let slice = mesh.to_slice(gfx::Line);

		let program = graphics.device
			.link_program(
				GRID_VERTEX_SHADER.clone(),
				GRID_FRAGMENT_SHADER.clone()
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

		Grid {
			batch: batch,
		}
	}
}

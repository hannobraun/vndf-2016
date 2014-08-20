use std::rc::Rc;

use device::{
	mod,
	Line
};
use gfx::{
	mod,
	Device,
	DeviceHelper,
};
use render::mesh::{
	Mesh,
	VertexFormat,
};

use window::Window;


#[vertex_format]
struct Vertex {
	pos: [f32, ..2],
}

#[shader_param(Program)]
struct GridParams {
	screen_size: [f32, ..2],
	camera_pos : [f32, ..2],
}


static GRID_VERTEX_SHADER: gfx::ShaderSource = shaders! {
	GLSL_150: b"
		#version 150 core

		uniform vec2 screen_size;
		uniform vec2 camera_pos;

		in vec2 pos;

		void main() {
			mat4 m = mat4(
				2.0 / screen_size.x,                 0.0,  0.0 , 0.0,
				                0.0, 2.0 / screen_size.y,  0.0 , 0.0,
				                0.0,                 0.0, -0.01, 0.0,
				               -1.0,                -1.0,  0.0 , 1.0);

			vec2 camera_trans = screen_size * 0.5 - camera_pos;

			gl_Position = m * vec4(pos + camera_trans, 0.0, 1.0);
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


pub struct Renderer {
	device: device::gl::GlDevice,
	window: Rc<Window>,
}

impl Renderer {
	pub fn new(window: Rc<Window>) -> Renderer {
		Renderer {
			device: window.new_device(),
			window: window,
		}
	}

	pub fn render(&mut self) {
		let grid_data = vec![
			Vertex { pos: [ -700.0, -600.0 ] },
			Vertex { pos: [ -700.0,  600.0 ] },
			Vertex { pos: [ -500.0, -600.0 ] },
			Vertex { pos: [ -500.0,  600.0 ] },
			Vertex { pos: [ -300.0, -600.0 ] },
			Vertex { pos: [ -300.0,  600.0 ] },
			Vertex { pos: [ -100.0, -600.0 ] },
			Vertex { pos: [ -100.0,  600.0 ] },
			Vertex { pos: [  100.0, -600.0 ] },
			Vertex { pos: [  100.0,  600.0 ] },
			Vertex { pos: [  300.0, -600.0 ] },
			Vertex { pos: [  300.0,  600.0 ] },
			Vertex { pos: [  500.0, -600.0 ] },
			Vertex { pos: [  500.0,  600.0 ] },
			Vertex { pos: [  700.0, -600.0 ] },
			Vertex { pos: [  700.0,  600.0 ] },

			Vertex { pos: [ -700.0, -600.0 ] },
			Vertex { pos: [  700.0, -600.0 ] },
			Vertex { pos: [ -700.0, -400.0 ] },
			Vertex { pos: [  700.0, -400.0 ] },
			Vertex { pos: [ -700.0, -200.0 ] },
			Vertex { pos: [  700.0, -200.0 ] },
			Vertex { pos: [ -700.0,    0.0 ] },
			Vertex { pos: [  700.0,    0.0 ] },
			Vertex { pos: [ -700.0,  200.0 ] },
			Vertex { pos: [  700.0,  200.0 ] },
			Vertex { pos: [ -700.0,  400.0 ] },
			Vertex { pos: [  700.0,  400.0 ] },
			Vertex { pos: [ -700.0,  600.0 ] },
			Vertex { pos: [  700.0,  600.0 ] },
		];

		let mut renderer = self.device.create_renderer();

		let frame  = gfx::Frame::new(self.window.width, self.window.height);
		let state  = gfx::DrawState::new();
		let buffer = self.device.create_buffer_static(&grid_data);

		let mesh = Mesh {
			prim_type   : Line,
			num_vertices: grid_data.len() as u32,
			attributes  : VertexFormat::generate(None::<Vertex>, buffer.raw()),
		};

		let program: Program =
			self.device.link_program(
				GRID_VERTEX_SHADER.clone(),
				GRID_FRAGMENT_SHADER.clone()
			)
			.unwrap_or_else(|error| fail!("error linking program: {}", error));

		let params = GridParams {
			screen_size: [self.window.width as f32, self.window.height as f32],
			camera_pos : [0.0, 0.0],
		};

		renderer.clear(
			gfx::ClearData {
				color  : Some(gfx::Color([0.0, 0.0, 0.0, 1.0])),
				depth  : None,
				stencil: None,
			},
			&frame
		);

		renderer
			.draw(
				&mesh,
				mesh.get_slice(),
				&frame,
				(&program, &params),
				&state
			)
			.unwrap();

		self.device.submit(renderer.as_buffer());
        self.window.swap_buffers();
	}
}

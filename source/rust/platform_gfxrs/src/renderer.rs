use std::rc::Rc;

use device;
use gfx::{
	mod,
	Device,
	DeviceHelper,
};
use render::mesh::VertexFormat;

use window::Window;


#[vertex_format]
struct Vertex {
	pos: [f32, ..2],
}

#[shader_param(GridProgram)]
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
	device  : device::gl::GlDevice,
	renderer: gfx::Renderer,
	window  : Rc<Window>,

	frame: gfx::Frame,
	state: gfx::DrawState,
}

impl Renderer {
	pub fn new(window: Rc<Window>) -> Renderer {
		let mut device   = window.new_device();
		let     renderer = device.create_renderer();

		let frame = gfx::Frame::new(window.width, window.height);
		let state = gfx::DrawState::new();

		Renderer {
			device  : device,
			renderer: renderer,
			window  : window,

			frame: frame,
			state: state,
		}
	}

	pub fn render(&mut self) {
		let (grid_mesh, grid_program) = init_grid(&mut self.device);

		let params = GridParams {
			screen_size: [self.window.width as f32, self.window.height as f32],
			camera_pos : [0.0, 0.0],
		};

		self.renderer.clear(
			gfx::ClearData {
				color  : Some(gfx::Color([0.0, 0.0, 0.0, 1.0])),
				depth  : None,
				stencil: None,
			},
			&self.frame
		);

		self.renderer
			.draw(
				&grid_mesh,
				grid_mesh.get_slice(),
				&self.frame,
				(&grid_program, &params),
				&self.state
			)
			.unwrap();

		self.device.submit(self.renderer.as_buffer());
        self.window.swap_buffers();
	}
}


fn init_grid(device: &mut device::gl::GlDevice) -> (gfx::Mesh, GridProgram) {
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

	let mesh = device.create_mesh(grid_data, gfx::Line);

	let program =
		device.link_program(
			GRID_VERTEX_SHADER.clone(),
			GRID_FRAGMENT_SHADER.clone()
		)
		.unwrap_or_else(|error| fail!("error linking program: {}", error));

	(mesh, program)
}

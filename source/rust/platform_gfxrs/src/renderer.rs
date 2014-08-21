use std::rc::Rc;

use gfx::{
	mod,
	Device,
	DeviceHelper,
};

use physics::Vec2;
use platform::Frame;
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
	device  : gfx::GlDevice,
	renderer: gfx::Renderer,
	window  : Rc<Window>,

	frame: gfx::Frame,
	state: gfx::DrawState,

	grid: Grid,
}

impl Renderer {
	pub fn new(window: Rc<Window>) -> Renderer {
		let mut device   = window.new_device();
		let     renderer = device.create_renderer();

		let frame = gfx::Frame::new(window.width, window.height);
		let state = gfx::DrawState::new();

		let grid = Grid::new(&mut device);

		Renderer {
			device  : device,
			renderer: renderer,
			window  : window,

			frame: frame,
			state: state,

			grid: grid,
		}
	}

	pub fn render(&mut self, frame: &Frame) {
		self.renderer.clear(
			gfx::ClearData {
				color  : Some(gfx::Color([0.0, 0.0, 0.0, 1.0])),
				depth  : None,
				stencil: None,
			},
			&self.frame
		);

		self.draw_grid(&frame.camera);

		self.device.submit(self.renderer.as_buffer());
		self.window.swap_buffers();
	}

	fn draw_grid(&mut self, &Vec2(camera_x, camera_y): &Vec2) {
		let params = GridParams {
			screen_size: [self.window.width as f32, self.window.height as f32],
			camera_pos : [camera_x as f32, camera_y as f32],
		};

		self.renderer
			.draw(
				&self.grid.mesh,
				self.grid.mesh.get_slice(),
				&self.frame,
				(&self.grid.program, &params),
				&self.state
			)
			.unwrap();
	}
}


struct Grid {
	mesh   : gfx::Mesh,
	program: GridProgram,
}

impl Grid {
	fn new(device: &mut gfx::GlDevice) -> Grid {
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

		Grid {
			mesh   : mesh,
			program: program,
		}
	}
}

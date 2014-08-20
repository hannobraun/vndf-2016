use std::rc::Rc;

use device;
use gfx::{
	mod,
	Device,
	DeviceHelper,
};

use window::Window;


#[vertex_format]
struct Vertex {
	pos  : [f32, ..2],
}


static VERTEX_SRC: gfx::ShaderSource = shaders! {
GLSL_150: b"
	#version 150 core
	in vec2 pos;
	void main() {
		gl_Position = vec4(pos, 0.0, 1.0);
	}
"
};

static FRAGMENT_SRC: gfx::ShaderSource = shaders! {
GLSL_150: b"
	#version 150 core
	out vec4 o_Color;
	void main() {
		o_Color = vec4(0.0, 0.0, 1.0, 1.0);
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
		let vertex_data = vec![
			Vertex { pos: [ -0.5, -0.5 ] },
			Vertex { pos: [  0.5, -0.5 ] },
			Vertex { pos: [  0.0,  0.5 ] }
		];

		let mut renderer = self.device.create_renderer();

		let frame = gfx::Frame::new(self.window.width, self.window.height);
		let state = gfx::DrawState::new();
		let mesh  = self.device.create_mesh(vertex_data);

		let program: gfx::shade::EmptyProgram = self.device.link_program(
			VERTEX_SRC.clone(), FRAGMENT_SRC.clone()).unwrap();

		renderer.clear(
			gfx::ClearData {
				color  : Some(gfx::Color([0.0, 0.0, 0.0, 1.0])),
				depth  : None,
				stencil: None,
			},
			&frame
		);

		renderer.draw(&mesh, mesh.get_slice(), &frame, &program, &state)
			.unwrap();

		self.device.submit(renderer.as_buffer());
        self.window.swap_buffers();
	}
}

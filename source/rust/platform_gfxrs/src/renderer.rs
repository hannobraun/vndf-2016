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
	pos: [f32, ..2],
	color: [f32, ..3],
}


static VERTEX_SRC: gfx::ShaderSource = shaders! {
GLSL_150: b"
	#version 150 core
	in vec2 pos;
	in vec3 color;
	out vec4 v_Color;
	void main() {
		v_Color = vec4(color, 1.0);
		gl_Position = vec4(pos, 0.0, 1.0);
	}
"
};

static FRAGMENT_SRC: gfx::ShaderSource = shaders! {
GLSL_150: b"
	#version 150 core
	in vec4 v_Color;
	out vec4 o_Color;
	void main() {
		o_Color = v_Color;
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
		let frame = gfx::Frame::new(
			self.window.width,
			self.window.height
		);
		let mut list = self.device.create_draw_list();

		let state = gfx::DrawState::new();
		let vertex_data = vec![
			Vertex { pos: [ -0.5, -0.5 ], color: [1.0, 0.0, 0.0] },
			Vertex { pos: [ 0.5, -0.5 ], color: [0.0, 1.0, 0.0]  },
			Vertex { pos: [ 0.0, 0.5 ], color: [0.0, 0.0, 1.0]  }
		];
		let mesh = self.device.create_mesh(vertex_data);
		let program: gfx::shade::EmptyProgram = self.device.link_program(
			VERTEX_SRC.clone(), FRAGMENT_SRC.clone()).unwrap();

		list.clear(
			gfx::ClearData {
				color: Some(gfx::Color([0.0, 0.0, 0.0, 1.0])),
				depth: None,
				stencil: None,
			},
			&frame
		);

		list.draw(&mesh, mesh.get_slice(), &frame, &program, &state)
			.unwrap();

		self.device.submit(list.as_slice());
        self.window.swap_buffers();
	}
}

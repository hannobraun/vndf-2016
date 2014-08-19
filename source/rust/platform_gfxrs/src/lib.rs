#![feature(phase)]


extern crate sync;

extern crate device;
extern crate gfx;
#[phase(plugin)] extern crate gfx_macros;
extern crate glfw;

extern crate platform;


use gfx::{
	Device,
	DeviceHelper,
};
use glfw::Context;

use platform::{
	Frame,
	Input,
	Platform,
};
use window::Window;


mod window;


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


#[vertex_format]
struct Vertex {
	pos: [f32, ..2],
	color: [f32, ..3],
}


struct DesktopPlatform {
	window: Window,
}

impl Platform for DesktopPlatform {
	fn input(&mut self) -> Result<Input, String> {
		self.window.poll_events();

		let mut input = Input::default();
		input.exit = self.window.should_close();

		Ok(input)
	}

	fn render(&mut self, frame: &Frame) {
		let frame = gfx::Frame::new(
			self.window.width as u16,
			self.window.height as u16
		);
		let mut list = self.window.device.create_draw_list();

		let state = gfx::DrawState::new();
		let vertex_data = vec![
			Vertex { pos: [ -0.5, -0.5 ], color: [1.0, 0.0, 0.0] },
			Vertex { pos: [ 0.5, -0.5 ], color: [0.0, 1.0, 0.0]  },
			Vertex { pos: [ 0.0, 0.5 ], color: [0.0, 0.0, 1.0]  }
		];
		let mesh = self.window.device.create_mesh(vertex_data);
		let program: gfx::shade::EmptyProgram = self.window.device.link_program(
			VERTEX_SRC.clone(), FRAGMENT_SRC.clone()).unwrap();

		list.clear(
			gfx::ClearData {
				color: Some(gfx::Color([0.3, 0.3, 0.3, 1.0])),
				depth: None,
				stencil: None,
			},
			&frame
		);

		list.draw(&mesh, mesh.get_slice(), &frame, &program, &state)
			.unwrap();

		self.window.device.submit(list.as_slice());
        self.window.swap_buffers();
	}
}


pub fn init() -> Box<Platform> {
	let window = Window::create(800, 600);

	box
		DesktopPlatform {
			window: window,
		}
	as Box<Platform>
}

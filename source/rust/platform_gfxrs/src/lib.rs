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
	glfw  : glfw::Glfw,
	window: glfw::Window,
	events: sync::comm::Receiver<(f64,glfw::WindowEvent)>,
	device: device::gl::GlDevice,
}

impl Platform for DesktopPlatform {
	fn input(&mut self) -> Result<Input, String> {
		self.glfw.poll_events();

		let mut input = Input::default();
		input.exit = self.window.should_close();

		for (_, event) in glfw::flush_messages(&self.events) {
			match event {
				glfw::KeyEvent(glfw::KeyEscape, _, glfw::Press, _) =>
					input.exit = true,

				_ => {},
			}
		}

		Ok(input)
	}

	fn render(&mut self, frame: &Frame) {
		let (width, height) = self.window.get_framebuffer_size();

		let frame = gfx::Frame::new(width as u16, height as u16);
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
				color: Some(gfx::Color([0.3, 0.3, 0.3, 1.0])),
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


pub fn init() -> Box<Platform> {
	let width  = 800;
	let height = 600;

	let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

	glfw.window_hint(glfw::ContextVersion(3, 2));
	glfw.window_hint(glfw::OpenglForwardCompat(true));
	glfw.window_hint(glfw::OpenglProfile(glfw::OpenGlCoreProfile));

	let (window, events) = glfw
		.create_window(
			width,
			height,
			"Von Neumann Defense Force *EARLY PROTOTYPE*",
			glfw::Windowed)
		.expect("failed to create window");

	window.make_current();
	window.set_key_polling(true);

	let device = gfx::GlDevice::new(|s| glfw.get_proc_address(s));

	box
		DesktopPlatform {
			glfw  : glfw,
			window: window,
			events: events,
			device: device,
		}
	as Box<Platform>
}

use std::rc::Rc;

use gl;

use common::io;
use common::io::Frame;
use common::physics::{
	Body,
	Radians,
	Vec2
};

use client::error::exit;
use client::ui::{
	Font,
	Shaders,
	Texture,
	Textures,
	Window
};
use client::ui::shaders;


pub struct Renderer {
	screen_width : f64,
	screen_height: f64,

	window  : Rc<Window>,
	shaders : Shaders,
	textures: Textures,
	font    : Font,

	camera : Vec2,
	program: shaders::Program
}

impl io::Renderer for Renderer {
	fn render(&mut self, frame: &Frame) {
		gl::Clear(gl::COLOR_BUFFER_BIT);

		self.camera = frame.camera;

		self.draw_grid();

		for &body in frame.ships.iter() {
			let &texture = self.textures.get("images/spaceship.png");
			self.draw_ship(body, texture);
		}

		for &body in frame.missiles.iter() {
			let &texture = self.textures.get("images/missile.png");
			self.draw_ship(body, texture);
		}

		self.draw_ui_overlay(frame.input.attitude);

		self.window.swap_buffers();

		match gl::GetError() {
			gl::NO_ERROR => (),

			error @ _ => {
				let error_as_str = match error {
					gl::INVALID_ENUM =>
						"GL_INVALID_ENUM",
					gl::INVALID_VALUE =>
						"GL_INVALID_VALUE",
					gl::INVALID_OPERATION =>
						"GL_INVALID_OPERATION",
					gl::OUT_OF_MEMORY =>
						"GL_OUT_OF_MEMORY",
					gl::STACK_UNDERFLOW =>
						"GL_STACK_UNDERFLOW",
					gl::STACK_OVERFLOW =>
						"GL_STACK_OVERFLOW",

					_ => "unknown"
				};

				exit(format!("OpenGL error: {} ({})", error_as_str, error).as_slice())
			}
		}
	}
}

impl Renderer {
	pub fn new(
		window  : Rc<Window>,
		shaders : Shaders,
		textures: Textures,
		font    : Font) -> Renderer {

		Renderer {
			screen_width : window.width as f64,
			screen_height: window.height as f64,

			window  : window,
			shaders : shaders,
			textures: textures,
			font    : font,

			camera : Vec2::zero(),
			program: 0
		}
	}

	fn draw_grid(&mut self) {
		let Vec2(mut cam_x, mut cam_y) = self.camera;

		cam_x = cam_x % 200.0;
		cam_y = cam_y % 200.0;

		let vertices = [
			-700.0f32, -600.0f32,
			-700.0f32,  600.0f32,
			-500.0f32, -600.0f32,
			-500.0f32,  600.0f32,
			-300.0f32, -600.0f32,
			-300.0f32,  600.0f32,
			-100.0f32, -600.0f32,
			-100.0f32,  600.0f32,
			 100.0f32, -600.0f32,
			 100.0f32,  600.0f32,
			 300.0f32, -600.0f32,
			 300.0f32,  600.0f32,
			 500.0f32, -600.0f32,
			 500.0f32,  600.0f32,
			 700.0f32, -600.0f32,
			 700.0f32,  600.0f32,

			-700.0f32, -600.0f32,
			 700.0f32, -600.0f32,
			-700.0f32, -400.0f32,
			 700.0f32, -400.0f32,
			-700.0f32, -200.0f32,
			 700.0f32, -200.0f32,
			-700.0f32,    0.0f32,
			 700.0f32,    0.0f32,
			-700.0f32,  200.0f32,
			 700.0f32,  200.0f32,
			-700.0f32,  400.0f32,
			 700.0f32,  400.0f32,
			-700.0f32,  600.0f32,
			 700.0f32,  600.0f32];

		self.program = self.shaders.program("grid");
		gl::UseProgram(self.program);

		let screen_pos = unsafe {
			gl::GetUniformLocation(
				self.program,
				"screen".to_c_str().unwrap())
		};
		gl::Uniform2f(
			screen_pos,
			self.screen_width as f32,
			self.screen_height as f32);

		let camera_pos = unsafe {
			gl::GetUniformLocation(
				self.program,
				"camera".to_c_str().unwrap())
		};
		gl::Uniform2f(
			camera_pos,
			cam_x as f32,
			cam_y as f32);

		gl::EnableClientState(gl::VERTEX_ARRAY);

		unsafe {
			gl::VertexPointer(
				2,
				gl::FLOAT,
				0,
				vertices.as_ptr() as *const gl::types::GLvoid);
		}

		gl::DrawArrays(gl::LINES, 0, 30);
		gl::DisableClientState(gl::VERTEX_ARRAY);

		gl::UseProgram(0);
	}

	fn draw_ship(&mut self, body: Body, texture: Texture) {
		let Vec2(cam_x, cam_y) = self.camera;

		self.program = self.shaders.program("ship-image");

		gl::UseProgram(self.program);
		let camera_pos = unsafe {
			gl::GetUniformLocation(
				self.program,
				"camera".to_c_str().unwrap())
		};
		gl::Uniform2f(camera_pos, cam_x as f32, cam_y as f32);

		let draw_position = body.position - texture.size * 0.5;
		self.draw_texture(draw_position, &texture);

		self.program = self.shaders.program("ship-text");

		gl::UseProgram(self.program);
		let camera_pos = unsafe {
			gl::GetUniformLocation(
				self.program,
				"camera".to_c_str().unwrap())
		};
		gl::Uniform2f(camera_pos, cam_x as f32, cam_y as f32);

		let mut text_position = draw_position + texture.size;
		let Vec2(body_x, body_y) = body.position;
		self.draw_text(
			text_position,
			format!("pos: {:i} / {:i}",
				body_x as int,
				body_y as int).as_slice());

		text_position = text_position - Vec2(0.0, 15.0);
		self.draw_text(
			text_position,
			format!("att: {:+04i}", body.attitude.degrees()).as_slice());
	}

	fn draw_ui_overlay(&mut self, attitude: Radians) {
		self.program = self.shaders.program("ui-overlay");

		self.draw_text(
			Vec2(20.0, 40.0),
			"Change course with the left and right cursor keys");
		self.draw_text(
			Vec2(20.0, 20.0),
			"Shoot missiles with Enter");

		self.draw_text(
			Vec2(self.screen_width - 50.0, 40.0),
			format!("{:+04i}", attitude.degrees()).as_slice());
	}

	fn draw_text(&self, mut position: Vec2, text: &str) {
		for c in text.chars() {
			let glyph   = self.font.get(c);
			let texture = self.textures.get(glyph.texture_id.as_slice());

			self.draw_texture(
				position + glyph.offset,
				texture);

			position = position + glyph.advance;
		}
	}

	fn draw_texture(&self, Vec2(x, y): Vec2, texture: &Texture) {
		let Vec2(width, height) = texture.size;

		let vertices = [
			width as f32, height as f32, 0.0,
			width as f32, 0.0          , 0.0,
			0.0         , height as f32, 0.0,
			0.0         , 0.0          , 0.0];
		let texture_coordinates = [
			1.0f32, 0.0f32,
			1.0f32, 1.0f32,
			0.0f32, 0.0f32,
			0.0f32, 1.0f32];

		gl::UseProgram(self.program);

		let screen_pos = unsafe {
			gl::GetUniformLocation(
				self.program,
				"screen".to_c_str().unwrap())
		};
		gl::Uniform2f(
			screen_pos,
			self.screen_width as f32,
			self.screen_height as f32);

		let position_pos = unsafe {
			gl::GetUniformLocation(
				self.program,
				"position".to_c_str().unwrap())
		};
		gl::Uniform2f(position_pos, x as f32, y as f32);

		let texture_pos = unsafe {
			gl::GetUniformLocation(
				self.program,
				"tex".to_c_str().unwrap())
		};
		gl::Uniform1i(texture_pos, 0);

		gl::ActiveTexture(gl::TEXTURE0);
		gl::BindTexture(
			gl::TEXTURE_2D,
			texture.name);

		gl::EnableClientState(gl::VERTEX_ARRAY);
		gl::EnableClientState(gl::TEXTURE_COORD_ARRAY);

		unsafe {
			gl::VertexPointer(
				3,
				gl::FLOAT,
				0,
				vertices.as_ptr() as *const gl::types::GLvoid);
			gl::TexCoordPointer(
				2,
				gl::FLOAT,
				0,
				texture_coordinates.as_ptr() as *const gl::types::GLvoid);
		}

		gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);

		gl::DisableClientState(gl::VERTEX_ARRAY);
		gl::DisableClientState(gl::TEXTURE_COORD_ARRAY);

		gl::UseProgram(0);
	}
}

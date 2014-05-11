use std::rc::Rc;

use gl;

use common::io;
use common::io::Frame;
use common::physics::{
	Body,
	Radians,
	Vec2
};

use error::exit;
use ui::{
	Font,
	Shaders,
	Texture,
	Textures,
	Window
};
use ui::shaders;


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

		for &body in frame.ships.iter() {
			self.draw_ship(body);
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

				exit(format!("OpenGL error: {} ({})", error_as_str, error))
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

			camera : Vec2(0.0, 0.0),
			program: 0
		}
	}

	fn draw_ship(&mut self, body: Body) {
		let texture = self.textures.get("images/spaceship.png");

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
		self.draw_texture(draw_position, texture);

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
				body_y as int));

		text_position = text_position - Vec2(0.0, 15.0);
		self.draw_text(
			text_position,
			format!("att: {:+04i}", body.attitude.degrees()));
	}

	fn draw_ui_overlay(&mut self, attitude: Radians) {
		self.program = self.shaders.program("ui-overlay");

		self.draw_text(
			Vec2(20.0, 40.0),
			"Set attitude with the left and right cursor keys");
		self.draw_text(
			Vec2(20.0, 20.0),
			"Start maneuver with Enter");

		self.draw_text(
			Vec2(self.screen_width - 50.0, 40.0),
			format!("{:+04i}", attitude.degrees()));
	}

	fn draw_text(&self, mut position: Vec2, text: &str) {
		for c in text.chars() {
			let glyph   = self.font.get(c);
			let texture = self.textures.get(glyph.texture_id);

			self.draw_texture(
				position + glyph.offset,
				texture);

			position = position + glyph.advance;
		}
	}

	fn draw_texture(&self, Vec2(x, y): Vec2, texture: &Texture) {
		let Vec2(width, height) = texture.size;

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

		gl::EnableClientState(gl::VERTEX_ARRAY);
		gl::EnableClientState(gl::TEXTURE_COORD_ARRAY);

		unsafe {
			gl::VertexPointer(
				3,
				gl::FLOAT,
				0,
				vertices.as_ptr() as *gl::types::GLvoid);
			gl::TexCoordPointer(
				2,
				gl::FLOAT,
				0,
				texture_coordinates.as_ptr() as *gl::types::GLvoid);
		}

		gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);

		gl::DisableClientState(gl::VERTEX_ARRAY);
		gl::DisableClientState(gl::TEXTURE_COORD_ARRAY);

		gl::UseProgram(0);
	}
}

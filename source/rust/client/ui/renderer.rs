use std::io::File;
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
use ui::{Font, Texture, Textures, Window};


pub struct Renderer {
	screen_width : f64,
	screen_height: f64,

	window  : Rc<Window>,
	textures: Textures,
	font    : Font
}

impl Renderer {
	pub fn new(window: Rc<Window>, textures: Textures, font: Font) -> Renderer {
		gl::LoadIdentity();
		gl::Ortho(
			0.0,
			window.width as f64,
			0.0,
			window.height as f64,
			-100.0,
			100.0);

		Renderer {
			screen_width : window.width as f64,
			screen_height: window.height as f64,

			window  : window,
			textures: textures,
			font    : font
		}
	}

	fn draw_ship(&self, body: Body) {
		let texture = self.textures.get(&"images/spaceship.png".to_owned());

		let draw_position = body.position - texture.size * 0.5;
		draw_texture(draw_position, texture);

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

	fn draw_ui_overlay(&self, attitude: Radians) {
		self.draw_text2(
			Vec2(20.0, 40.0),
			"Set attitude with the left and right cursor keys");
		self.draw_text2(
			Vec2(20.0, 20.0),
			"Start maneuver with Enter");

		self.draw_text2(
			Vec2(self.screen_width - 50.0, 40.0),
			format!("{:+04i}", attitude.degrees()));
	}

	fn draw_text(&self, mut position: Vec2, text: &str) {
		for c in text.chars() {
			let glyph   = self.font.get(c);
			let texture = self.textures.get(&glyph.texture_id);

			draw_texture(position + glyph.offset, texture);

			position = position + glyph.advance;
		}
	}

	fn draw_text2(&self, mut position: Vec2, text: &str) {
		for c in text.chars() {
			let glyph   = self.font.get(c);
			let texture = self.textures.get(&glyph.texture_id);

			draw_texture2(position + glyph.offset, texture);

			position = position + glyph.advance;
		}
	}
}

impl io::Renderer for Renderer {
	fn render(&self, frame: &Frame) {
		gl::Clear(gl::COLOR_BUFFER_BIT);
		gl::Color4f(1.0, 1.0, 1.0, 1.0);

		gl::PushMatrix();
		{
			let Vec2(camera_x, camera_y) = frame.camera;
			gl::Translatef(
				(self.screen_width / 2.0 - camera_x) as f32,
				(self.screen_height / 2.0 - camera_y) as f32,
				0.0);

			for &body in frame.ships.iter() {
				self.draw_ship(body);
			}
		}
		gl::PopMatrix();

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

fn draw_texture(Vec2(pos_x, pos_y): Vec2, texture: &Texture) {
	let Vec2(texture_width, texture_height) = texture.size;

	gl::BindTexture(
		gl::TEXTURE_2D,
		texture.name);

	gl::PushMatrix();
	{
		gl::Translatef(
			pos_x as f32,
			pos_y as f32,
			0.0);

		let vertices = [
			texture_width as f32, texture_height as f32, 0.0f32,
			texture_width as f32, 0.0f32               , 0.0f32,
			0.0f32              , texture_height as f32, 0.0f32,
			0.0f32              , 0.0f32               , 0.0f32];
		let texture_coordinates = [
			1.0f32, 0.0f32,
			1.0f32, 1.0f32,
			0.0f32, 0.0f32,
			0.0f32, 1.0f32];

		gl::EnableClientState(gl::VERTEX_ARRAY);
		gl::EnableClientState(gl::TEXTURE_COORD_ARRAY);
		gl::Enable(gl::TEXTURE_2D);

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
		gl::Disable(gl::TEXTURE_2D);
	}
	gl::PopMatrix();
}

fn draw_texture2(Vec2(pos_x, pos_y): Vec2, texture: &Texture) {
	let Vec2(texture_width, texture_height) = texture.size;

	let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
	unsafe {
		gl::ShaderSource(
			vertex_shader,
			1,
			&load_shader("glsl/ui-overlay.vert").to_c_str().unwrap(),
			::std::ptr::null());
	}
	gl::CompileShader(vertex_shader);

	let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
	unsafe {
		gl::ShaderSource(
			fragment_shader,
			1,
			&load_shader("glsl/ui-overlay.frag").to_c_str().unwrap(),
			::std::ptr::null());
	}
	gl::CompileShader(fragment_shader);

	let shader_program = gl::CreateProgram();
	gl::AttachShader(shader_program, vertex_shader);
	gl::AttachShader(shader_program, fragment_shader);

	gl::LinkProgram(shader_program);
	gl::UseProgram(shader_program);

	let position_pos = unsafe {
		gl::GetUniformLocation(
			shader_program,
			"position".to_c_str().unwrap())
	};
	gl::Uniform2f(position_pos, pos_x as f32, pos_y as f32);

	let texture_pos = unsafe {
		gl::GetUniformLocation(
			shader_program,
			"tex".to_c_str().unwrap())
	};
	gl::Uniform1i(texture_pos, 0);

	gl::ActiveTexture(gl::TEXTURE0);
	gl::BindTexture(
		gl::TEXTURE_2D,
		texture.name);

	let vertices = [
		texture_width as f32, texture_height as f32, 0.0f32,
		texture_width as f32, 0.0f32               , 0.0f32,
		0.0f32              , texture_height as f32, 0.0f32,
		0.0f32              , 0.0f32               , 0.0f32];
	let texture_coordinates = [
		1.0f32, 0.0f32,
		1.0f32, 1.0f32,
		0.0f32, 0.0f32,
		0.0f32, 1.0f32];

	gl::EnableClientState(gl::VERTEX_ARRAY);
	gl::EnableClientState(gl::TEXTURE_COORD_ARRAY);
	gl::Enable(gl::TEXTURE_2D);

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
	gl::Disable(gl::TEXTURE_2D);

	gl::UseProgram(0);
	gl::DeleteProgram(shader_program);
}

fn load_shader(path: &str) -> ~str {
	match File::open(&Path::new(path)).read_to_str() {
		Ok(string) => string,
		Err(error) => fail!("Error loading shader: {}", error)
	}
}

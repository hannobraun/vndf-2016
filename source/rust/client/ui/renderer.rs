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
		let texture = self.textures.get(&~"images/spaceship.png");

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
			let texture = self.textures.get(&glyph.texture_id);

			draw_texture(position + glyph.offset, texture);

			position = position + glyph.advance;
		}
	}
}

impl io::Renderer for Renderer {
	fn render(&self, frame: &Frame) {
		gl::Clear(gl::COLOR_BUFFER_BIT);
		gl::Color4d(1.0, 1.0, 1.0, 1.0);

		gl::PushMatrix();
		{
			let Vec2(camera_x, camera_y) = frame.camera;
			gl::Translated(
				self.screen_width / 2.0 - camera_x,
				self.screen_height / 2.0 - camera_y,
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
			error @ _    => exit(format!("OpenGL error ({})", error))
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
		gl::Translated(
			pos_x,
			pos_y,
			0.0);

		let vertices = [
			texture_width, texture_height, 0.0,
			texture_width, 0.0           , 0.0,
			0.0          , texture_height, 0.0,
			0.0          , 0.0           , 0.0];
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
				gl::DOUBLE,
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

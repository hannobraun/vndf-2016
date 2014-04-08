use gl;

use common::physics::{Body, Vec2};

use components::{Control, Visual};
use entities::Components;
use ui::{Font, Texture, Textures, Window};


pub struct Renderer {
	screen_width : f64,
	screen_height: f64,

	textures: Textures,
	font    : Font
}

impl Renderer {
	pub fn init(window: &Window, textures: Textures, font: Font) -> Renderer {
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

			textures: textures,
			font    : font
		}
	}

	pub fn render(&self,
		window  : &Window,
		camera  : Vec2,
		controls: &Components<Control>,
		bodies  : &Components<Body>,
		visuals : &Components<Visual>) {

		gl::Clear(gl::COLOR_BUFFER_BIT);
		gl::Color4d(1.0, 1.0, 1.0, 1.0);

		gl::PushMatrix();
		{
			gl::Translated(
				self.screen_width / 2.0 - camera.x,
				self.screen_height / 2.0 - camera.y,
				0.0);

			for (id, &body) in bodies.iter() {
				self.draw_ship(
					body,
					visuals.get(id));
			}
		}
		gl::PopMatrix();

		for (_, &control) in controls.iter() {
			self.draw_ui_overlay(control);
			break;
		}

		window.swap_buffers();

		match gl::GetError() {
			gl::NO_ERROR => (),
			error @ _    => fail!("OpenGL error ({})", error)
		}
	}

	fn draw_ship(&self, body: Body, visual: &Visual) {
		let texture = self.textures.get(&visual.texture);

		let draw_position = body.position - texture.size * 0.5;
		draw_texture(draw_position, texture);
	}

	fn draw_ui_overlay(&self, control: Control) {
		self.draw_text(
			Vec2 { x: 20.0, y: 40.0 },
			"Set attitude with the left and right cursor keys");
		self.draw_text(
			Vec2 { x: 20.0, y: 20.0 },
			"Start maneuver with Enter");

		self.draw_text(
			Vec2 { x: self.screen_width - 50.0, y: 40.0 },
			format!("{:+04i}", control.attitude.to_degrees() as i64));
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

fn draw_texture(position: Vec2, texture: &Texture) {
	gl::BindTexture(
		gl::TEXTURE_2D,
		texture.name);

	gl::PushMatrix();
	{
		gl::Translated(
			position.x,
			position.y,
			0.0);

		gl::Begin(gl::TRIANGLE_STRIP);
		{
			gl::TexCoord2d(
				1.0,
				0.0);
			gl::Vertex3d(
				texture.size.x,
				texture.size.y,
				0.0);

			gl::TexCoord2d(
				1.0,
				1.0);
			gl::Vertex3d(
				texture.size.x,
				0.0,
				0.0);

			gl::TexCoord2d(
				0.0,
				0.0);
			gl::Vertex3d(
				0.0,
				texture.size.y,
				0.0);

			gl::TexCoord2d(
				0.0,
				1.0);
			gl::Vertex3d(
				0.0,
				0.0,
				0.0);
		}
		gl::End();
	}
	gl::PopMatrix();
}

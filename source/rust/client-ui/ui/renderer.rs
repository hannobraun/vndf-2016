use std::str;

use gl;

use common::vec::Vec2;

use components::Visual;
use entities::Components;
use ui::{Texture, Textures, Window};


pub struct Renderer {
	screen_width : f64,
	screen_height: f64,

	textures: Textures
}

impl Renderer {
	pub fn init(window: &Window, textures: Textures) -> Renderer {
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

			textures: textures
		}
	}

	pub fn render(&self,
		window   : &Window,
		camera   : Vec2,
		positions: &Components<Vec2>,
		visuals  : &Components<Visual>) {

		gl::Clear(gl::COLOR_BUFFER_BIT);
		gl::Color4d(1.0, 1.0, 1.0, 1.0);

		gl::PushMatrix();
		{
			gl::Translated(
				self.screen_width / 2.0 - camera.x,
				self.screen_height / 2.0 - camera.y,
				0.0);

			for (id, &position) in positions.iter() {
				let texture = self.textures.get(&visuals.get(id).texture);
				draw_texture(position, texture);
			}
		}
		gl::PopMatrix();

		self.draw_ui_overlay();

		window.swap_buffers();

		match gl::GetError() {
			gl::NO_ERROR => (),
			error @ _    => fail!("OpenGL error ({})", error)
		}
	}

	fn draw_ui_overlay(&self) {
		self.draw_text(
			10.0,
			10.0,
			"Use cursor keys to control camera");
	}

	fn draw_text(&self, mut x: f64, y: f64, text: &str) {
		for c in text.chars() {
			if c != ' ' {
				draw_texture(
					Vec2 { x: x, y: y },
					self.textures.get(&str::from_char(c)));
			}
			x += 12.0;
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
			position.x - texture.width as f64 / 2.0,
			position.y - texture.height as f64 / 2.0,
			0.0);

		gl::Begin(gl::TRIANGLE_STRIP);
		{
			gl::TexCoord2d(
				1.0,
				0.0);
			gl::Vertex3d(
				texture.width as f64,
				texture.height as f64,
				0.0);

			gl::TexCoord2d(
				1.0,
				1.0);
			gl::Vertex3d(
				texture.width as f64,
				0.0,
				0.0);

			gl::TexCoord2d(
				0.0,
				0.0);
			gl::Vertex3d(
				0.0,
				texture.height as f64,
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

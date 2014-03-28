use std::f64;
use std::str;

use gl;

use common::vec::Vec3;

use camera::Camera;
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

		let z_near = 0.1;
		let fov_angle_y = 45.0;
		let half_height =
			f64::tan( fov_angle_y / 360.0 * f64::consts::PI ) * z_near;
		let half_width =
			half_height * window.width as f64 / window.height as f64;
		gl::Frustum(
			-half_width, half_width,
			-half_height, half_height,
			z_near, 2000.0);

		Renderer {
			screen_width : window.width as f64,
			screen_height: window.height as f64,

			textures: textures
		}
	}

	pub fn render(&self,
		window   : &Window,
		camera   : &Camera,
		positions: &Components<Vec3>,
		visuals  : &Components<Visual>) {

		gl::Clear(gl::COLOR_BUFFER_BIT);
		gl::Color4d(1.0, 1.0, 1.0, 1.0);

		gl::PushMatrix();
		{
			gl::Translated(0.0, 0.0, -500.0);
			gl::Rotated(camera.v, 1.0, 0.0, 0.0);
			gl::Rotated(camera.h, 0.0, 1.0, 0.0);
			gl::Translated(-camera.x, -camera.y, -camera.z);

			draw_grid();

			for (id, position) in positions.iter() {
				let texture = self.textures.get(&visuals.get(id).texture);
				draw_texture(position.x, position.y, position.z, texture);
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
		gl::PushMatrix();
		{
			gl::LoadIdentity();
			gl::Ortho(
				0.0,
				self.screen_width,
				0.0,
				self.screen_height,
				-1.0,
				1.0);

			self.draw_text(
				10.0,
				10.0,
				"Use cursor keys to control camera");
		}
		gl::PopMatrix();
	}

	fn draw_text(&self, mut x: f64, y: f64, text: &str) {
		for c in text.chars() {
			if c != ' ' {
				draw_texture(x, y, 0.0,
					self.textures.get(&str::from_char(c)));
			}
			x += 12.0;
		}
	}
}

fn draw_grid() {
	gl::Begin(gl::LINES);
	{
		for &z in [-500.0, 500.0].iter() {
			let mut x = -500.0;
			while x <= 500.0 {
				gl::Vertex3d(
					x,
					500.0,
					z);
				gl::Vertex3d(
					x,
					-500.0,
					z);

				x += 100.0;
			}
		}

		for &x in [-500.0, 500.0].iter() {
			let mut z = -500.0;
			while z <= 500.0 {
				gl::Vertex3d(
					x,
					500.0,
					z);
				gl::Vertex3d(
					x,
					-500.0,
					z);

				z += 100.0;
			}
		}

		for &x in [-500.0, 500.0].iter() {
			let mut y = -500.0;
			while y <= 500.0 {
				gl::Vertex3d(
					x,
					y,
					-500.0);
				gl::Vertex3d(
					x,
					y,
					500.0);

				y += 100.0;
			}
		}

		for &y in [-500.0, 500.0].iter() {
			let mut x = -500.0;
			while x <= 500.0 {
				gl::Vertex3d(
					x,
					y,
					-500.0);
				gl::Vertex3d(
					x,
					y,
					500.0);

				x += 100.0;
			}
		}

		for &y in [-500.0, 500.0].iter() {
			let mut z = -500.0;
			while z <= 500.0 {
				gl::Vertex3d(
					-500.0,
					y,
					z);
				gl::Vertex3d(
					500.0,
					y,
					z);

				z += 100.0;
			}
		}

		for &z in [-500.0, 500.0].iter() {
			let mut y = -500.0;
			while y <= 500.0 {
				gl::Vertex3d(
					-500.0,
					y,
					z);
				gl::Vertex3d(
					500.0,
					y,
					z);

				y += 100.0;
			}
		}
	}
	gl::End();
}

fn draw_texture(x: f64, y: f64, z: f64, texture: &Texture) {
	gl::BindTexture(
		gl::TEXTURE_2D,
		texture.name);

	gl::PushMatrix();
	{
		gl::Translated(
			x - texture.width as f64 / 2.0,
			y - texture.height as f64 / 2.0,
			z);

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

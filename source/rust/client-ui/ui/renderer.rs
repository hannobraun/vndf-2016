use collections::HashMap;
use std::f64;

use gl;
use glfw;

use common::vec::Vec2;

use camera::Camera;
use entities::Components;
use texture::Texture;
use ui::Window;
use visual::Visual;


pub struct Renderer {
	screen_width : f64,
	screen_height: f64
}

impl Renderer {
	pub fn init(window: &Window) -> ~Renderer {
		gl::load_with(glfw::get_proc_address);

		gl::Enable(gl::TEXTURE_2D);

		gl::Enable(gl::BLEND);
		gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

		gl::LoadIdentity();

		// I'm not a 100% sure what this does, but it has to do with using
		// textures that are not power of two. Before I added this call,
		// glTexture2D wouldn't work correctly on an 11x11 texture, causing
		// memory access errors and not displaying it correctly.
		gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);

		let z_near = 0.1;
		let fov_angle_y = 45.0;
		let half_height =
			f64::tan( fov_angle_y / 360.0 * f64::consts::PI ) * z_near;
		let half_width =
			half_height * window.width as f64 / window.height as f64;
		gl::Frustum(
			-half_width, half_width,
			-half_height, half_height,
			z_near, 1000.0);

		~Renderer {
			screen_width : window.width as f64,
			screen_height: window.height as f64
		}
	}

	pub fn render(&self,
		window   : &Window,
		camera   : &Camera,
		positions: &Components<Vec2>,
		visuals  : &Components<Visual>,
		textures : &HashMap<~str, Texture>) {

		gl::Clear(gl::COLOR_BUFFER_BIT);
		gl::Color4d(1.0, 1.0, 1.0, 1.0);

		gl::PushMatrix();
		{
			gl::Translated(0.0, 0.0, -500.0);
			gl::Rotated(camera.v, 1.0, 0.0, 0.0);
			gl::Rotated(camera.h, 0.0, 1.0, 0.0);

			for (id, position) in positions.iter() {
				let texture = textures.get(&visuals.get(id).texture);
				draw_texture(position.x, position.y, texture);
			}
		}
		gl::PopMatrix();
		window.swap_buffers();
	}
}

fn draw_texture(x: f64, y: f64, texture: &Texture) {
	gl::BindTexture(
		gl::TEXTURE_2D,
		texture.name);

	gl::PushMatrix();
	{
		gl::Translated(
			x - texture.width as f64 / 2.0,
			y - texture.height as f64 / 2.0,
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

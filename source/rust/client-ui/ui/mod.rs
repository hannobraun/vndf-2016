use collections::HashMap;
use std::iter::Iterator;

use gl;

use common::vec::Vec2;

use camera::Camera;
use entities::Components;
use texture::Texture;
use visual::Visual;

pub use ui::renderer::Renderer;
pub use ui::window::Window;

mod renderer;
mod window;


pub fn render(
	window   : &Window,
	camera   : &Camera,
	positions: &Components<Vec2>,
	visuals  : &Components<Visual>,
	textures : &HashMap<~str, Texture>) {

	gl::Clear(gl::COLOR_BUFFER_BIT);

	gl::PushMatrix();

	gl::Translated(0.0, 0.0, -500.0);
	gl::Rotated(camera.v, 1.0, 0.0, 0.0);
	gl::Rotated(camera.h, 0.0, 1.0, 0.0);

	gl::Color4d(1.0, 1.0, 1.0, 1.0);

	for (id, position) in positions.iter() {
		let texture = textures.get(&visuals.get(id).texture);
		draw_texture(position.x, position.y, texture);
	}

	gl::PopMatrix();
	window.swap_buffers();
}

fn draw_texture(x: f64, y: f64, texture: &Texture) {
	gl::BindTexture(
		gl::TEXTURE_2D,
		texture.name);

	gl::PushMatrix();

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

	gl::PopMatrix();
}

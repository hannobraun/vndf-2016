use std::f64;

use gl;
use glfw;


pub struct Renderer;

impl Renderer {
	pub fn init(screen_width: u32, screen_height: u32) -> ~Renderer {
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
			half_height * screen_width as f64 / screen_height as f64;
		gl::Frustum(
			-half_width, half_width,
			-half_height, half_height,
			z_near, 1000.0);

		~Renderer
	}
}

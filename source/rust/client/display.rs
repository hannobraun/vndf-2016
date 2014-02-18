use std::f64;
use std::hashmap::HashMap;
use std::iter::Iterator;

use gl;
use glfw;
use glfw::Window;

use common::vec::Vec2;

use camera::Camera;
use entities::Components;
use texture::Texture;
use visual::Visual;


pub fn init(screen_width: u32, screen_height: u32) -> Window {
	match glfw::init() {
		Err(_) => fail!("Failed to initialize GLFW."),
		_      => ()
	}

	let window = create_window(screen_width, screen_height);
	init_gl(screen_width, screen_height);

	window
}

fn create_window(width: u32, height: u32) -> Window {
	let window_opt = Window::create(
		width, height,
		"Von Neumann Defense Force",
		glfw::Windowed);

	let window = match window_opt {
		Some(window) => window,
		None         => fail!("Failed to create window.")
	};

	window.make_context_current();

	window
}

fn init_gl(screen_width: u32, screen_height: u32) {
	gl::load_with(glfw::get_proc_address);

	gl::Enable(gl::TEXTURE_2D);

	gl::Enable(gl::BLEND);
	gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

	gl::LoadIdentity();

	// I'm not a 100% sure what this does, but it has to do with using textures
	// that are not power of two. Before I added this call, glTexture2D wouldn't
	// work correctly on an 11x11 texture, causing memory access errors and not
	// displaying it correctly.
	gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);

	let z_near = 0.1;
	let fov_angle_y = 45.0;
	let half_height =
		f64::tan( fov_angle_y / 360.0 * f64::consts::PI ) * z_near;
	let half_width = half_height * screen_width as f64 / screen_height as f64;
	gl::Frustum(
		-half_width, half_width,
		-half_height, half_height,
		z_near, 1000.0);
}

pub fn render(
	window   : &Window,
	camera   : Camera,
	positions: &Components<Vec2>,
	visuals  : &Components<Visual>,
	textures : &HashMap<~str, Texture>) {

	gl::Clear(gl::COLOR_BUFFER_BIT);

	gl::PushMatrix();

	gl::Translated(0.0, 0.0, -500.0);
	gl::Rotatef(camera.v, 1.0f32, 0.0f32, 0.0f32);
	gl::Rotatef(camera.h, 0.0f32, 1.0f32, 0.0f32);

	gl::Color4d(1.0, 1.0, 1.0, 1.0);

	for (id, position) in positions.iter() {
		let texture = textures.get(&visuals.get(id).texture);
		gl::BindTexture(
			gl::TEXTURE_2D,
			texture.name);

		gl::PushMatrix();

		gl::Translated(
			position.x - texture.width as f64 / 2.0,
			position.y - texture.height as f64 / 2.0,
			0.0);

		gl::Begin(gl::TRIANGLE_STRIP);
			gl::TexCoord2d(1.0, 0.0);
			gl::Vertex3d(
				texture.width as f64,
				texture.height as f64,
				0.0);

			gl::TexCoord2d(1.0, 1.0);
			gl::Vertex3d(
				texture.width as f64,
				0.0,
				0.0);

			gl::TexCoord2d(0.0, 0.0);
			gl::Vertex3d(
				0.0,
				texture.height as f64,
				0.0);

			gl::TexCoord2f(0.0f32, 1.0f32);
			gl::Vertex3f(0.0f32, 0.0f32, 0.0f32);
		gl::End();

		gl::PopMatrix();
	}

	gl::PopMatrix();
	window.swap_buffers();
}

extern crate collections;

extern crate freetype;
extern crate gl;
extern crate glfw = "glfw-rs";
extern crate stb_image;

extern crate common;
extern crate ccore = "client-core";

use collections::HashMap;

use camera::Camera;
use core::Core;
use entities::Entities;


mod camera;
mod core;
mod display;
mod entities;
mod font;
mod images;
mod input;
mod texture;
mod visual;


#[link(name = "stb-image", kind = "static")]
extern {}


fn main() {
	let screen_width  = 800;
	let screen_height = 600;

	let mut core = Core::start();

	let window = display::init(screen_width, screen_height);
	let images = images::load();
	let font   = font::load();

	let mut textures = HashMap::new();
	for (id, &texture) in images.iter().chain(font.iter()) {
		textures.insert(id.clone(), texture);
	}

	let mut entities = Entities::new();

	entities.update_asteroid(999, 0.0, 0.0);

	let mut cam = Camera::new();

	while !window.should_close() &&
		window.get_key(glfw::KeyEscape) == glfw::Release {

		core.update_positions(entities);
		input::apply(
			&window,
			cam);
		display::render(
			&window,
			cam,
			&entities.positions,
			&entities.visuals,
			&textures);

		glfw::poll_events();
	}
}

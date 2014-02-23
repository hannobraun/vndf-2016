extern crate common;

extern crate freetype;
extern crate gl;
extern crate glfw;
extern crate stb_image;


use std::hashmap::HashMap;

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
mod net;
mod protocol;
mod texture;
mod visual;


#[link(name = "stb-image", kind = "static")]
extern {}


fn main() {
	let screen_width  = 800;
	let screen_height = 600;

	let core = Core::start();

	let window = display::init(screen_width, screen_height);
	let images = images::load();
	let font   = font::load();

	let mut textures = HashMap::new();
	for (id, &texture) in images.iter().chain(font.iter()) {
		textures.insert(id.clone(), texture);
	}

	let mut connection = protocol::init(core.socket_fd);

	let mut entities = Entities::new();

	entities.update_asteroid(999, 0.0, 0.0);

	let mut cam = Camera::new();

	while !window.should_close() &&
		window.get_key(glfw::KeyEscape) == glfw::Release {

		protocol::receive_positions(
			&mut connection,
			entities);
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

extern crate collections;
extern crate libc;

extern crate freetype;
extern crate gl;
extern crate glfw;
extern crate stb_image;

extern crate common;

use common::physics::Vec2;

use core::Core;
use entities::Entities;
use ui::{Font, Renderer, Textures, Window};


mod args;
mod components;
mod core;
mod entities;
mod images;
mod ui;


#[link(name = "stb-image", kind = "static")]
extern {}


fn main() {
	let screen_width  = 800;
	let screen_height = 600;

	let mut core = Core::start(args::get_server_address());

	let     window   = Window::create(screen_width, screen_height);
	let mut textures = Textures::init(window);
	images::load(&mut textures);
	let font     = Font::load(&mut textures);
	let renderer = Renderer::init(window, textures, font);

	let mut entities = Entities::new();

	entities.update_asteroid(999, Vec2 { x: 0.0, y: 0.0 });

	let mut cam = Vec2 { x: 0.0, y: 0.0 };

	let self_id = core.get_self_id();
	entities.self_id = Some(self_id);

	while !window.should_close() {
		core.update_positions(entities);

		let player_ship = entities.bodies.find(&self_id);

		match player_ship {
			Some(ship) => {
				cam = ship.position;
			},

			_ => ()
		}

		renderer.render(
			window,
			cam,
			&entities.bodies,
			&entities.visuals);

		window.poll_events();
	}
}

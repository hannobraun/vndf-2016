extern crate collections;
extern crate getopts;
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

	let args = match args::parse() {
		Some(args) => args,
		None       => fail!("Failed to parse arguments")
	};

	let mut core = Core::start(args.address, args.port);

	let     window   = Window::create(screen_width, screen_height);
	let mut textures = Textures::init(&window);
	images::load(&mut textures);
	let font     = Font::load(&mut textures);
	let renderer = Renderer::init(&window, textures, font);

	let mut entities = Entities::new();

	entities.update_asteroid(999, Vec2 { x: 0.0, y: 0.0 });

	let mut cam = Vec2 { x: 0.0, y: 0.0 };

	while !window.should_close() {
		core.update_ships(&mut entities);

		match entities.bodies.find(&entities.self_id.expect("self id")) {
			Some(ship) => {
				cam = ship.position;
			},

			_ => ()
		}

		ui::apply_input(
			&window,
			&mut entities.controls);

		for (_, control) in entities.controls.mut_iter() {
			if control.send {
				core.send_command(control.attitude);
				control.send = false;
			}
		}

		renderer.render(
			&window,
			cam,
			&entities.controls,
			&entities.bodies,
			&entities.visuals);

		window.poll_events();
	}
}

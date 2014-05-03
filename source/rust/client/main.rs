extern crate collections;
extern crate getopts;
extern crate libc;

extern crate freetype;
extern crate gl;
extern crate glfw;
extern crate stb_image;

extern crate common;


use std::rc::Rc;

use common::physics::Vec2;

use core::Network;
use entities::Entities;
use io::{
	Input,
	Renderer
};
use ui::{
	Font,
	Textures,
	Window
};


mod args;
mod components;
mod core;
mod entities;
mod error;
mod headless;
mod images;
mod io;
mod ui;


#[link(name = "stb-image", kind = "static")]
extern {}


fn main() {
	let screen_width  = 800;
	let screen_height = 600;

	let args = match args::parse() {
		Some(args) => args,
		None       => error::exit(format!("Failed to parse arguments"))
	};

	let mut core = Network::start(args.address, args.port);

	let (input, renderer) = if args.headless {
		(
			~headless::InputHandler::new() as ~Input,
			~headless::Renderer::new() as ~Renderer)
	}
	else {
		let     window   = Rc::new(Window::create(screen_width, screen_height));
		let mut textures = Textures::init(&*window);
		let     font     = Font::load(&mut textures);

		images::load(&mut textures);

		(
			~ui::Input::new(window.clone()) as ~Input,
			~ui::Renderer::new(window.clone(), textures, font) as ~Renderer)
	};

	let mut entities = Entities::new();

	let mut cam = Vec2::zero();

	let mut should_close = false;
	while !should_close {
		core.update_ships(&mut entities);

		match entities.self_id {
			Some(self_id) => match entities.bodies.find(&self_id) {
				Some(ship) => {
					cam = ship.position;
				},

				None => ()
			},

			None => ()
		}

		should_close = input.apply(&mut entities.controls);

		for (_, control) in entities.controls.mut_iter() {
			if control.send {
				core.send_command(control.attitude);
				control.send = false;
			}
		}

		renderer.render(
			cam,
			&entities.controls,
			&entities.bodies,
			&entities.visuals);
	}
}

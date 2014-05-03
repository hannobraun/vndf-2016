extern crate collections;
extern crate getopts;
extern crate libc;

extern crate freetype;
extern crate gl;
extern crate glfw;
extern crate stb_image;

extern crate common;


use std::rc::Rc;

use common::io::Frame;
use common::physics::Vec2;

use entities::Entities;
use io::{
	InputHandler,
	Renderer
};
use network::Network;
use ui::{
	Font,
	Textures,
	Window
};


mod args;
mod components;
mod entities;
mod error;
mod headless;
mod images;
mod io;
mod network;
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

	let mut network = Network::connect(args.address, args.port);

	let (input_handler, renderer) = if args.headless {
		(
			~headless::InputHandler::new() as ~InputHandler,
			~headless::Renderer::new() as ~Renderer)
	}
	else {
		let     window   = Rc::new(Window::create(screen_width, screen_height));
		let mut textures = Textures::init(&*window);
		let     font     = Font::load(&mut textures);

		images::load(&mut textures);

		(
			~ui::InputHandler::new(window.clone()) as ~InputHandler,
			~ui::Renderer::new(window.clone(), textures, font) as ~Renderer)
	};

	let mut entities = Entities::new();

	let mut cam = Vec2::zero();

	let mut should_close = false;
	while !should_close {
		network.update_ships(&mut entities);

		match entities.self_id {
			Some(self_id) => match entities.bodies.find(&self_id) {
				Some(ship) => {
					cam = ship.position;
				},

				None => ()
			},

			None => ()
		}

		let input = input_handler.apply(&mut entities.controls);
		should_close = input.exit;

		for (_, control) in entities.controls.mut_iter() {
			if control.send {
				network.send_command(control.attitude);
				control.send = false;
			}
		}

		let frame = Frame {
			camera: cam,
			ships : entities.bodies.values().map(|&x| x).collect()
		};

		renderer.render(
			&frame,
			&entities.controls);
	}
}

extern crate collections;
extern crate getopts;
extern crate libc;

extern crate freetype;
extern crate gl;
extern crate glfw;
extern crate stb_image;

extern crate common;


use std::rc::Rc;

use common::io::{
	Frame,
	InputHandler,
	Renderer
};
use common::physics::{
	Body,
	Vec2
};

use network::Network;
use ui::{
	Font,
	Textures,
	Window
};


mod args;
mod error;
mod headless;
mod images;
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

	let (mut input_handler, renderer) = if args.headless {
		(
			box headless::InputHandler::new() as Box<InputHandler>,
			box headless::Renderer::new() as Box<Renderer>)
	}
	else {
		let     window   = Rc::new(Window::create(screen_width, screen_height));
		let mut textures = Textures::init(&*window);
		let     font     = Font::load(&mut textures);

		images::load(&mut textures);

		(
			box ui::InputHandler::new(window.clone()) as Box<InputHandler>,
			box ui::Renderer::new(window.clone(), textures, font) as Box<Renderer>)
	};

	let mut camera = Vec2::zero();

	let mut ships: ~[Body] = ~[];

	let mut should_close = false;
	while !should_close {
		network.receive(|perception| {
			ships = perception.ships.iter().map(|ship| {
				if ship.id == perception.self_id {
					camera = ship.body.position;
				}

				ship.body
			}).collect();
		});

		let input = input_handler.input();
		should_close = input.exit;

		if input.send {
			network.send(input.attitude);
		}

		let frame = Frame {
			input : input,
			camera: camera,
			ships : ships.clone()
		};

		renderer.render(&frame);
	}
}

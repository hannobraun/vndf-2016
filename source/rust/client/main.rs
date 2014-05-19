extern crate collections;
extern crate getopts;
extern crate libc;
extern crate time;

extern crate freetype;
extern crate gl;
extern crate glfw;
extern crate stb_image;

extern crate common;


use collections::HashMap;
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
	Shaders,
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

	let mut network = Network::connect(
		args.address.as_slice(),
		args.port.as_slice());

	let (mut input_handler, mut renderer) = if args.headless {
		(
			box headless::InputHandler::new() as Box<InputHandler>,
			box headless::Renderer::new() as Box<Renderer>)
	}
	else {
		let     window   = Rc::new(Window::create(screen_width, screen_height));
		let     shaders  = Shaders::new(&*window);
		let mut textures = Textures::init(&*window);
		let     font     = Font::load(&mut textures);

		images::load(&mut textures);

		(
			box ui::InputHandler::new(window.clone()) as Box<InputHandler>,
			box ui::Renderer::new(
				window.clone(),
				shaders,
				textures,
				font) as Box<Renderer>)
	};

	let mut camera = Vec2::zero();

	let mut previous_time = time::precise_time_ns();
	let mut current_time  = time::precise_time_ns();

	let mut previous_ships = HashMap::new();
	let mut current_ships  = HashMap::new();

	let mut self_id = None;

	let mut should_close = false;
	while !should_close {
		let latest_self_id = receive_updates(
			&mut network,
			&mut previous_ships,
			&mut current_ships,
			&mut previous_time,
			&mut current_time);

		match latest_self_id {
			Some(id) => self_id = Some(id),
			None     => ()
		}

		let input = input_handler.input();
		should_close = input.exit;

		if input.send {
			network.send(input.attitude);
		}

		let i = {
			let diff = (current_time - previous_time) as f64;
			if diff <= 0.0 {
				0.0
			}
			else {
				(time::precise_time_ns() - current_time) as f64 / diff
			}
		};

		let mut ships = Vec::new();
		for (&ship_id, &current) in current_ships.iter() {
			match previous_ships.find(&ship_id) {
				Some(&previous) => {
					let mut body = current.clone();
					body.position = previous.position + (current.position - previous.position) * i;
					ships.push(body);

					match self_id {
						Some(id) => if id == ship_id {
							camera = body.position;
						},

						None => ()
					}
				},

				None => ()
			}
		}

		let frame = Frame {
			input : input,
			camera: camera,
			ships : ships
		};

		renderer.render(&frame);
	}
}

fn receive_updates(
	network       : &mut Network,
	previous_ships: &mut HashMap<uint, Body>,
	current_ships : &mut HashMap<uint, Body>,
	previous_time : &mut u64,
	current_time  : &mut u64) -> Option<uint> {

	let mut self_id = None;

	network.receive(|perception| {
		self_id = Some(perception.self_id);

		*previous_time = *current_time;
		*current_time  = time::precise_time_ns();

		previous_ships.clear();
		for (&id, &ship) in current_ships.iter() {
			previous_ships.insert(id, ship);
		}

		current_ships.clear();
		for ship in perception.ships.iter() {
			current_ships.insert(ship.id, ship.body);
		}
	});

	self_id
}

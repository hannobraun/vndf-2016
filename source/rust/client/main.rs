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

use common::io::{
	Frame,
	Input,
	InputHandler,
	Renderer
};
use common::physics::{
	Body,
	Vec2
};

use network::Network;


mod args;
mod error;
mod headless;
mod network;
mod ui;


#[link(name = "stb-image", kind = "static")]
extern {}


struct GameState {
	self_id: Option<uint>,

	previous_time: u64,
	current_time : u64
}


fn main() {
	let args = match args::parse() {
		Some(args) => args,
		None       => error::exit(format!("Failed to parse arguments"))
	};

	let mut network = Network::connect(
		args.address.as_slice(),
		args.port.as_slice());

	let (mut input_handler, mut renderer) = if args.headless {
		headless::init()
	}
	else {
		ui::init()
	};

	let mut game_state = GameState {
		self_id: None,

		previous_time: time::precise_time_ns(),
		current_time : time::precise_time_ns()
	};

	let mut camera = Vec2::zero();

	let mut previous_ships = HashMap::new();
	let mut current_ships  = HashMap::new();

	let mut missiles = HashMap::new();

	let mut next_input_send = 0;
	let mut input_to_send   = Input::default();

	let mut should_close = false;
	while !should_close {
		let self_id = receive_updates(
			&mut network,
			&mut previous_ships,
			&mut current_ships,
			&mut missiles,
			&mut game_state.previous_time,
			&mut game_state.current_time);

		match self_id {
			Some(id) => game_state.self_id = Some(id),
			None     => ()
		}

		let input = input_handler.input();
		should_close = input.exit;

		input_to_send.attitude = input.attitude;
		if time::precise_time_ns() >= next_input_send {
			network.send(input);
			next_input_send =
				time::precise_time_ns() + args.period as u64 * 1000 * 1000;
		}

		let i = {
			let diff = (game_state.current_time - game_state.previous_time) as f64;
			if diff <= 0.0 {
				0.0
			}
			else {
				(time::precise_time_ns() - game_state.current_time) as f64 / diff
			}
		};

		let mut ships = Vec::new();
		for (&ship_id, &current) in current_ships.iter() {
			match previous_ships.find(&ship_id) {
				Some(&previous) => {
					let mut body = current.clone();
					body.position = previous.position + (current.position - previous.position) * i;
					ships.push(body);

					match game_state.self_id {
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
			input   : input,
			camera  : camera,
			ships   : ships,
			missiles: missiles.iter().map(|(_, &body)| body).collect()
		};

		renderer.render(&frame);
	}
}

fn receive_updates(
	network       : &mut Network,
	previous_ships: &mut HashMap<uint, Body>,
	current_ships : &mut HashMap<uint, Body>,
	missiles      : &mut HashMap<uint, Body>,
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

		for missile in perception.missiles.iter() {
			missiles.insert(missile.id, missile.body);
		}
	});

	self_id
}

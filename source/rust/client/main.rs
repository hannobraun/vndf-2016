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

	previous_time : u64,
	current_time  : u64,
	previous_ships: HashMap<uint, Body>,
	current_ships : HashMap<uint, Body>,

	missiles: HashMap<uint, Body>
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

		previous_time : time::precise_time_ns(),
		current_time  : time::precise_time_ns(),
		previous_ships: HashMap::new(),
		current_ships : HashMap::new(),

		missiles: HashMap::new()
	};

	let mut camera = Vec2::zero();

	let mut next_input_send = 0;
	let mut input_to_send   = Input::default();

	let mut should_close = false;
	while !should_close {
		receive_updates(&mut network, &mut game_state);

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
		for (&ship_id, &current) in game_state.current_ships.iter() {
			match game_state.previous_ships.find(&ship_id) {
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
			missiles: game_state.missiles.iter().map(|(_, &body)| body).collect()
		};

		renderer.render(&frame);
	}
}

fn receive_updates(network: &mut Network, game_state: &mut GameState) {
	network.receive(|perception| {
		game_state.self_id = Some(perception.self_id);

		game_state.previous_time = game_state.current_time;
		game_state.current_time  = time::precise_time_ns();

		game_state.previous_ships.clear();
		for (&id, &ship) in game_state.current_ships.iter() {
			game_state.previous_ships.insert(id, ship);
		}

		game_state.current_ships.clear();
		for ship in perception.ships.iter() {
			game_state.current_ships.insert(ship.id, ship.body);
		}

		for missile in perception.missiles.iter() {
			game_state.missiles.insert(missile.id, missile.body);
		}
	});
}

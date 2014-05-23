extern crate collections;
extern crate getopts;
extern crate libc;
extern crate time;

extern crate freetype;
extern crate gl;
extern crate glfw;
extern crate stb_image;

extern crate common;


use common::io::{
	Frame,
	InputHandler,
	Renderer
};
use common::physics::Vec2;

use gamestate::GameState;
use inputsender::InputSender;
use network::Network;


mod args;
mod error;
mod gamestate;
mod headless;
mod inputsender;
mod network;
mod ui;


#[link(name = "stb-image", kind = "static")]
extern {}


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

	let mut game_state   = GameState::new();
	let mut input_sender = InputSender::new(args.period as u64);
	let mut camera       = Vec2::zero();

	let mut should_close = false;
	while !should_close {
		game_state.receive_updates(&mut network);

		let input = input_handler.input();
		should_close = input.exit;

		input_sender.update(input, &mut network);

		let ships = gamestate::interpolate_ships_and_camera(&mut game_state, &mut camera);

		let frame = Frame {
			input   : input,
			camera  : camera,
			ships   : ships,
			missiles: game_state.missiles.iter().map(|(_, &body)| body).collect()
		};

		renderer.render(&frame);
	}
}

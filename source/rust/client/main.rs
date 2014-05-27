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
	Input,
	InputHandler,
	Renderer
};
use common::physics::{
	Body,
	Vec2
};

use inputsender::InputSender;
use network::Network;


mod args;
mod error;
mod game;
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

	let mut game_state   = game::State::new();
	let mut input_sender = InputSender::new(args.period as u64);
	let mut camera       = Vec2::zero();

	let mut should_close = false;
	while !should_close {
		game_state.receive_updates(&mut network);
		game_state.update_camera(&mut camera);

		let input = input_handler.input();
		should_close = input.exit;

		input_sender.update(input, &mut network);

		let (ships, missiles) = game_state.interpolate();

		let frame = make_frame(input, ships, missiles, camera);

		renderer.render(&frame);
	}
}

fn make_frame(input: Input, ships: Vec<Body>, missiles: Vec<Body>, camera: Vec2) -> Frame {
	Frame {
		input   : input,
		camera  : camera,
		ships   : ships,
		missiles: missiles
	}
}

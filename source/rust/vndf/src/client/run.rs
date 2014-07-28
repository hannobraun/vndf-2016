use common::ecs::{
	ClientWorld,
	ShowAsMissile,
	ShowAsShip,
};
use common::io::{
	Frame,
	Input,
	InputHandler,
	Renderer
};

use client::args;
use client::error;
use client::game;
use client::headless;
use client::inputsender::InputSender;
use client::network::Network;
use client::ui;
use physics::Vec2;


pub fn run() {
	let args = match args::parse() {
		Some(args) => args,
		None       => error::exit(format!("Failed to parse arguments").as_slice())
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

		game_state.interpolate();

		let frame = make_frame(input, camera, &game_state.world);

		renderer.render(&frame);
	}
}

fn make_frame(input: Input, camera: Vec2, world: &ClientWorld) -> Frame {
	let ships = world.bodies
		.iter()
		.filter(|&(id, _)|
			world.visuals.get(id) == &ShowAsShip)
		.map(|(_, &body)|
			body)
		.collect();
	let missiles = world.bodies
		.iter()
		.filter(|&(id, _)|
			world.visuals.get(id) == &ShowAsMissile)
		.map(|(_, &body)|
			body)
		.collect();

	Frame {
		input   : input,
		camera  : camera,
		ships   : ships,
		missiles: missiles
	}
}

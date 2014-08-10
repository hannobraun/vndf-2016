use client::args;
use client::game;
use client::inputsender::InputSender;
use client::network::Network;
use game::ecs::{
	ClientWorld,
	ShowAsMissile,
	ShowAsShip,
};
use physics::Vec2;
use platform::{
	Frame,
	Input,
};
use platform_cli;
use platform_desktop;


pub fn run() {
	let args = match args::parse() {
		Some(args) => args,
		None       => fail!(format!("Failed to parse arguments"))
	};

	let mut network = Network::connect(
		args.address.as_slice(),
		args.port.as_slice());

	let mut platform = if args.headless {
		platform_cli::init()
	}
	else {
		platform_desktop::init()
	};

	let mut game_state   = game::State::new();
	let mut input_sender = InputSender::new(args.period as u64);
	let mut camera       = Vec2::zero();

	let mut should_close = false;
	while !should_close {
		game_state.receive_updates(&mut network);
		game_state.update_camera(&mut camera);

		let input = match platform.input() {
			Ok(input)  => input,
			Err(error) => fail!("Error reading input: {}", error)
		};
		should_close = input.exit;

		input_sender.update(input, &mut network);

		game_state.interpolate();

		let frame = make_frame(input, camera, &game_state.world);

		platform.render(&frame);
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

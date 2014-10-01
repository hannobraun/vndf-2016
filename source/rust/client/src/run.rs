use cgmath::Vector3;

use game::ecs::{
	Planet,
	ShowAsMissile,
	ShowAsShip,
};
use platform::{
	Camera,
	Frame,
	Input,
};
use platform_cli;
use platform_desktop;

use super::args;
use super::ecs::World;
use super::gamestate::GameState;
use super::inputsender::InputSender;
use super::network::Network;


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

	let mut game_state   = GameState::new();
	let mut input_sender = InputSender::new(args.period as u64);
	let mut camera       = Camera::new();

	let mut should_close = false;
	while !should_close {
		game_state.receive_updates(&mut network);
		game_state.update_camera(&mut camera);

		let input = match platform.input() {
			Ok(input)  => input,
			Err(error) => fail!("Error reading input: {}", error)
		};
		should_close = input.exit;
		camera.perspective = input.camera_angle;
		camera.distance = input.camera_distance;

		input_sender.update(input, &mut network);

		game_state.interpolate();

		let frame = make_frame(input, camera, &game_state.world);

		platform.render(&frame);
	}
}

fn make_frame(input: Input, camera: Camera, world: &World) -> Frame {
	let ships = world.bodies
		.iter()
		.filter(|&(&id, _)|
			world.visuals[id] == ShowAsShip)
		.map(|(_, &body)|
			body)
		.collect();
	let missiles = world.bodies
		.iter()
		.filter(|&(&id, _)|
			world.visuals[id] == ShowAsMissile)
		.map(|(_, &body)|
			body)
		.collect();

	Frame {
		input   : input,
		camera  : camera,
		ships   : ships,
		missiles: missiles,
		planets : vec![
			Planet {
				position: Vector3::zero(),
				radius  : 2576.0,
			},
			Planet {
				position: Vector3::new(0.0, 0.0, 5000.0),
				radius  : 2576.0,
			},
		],
	}
}

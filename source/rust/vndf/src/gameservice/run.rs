use std::os;

use super::args;
use super::events::Init;
use super::game;
use super::network::Network;
use super::updater;


pub fn run() {
	let args = match args::parse() {
		Some(args) => args,

		None => {
			os::set_exit_status(1);
			return;
		}
	};

	let mut network    = Network::new(args.port.as_slice());
	let mut game_state = game::State::new(network.events.clone());

	let frame_time_in_ms = args.frame_time;

	updater::init(frame_time_in_ms as u64, game_state.events.clone());

	game_state.events.send(Init);

	loop {
		network.update(frame_time_in_ms, &mut game_state.events);
		game_state.update();
	}
}

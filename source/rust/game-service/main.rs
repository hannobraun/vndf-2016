extern crate collections;
extern crate common;
extern crate getopts;
extern crate libc;
extern crate time;


use std::os;

use clients::Clients;
use events::Init;
use game::Game;
use network::Network;


mod args;
mod clients;
mod events;
mod game;
mod network;
mod updater;


fn main() {
	let args = match args::parse() {
		Some(args) => args,

		None => {
			os::set_exit_status(1);
			return;
		}
	};

	let mut network = Network::new(args.port.as_slice());
	let mut game    = Game::new(network.events.clone());
	let mut clients = Clients::new();

	let frame_time_in_ms = args.frame_time;

	updater::init(frame_time_in_ms as u64, game.events.clone());

	game.events.send(Init);

	loop {
		network.update(frame_time_in_ms, &mut game.events, &mut clients);

		game.handle();
	}
}

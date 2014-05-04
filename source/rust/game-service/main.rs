extern crate collections;
extern crate common;
extern crate getopts;
extern crate libc;
extern crate time;


use std::os;

use clients::Clients;
use events::{
	Init,
	Update
};
use game::Game;
use network::Network;


mod args;
mod clients;
mod events;
mod game;
mod network;


fn main() {
	let args = match args::parse() {
		Some(args) => args,

		None => {
			os::set_exit_status(1);
			return;
		}
	};

	let mut network = Network::new(args.port);
	let mut game    = Game::new(network.events.clone());
	let mut clients = Clients::new();

	let frame_time_in_ms = args.frame_time;

	game.events.send(Init);

	loop {
		network.update(frame_time_in_ms, &mut game.events, &mut clients);

		game.events.send(Update(frame_time_in_ms as f64 / 1000.0));
		game.handle(&mut clients);
	}
}

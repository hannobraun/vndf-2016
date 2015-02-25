#![feature(collections, core, env, old_io, libc, net, std_misc)]


extern crate getopts;
extern crate libc;
extern crate time;

extern crate client;
extern crate common;


mod args;
mod input;
mod platform;
mod render;
mod termios;
mod ui;


use std::collections::HashMap;
use std::env;
use std::old_io::timer::sleep;
use std::time::Duration;

use time::precise_time_s;

use args::Args;
use client::network::Network;
use client::platform::{
	Frame,
	InputEvent,
	Message,
};
use common::protocol::{
	ClientEvent,
	ServerEvent,
};
use platform::{
	HeadlessIo,
	PlatformIo,
	PlayerIo,
};


fn main() {
	let args = Args::parse(env::args());

	if args.headless {
		run(args, init_platform::<HeadlessIo>())
	}
	else {
		run(args, init_platform::<PlayerIo>())
	}
}


fn init_platform<P: PlatformIo>() -> P {
	match PlatformIo::new() {
		Ok(platform) =>
			platform,
		Err(error) =>
			panic!("Error initializing platform I/O: {}", error),
	}
}

fn run<P: PlatformIo>(args: Args, mut platform: P) {
	let mut frame = Frame::new();

	let mut broadcasts = HashMap::new();
	let mut network    = Network::new(args.server);

	let mut last_server_activity = precise_time_s();

	network.send(ClientEvent::Login);

	loop {
		let input_events = match platform.update(&frame) {
			Ok(events) => events,
			Err(error) => panic!("Error updating platform code: {}", error),
		};

		for event in input_events {
			match event {
				InputEvent::StartBroadcast(message) =>
					if message.len() == 0 {
						frame.status = Message::Error(
							"Broadcasts can not be empty".to_string()
						);
					}
					else if message.len() > 256 {
						frame.status = Message::Error(
							"Broadcast message too long".to_string()
						);
					}
					else {
						network.send(
							ClientEvent::StartBroadcast(message.clone())
						);

						frame.status = Message::Notice(
							"Sending broadcast".to_string()
						);
					},
				InputEvent::StopBroadcast => {
					network.send(ClientEvent::StopBroadcast);

					frame.status = Message::Notice(
						"Stopped sending broadcast".to_string()
					);
				},
			}
		}

		for event in network.receive() {
			match event {
				ServerEvent::Heartbeat => {
					// Nothing to do here. The activity time is updated below
					// for all types of messages.
				},
				ServerEvent::SelfId(self_id) => {
					frame.self_id = self_id;
				},
				ServerEvent::StartBroadcast(broadcast) => {
					broadcasts.insert(broadcast.sender.clone(), broadcast);
				},
				ServerEvent::StopBroadcast(sender) => {
					broadcasts.remove(&sender);
				},
			}

			last_server_activity = precise_time_s();
		}

		if precise_time_s() - last_server_activity > args.net_timeout_s {
			frame.status = Message::Error(
				"Lost connection to server".to_string()
			);
		}

		frame.broadcasts = broadcasts
			.iter()
			.map(|(_, broadcast)|
				broadcast.clone()
			)
			.collect();

		network.send(ClientEvent::Heartbeat);

		sleep(Duration::milliseconds(20));
	}
}

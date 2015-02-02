#![feature(collections, core, io, libc, os, slicing_syntax, std_misc)]


extern crate getopts;
extern crate libc;

extern crate client;
extern crate common;


mod args;
mod platform;
mod render;
mod termios;
mod ui;


use std::collections::HashMap;
use std::old_io::timer::sleep;
use std::time::Duration;

use args::Args;
use client::network::Network;
use client::platform::{
	Frame,
	InputEvent,
	Status,
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
	let args = Args::parse(std::os::args().as_slice());

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

	let mut broadcasts     = HashMap::new();
	let mut network        = Network::new(args.server);

	network.send(ClientEvent::Login);

	loop {
		for event in platform.update() {
			match event {
				InputEvent::StartBroadcast(message) =>
					if message.len() == 0 {
						frame.status = Status::Error(
							"Broadcasts can not be empty".to_string()
						);
					}
					else if message.len() > 256 {
						frame.status = Status::Error(
							"Broadcast message too long".to_string()
						);
					}
					else {
						network.send(
							ClientEvent::StartBroadcast(message.clone())
						);

						frame.status = Status::Notice(
							"Sending broadcast".to_string()
						);
					},
				InputEvent::StopBroadcast => {
					network.send(ClientEvent::StopBroadcast);

					frame.status = Status::Notice(
						"Stopped sending broadcast".to_string()
					);
				},
			}
		}

		for event in network.receive() {
			match event {
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
		}

		frame.broadcasts = broadcasts
			.iter()
			.map(|(_, broadcast)|
				broadcast.clone()
			)
			.collect();

		network.send(ClientEvent::Heartbeat);

		if let Err(error) = platform.render(&frame) {
			panic!("Error writing output: {}", error);
		}

		sleep(Duration::milliseconds(20));
	}
}

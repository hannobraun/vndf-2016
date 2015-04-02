#![feature(as_slice, collections, convert, io, libc, plugin, std_misc, thread_sleep)]
#![feature(custom_attribute)]
#![plugin(gfx_macros)]


extern crate freetype;
extern crate getopts;
extern crate gfx;
extern crate gfx_device_gl;
extern crate glutin;
extern crate libc;
extern crate nalgebra;
extern crate time;

extern crate client;
extern crate common;


mod args;
mod cli;
mod font;
mod interface;
mod render;
mod termios;
mod texture;
mod ui;
mod window;


use std::collections::HashMap;
use std::env;
use std::thread::sleep;

use nalgebra::Cast;
use time::precise_time_s;

use args::Args;
use client::interface::{
	Frame,
	InputEvent,
	Message,
};
use client::network::Network;
use common::protocol::{
	ClientEvent,
	ServerEvent,
};
use interface::Interface;


fn main() {
	let args = Args::parse(env::args());

	if args.headless {
		run(args, init_interface::<interface::Headless>())
	}
	else if args.cli {
		run(args, init_interface::<interface::CommandLine>())
	}
	else {
		run(args, init_interface::<interface::Player>())
	}
}


fn init_interface<I: Interface>() -> I {
	match Interface::new() {
		Ok(interface) => interface,
		Err(error)    => panic!("Error initializing interface: {}", error),
	}
}

fn run<I: Interface>(args: Args, mut interface: I) {
	let mut frame = Frame::new();

	let mut broadcasts = HashMap::new();
	let mut network    = Network::new(args.server);

	let mut last_server_activity = precise_time_s();

	network.send(ClientEvent::Login);

	'main: loop {
		let input_events = match interface.update(&frame) {
			Ok(events) => events,
			Err(error) => panic!("Error updating interface: {}", error),
		};

		for event in input_events {
			match event {
				InputEvent::StartBroadcast(message) =>
					if message.len() == 0 {
						frame.message = Message::Error(
							"Broadcasts can not be empty".to_string()
						);
					}
					else if message.len() > 256 {
						frame.message = Message::Error(
							"Broadcast message too long".to_string()
						);
					}
					else {
						network.send(
							ClientEvent::StartBroadcast(message.clone())
						);

						frame.message = Message::Notice(
							"Sending broadcast".to_string()
						);
					},
				InputEvent::StopBroadcast => {
					network.send(ClientEvent::StopBroadcast);

					frame.message = Message::Notice(
						"Stopped sending broadcast".to_string()
					);
				},
				InputEvent::Quit => {
					break 'main;
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
				ServerEvent::UpdateEntity(position, velocity) => {
					frame.position = Cast::from(position);
					frame.velocity = Cast::from(velocity);
				},
			}

			last_server_activity = precise_time_s();
		}

		if precise_time_s() - last_server_activity > args.net_timeout_s {
			frame.message = Message::Error(
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

		sleep(args.sleep_duration);
	}
}

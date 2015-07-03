#![feature(drain)]
#![feature(custom_attribute)]


extern crate freetype;
extern crate getopts;
#[macro_use]
extern crate gfx;
extern crate gfx_device_gl;
extern crate glutin;
extern crate libc;
extern crate nalgebra;
#[macro_use]
extern crate scan_fmt;
extern crate time;

extern crate client;
extern crate shared;


mod args;
mod cli;
mod font;
mod interface;
mod render;
mod window;


use std::collections::HashMap;
use std::env;
use std::thread::sleep_ms;

use nalgebra::Cast;
use time::precise_time_s;

use args::Args;
use client::interface::{
	Frame,
	InputEvent,
	Message,
};
use client::network::Network;
use shared::protocol::client::Event as ClientEvent;
use shared::protocol::client::event as client_event;
use shared::protocol::server;
use interface::Interface;


fn main() {
	let args = Args::parse(env::args());

	if args.headless {
		run(args, init_interface::<interface::Headless>())
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

	network.send(ClientEvent::Public(client_event::Public::Login));

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
							ClientEvent::Privileged(client_event::Privileged::StartBroadcast(message.clone()))
						);

						frame.message = Message::Notice(
							"Sending broadcast".to_string()
						);
					},
				InputEvent::StopBroadcast => {
					network.send(ClientEvent::Privileged(client_event::Privileged::StopBroadcast));

					frame.message = Message::Notice(
						"Stopped sending broadcast".to_string()
					);
				},
				InputEvent::ScheduleManeuver(angle) => {
					network.send(ClientEvent::Privileged(client_event::Privileged::ScheduleManeuver(angle as f32)));

					frame.message = Message::Notice(
						"Scheduling maneuver".to_string()
					);
				},
				InputEvent::Quit => {
					break 'main;
				},
			}
		}

		for event in network.receive() {
			match event {
				server::Event::Heartbeat => {
					// Nothing to do here. The activity time is updated below
					// for all types of messages.
				},
				server::Event::ShipId(ship_id) => {
					frame.ship_id = Some(ship_id);
				},
				server::Event::StartBroadcast(broadcast) => {
					broadcasts.insert(broadcast.sender, broadcast);
				},
				server::Event::StopBroadcast(sender) => {
					broadcasts.remove(&sender);
				},
				server::Event::UpdateShip(ship) => {
					frame.position = Cast::from(ship.position);
					frame.velocity = Cast::from(ship.velocity);
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

		network.send(ClientEvent::Privileged(client_event::Privileged::Heartbeat));

		sleep_ms(args.sleep_ms);
	}
}

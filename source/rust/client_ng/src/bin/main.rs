#![feature(default_type_params)]
#![feature(slicing_syntax)]


extern crate getopts;
extern crate libc;

extern crate acpe;

extern crate client;
extern crate common;


use std::io::timer::sleep;
use std::time::Duration;

use acpe::MAX_PACKET_SIZE;
use acpe::protocol::Encoder;

use action_assembler::ActionAssembler;
use args::Args;
use client::network::Socket;
use client::platform::{
	Frame,
	Input,
	Status,
};
use common::protocol::{
	Percept,
	Step,
};
use platform::{
	HeadlessIo,
	PlatformIo,
	PlayerIo,
};


mod action_assembler;
mod args;
mod platform;
mod termios;


fn main() {
	let args = Args::parse(std::os::args().as_slice());

	if args.headless {
		run::<HeadlessIo>(args, init_platform())
	}
	else {
		run::<PlayerIo>(args, init_platform())
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
	let mut frame = Frame {
		self_id   : String::new(),
		status    : Status::None,
		broadcasts: vec![],
	};

	let mut previous_input = Input::new();

	let mut action_assembler = ActionAssembler::new();
	let mut server           = Socket::new(args.server);
	let mut encoder          = Encoder::new();

	action_assembler.add_step(Step::Login);

	loop {
		let input = platform.input();

		if input != previous_input {
			match input.broadcast {
				Some(ref message) =>
					if message.len() == 0 {
						frame.status = Status::Error(
							"Broadcasts can not be empty".to_string()
						);
					}
					else if message.len() > MAX_PACKET_SIZE / 2 {
						frame.status = Status::Error(
							"Broadcast message too long".to_string()
						);
					}
					else {
						action_assembler.add_step(
							Step::Broadcast(message.clone())
						);

						frame.status = Status::Notice(
							"Sending broadcast".to_string()
						);
					},
				None => {
					action_assembler.add_step(Step::StopBroadcast);

					frame.status = Status::Notice(
						"Stopped sending broadcast".to_string()
					);
				},
			}
		}

		previous_input = input.clone();

		for perception in  server.receive().into_iter() {
			if let Some(self_id) = perception.header.self_id {
				frame.self_id = self_id;
			}

			frame.broadcasts = perception.update
				.into_iter()
				.map(|percept|
					match percept {
						Percept::Broadcast(broadcast) => broadcast,
					}
				)
				.collect();
			action_assembler.process_receipt(perception.header.confirm_action);
		}

		let message = action_assembler.assemble(&mut encoder);

		server.send_to(message.as_slice());
		if let Err(error) = platform.render(&frame) {
			panic!("Error writing output: {}", error);
		}

		sleep(Duration::milliseconds(20));
	}
}

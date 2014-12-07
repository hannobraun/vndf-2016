#![feature(default_type_params)]
#![feature(slicing_syntax)]


extern crate getopts;
extern crate libc;

extern crate acpe;

extern crate client;
extern crate common;


use std::io::timer::sleep;
use std::time::Duration;

use acpe::protocol::Encoder;

use action_assembler::ActionAssembler;
use args::Args;
use client::network::Socket;
use client::output::Frame;
use common::protocol::{
	Broadcast,
	Percept,
	Step,
};
use input::{
	Command,
	CommandError,
	Input,
};
use output::{
	HeadlessOutput,
	Output,
	PlayerOutput,
};


mod action_assembler;
mod args;
mod input;
mod output;
mod termios;


fn main() {
	let args = Args::parse(std::os::args().as_slice());

	if args.headless {
		match HeadlessOutput::new() {
			Ok(output) => run(args, output),
			Err(error) => panic!("Error initializing output: {}", error),
		}
	}
	else {
		match PlayerOutput::new() {
			Ok(output) => run(args, output),
			Err(error) => panic!("Error initializing output: {}", error),
		}
	}
}


fn run<O: Output>(args: Args, mut output: O) {
	let mut frame = Frame {
		self_id   : String::new(),
		input     : String::new(),
		status    : String::new(),
		commands  : vec![],
		broadcasts: vec![],
	};

	let mut input            = Input::new();
	let mut action_assembler = ActionAssembler::new();
	let mut server           = Socket::new(args.server);
	let mut encoder          = Encoder::new();

	action_assembler.add_step(Step::Login);

	loop {
		for result in input.read_commands().into_iter() {
			match result {
				Ok(command) => {
					let mut reset_status = true;

					match command {
						Command::Broadcast(message) =>
							action_assembler.add_step(Step::Broadcast(message)),
						Command::StopBroadcast =>
							action_assembler.add_step(Step::StopBroadcast),
						Command::Help(text) => {
							frame.status = text.to_string();
							reset_status = false;
						},
					}

					if reset_status {
						frame.status.clear();
					}
				},
				Err(error) => match error {
					CommandError::Incomplete(partial_command, applicable) => {
						frame.input    = partial_command;
						frame.commands = applicable;
					},
					CommandError::Invalid(error, command) =>
						frame.status = format!("\"{}\": {}", command, error),
				}
			}
		}

		for perception in  server.recv_from().into_iter() {
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

		server.send_to(message);
		match output.render(&frame) {
			Ok(())     => (),
			Err(error) => panic!("Error writing output: {}", error),
		}

		sleep(Duration::milliseconds(20));
	}
}

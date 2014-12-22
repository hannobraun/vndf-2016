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
use client::platform::{
	Frame,
	Input,
	Status,
};
use common::protocol::{
	Percept,
	Step,
};
use platform::input::{
	HeadlessInputReader,
	InputReader,
	ReadInput,
};
use platform::render::{
	HeadlessRenderer,
	Render,
	Renderer,
};


mod action_assembler;
mod args;
mod platform;
mod termios;


fn main() {
	let args = Args::parse(std::os::args().as_slice());

	if args.headless {
		let input_reader = HeadlessInputReader::new();

		let renderer = match HeadlessRenderer::new() {
			Ok(renderer) => renderer,
			Err(error)   => panic!("Error initializing renderer: {}", error),
		};

		run(args, input_reader, renderer)
	}
	else {
		let input_reader = InputReader::new();

		let renderer = match Renderer::new() {
			Ok(renderer) => renderer,
			Err(error)   => panic!("Error initializing renderer: {}", error),
		};

		run(args, input_reader, renderer)
	}
}


fn run<I: ReadInput, R: Render>(
	    args        : Args,
	mut input_reader: I,
	mut renderer    : R
) {
	let mut frame = Frame {
		self_id   : String::new(),
		input     : String::new(),
		status    : Status::None,
		commands  : vec![],
		broadcasts: vec![],
	};

	let mut previous_input = Input::new();

	let mut action_assembler = ActionAssembler::new();
	let mut server           = Socket::new(args.server);
	let mut encoder          = Encoder::new();

	action_assembler.add_step(Step::Login);

	loop {
		let input = input_reader.input();

		if input != previous_input {
			match input.broadcast {
				Some(ref message) =>
					// TODO(84970610): Reject broadcasts that are too large to
					//                 fit into a UDP packet.
					action_assembler.add_step(Step::Broadcast(message.clone())),
				None =>
					action_assembler.add_step(Step::StopBroadcast),
			}

			frame.status = Status::None;
		}

		let (partial_command, applicable) = input.command.clone();
		frame.input    = partial_command;
		frame.commands = applicable;

		if let Some((error, command)) = input.error.clone() {
			frame.status = Status::Error(format!("\"{}\": {}", command, error))
		}

		previous_input = input.clone();

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

		server.send_to(message.as_slice());
		if let Err(error) = renderer.render(&frame) {
			panic!("Error writing output: {}", error);
		}

		sleep(Duration::milliseconds(20));
	}
}

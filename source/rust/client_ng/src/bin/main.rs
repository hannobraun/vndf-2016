#![feature(slicing_syntax)]


extern crate getopts;
extern crate libc;

extern crate acpe;

extern crate client_ng;
extern crate protocol_ng;


use std::io::timer::sleep;
use std::time::Duration;

use acpe::protocol::Encoder;

use action_assembler::ActionAssembler;
use args::Args;
use client_ng::{
	Frame,
	Server,
};
use input::Input;
use output::{
	HeadlessOutput,
	Output,
	PlayerOutput,
};
use protocol_ng::protocol::{
	Percept,
	Step,
};


mod action_assembler;
mod args;
mod input;
mod output;
mod termios;


fn main() {
	let args = Args::parse(std::os::args().as_slice());

	if args.headless {
		run(args, HeadlessOutput::new());
	}
	else {
		run(args, PlayerOutput::new());
	}
}


fn run<O: Output>(args: Args, mut output: O) {
	let mut frame = Frame {
		broadcasts: vec![],
	};

	let     input            = Input::new();
	let mut action_assembler = ActionAssembler::new();
	let mut server           = Server::new(args.server);
	let mut encoder          = Encoder::new();

	action_assembler.add_step(Step::Login);

	loop {
		for line in input.read_line().into_iter() {
			action_assembler.add_step(Step::Broadcast(line));
		}

		match server.recv_from() {
			Some(perception) => {
				frame.broadcasts = perception.percepts
					.into_iter()
					.map(|percept|
						match percept {
							Percept::Broadcast(broadcast) => broadcast
						}
					)
					.collect();
				action_assembler.process_receipt(perception.last_action);
			},
			None => (),
		}

		let message = action_assembler.assemble(&mut encoder);

		server.send_to(message);
		output.render(&frame);

		sleep(Duration::milliseconds(20));
	}
}

#![feature(slicing_syntax)]


extern crate getopts;
extern crate libc;

extern crate client_ng;
extern crate protocol_ng;


use std::io::timer::sleep;
use std::time::Duration;

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
use protocol_ng::{
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

		let action = action_assembler.assemble();
		// TODO(83504690): Action may be too large to fit into a single UDP
		//                 package. Research suggests that, given typical MTU
		//                 sizes, 512 bytes are a safe bet for the maximum size.
		server.send_to(action.encode().as_slice());
		output.render(&frame);

		sleep(Duration::milliseconds(20));
	}
}

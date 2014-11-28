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
use protocol_ng::Step;


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
		match input.read_line() {
			Some(line) => {
				action_assembler.add_step(Step::Broadcast(line));
			},

			None => (),
		}
		match server.recv_from() {
			Some(perception) => {
				frame.broadcasts = perception.broadcasts;
				action_assembler.process_receipt(perception.last_action);
			},
			None => (),
		}

		let action = action_assembler.assemble();
		server.send_to(action.to_json().as_bytes());
		output.render(&frame);

		sleep(Duration::milliseconds(20));
	}
}

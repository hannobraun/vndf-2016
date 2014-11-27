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
	let args   = Args::parse(std::os::args().as_slice());
	let input  = Input::new();
	let server = Server::new(args.server);

	if args.headless {
		run(input, server, HeadlessOutput::new())
	}
	else {
		run(input, server, PlayerOutput::new());
	}
}


fn run<O: Output>(input : Input, mut server: Server, mut output: O) {
	let mut frame = Frame {
		broadcasts: vec![],
	};

	let mut action_assembler = ActionAssembler::new();

	server.send_to(action_assembler.assemble(Step::Login));

	loop {
		match input.read_line() {
			Some(line) => {
				let action = action_assembler.assemble(Step::Broadcast(line));
				server.send_to(action);
			},

			None => (),
		}
		match server.recv_from() {
			Some(perception) => frame.broadcasts = perception.broadcasts,
			None             => (),
		}

		output.render(&frame);

		sleep(Duration::milliseconds(10));
	}
}

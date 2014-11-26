#![feature(slicing_syntax)]


extern crate getopts;
extern crate libc;

extern crate client_ng;
extern crate protocol_ng;


use std::io::timer::sleep;
use std::time::Duration;

use args::Args;
use client_ng::Frame;
use input::Input;
use output::{
	HeadlessOutput,
	Output,
	PlayerOutput,
};
use protocol_ng::Action;
use server::Server;


mod args;
mod input;
mod output;
mod server;
mod termios;


fn main() {
	let args   = Args::parse(std::os::args().as_slice());
	let input  = Input::new();
	let server = Server::new(args.port);

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

	server.send_to(Action::Login);

	loop {
		match input.read_line() {
			Some(line) => server.send_to(Action::Broadcast(line)),
			None       => (),
		}
		match server.recv_from() {
			// TODO: Just setting the received broadcast as the only one will
			//       not be enough.
			Some(broadcast) => frame.broadcasts = vec![broadcast],
			None            => (),
		}

		output.render(&frame);

		sleep(Duration::milliseconds(10));
	}
}

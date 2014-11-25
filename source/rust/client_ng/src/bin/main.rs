#![feature(slicing_syntax)]


extern crate getopts;
extern crate libc;

extern crate client_ng;


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


fn run<O: Output>(input : Input, server: Server, mut output: O) {
	let mut frame = Frame {
		broadcasts: vec![],
	};

	loop {
		match input.read_line() {
			Some(_) => (),
			None    => (),
		}
		match server.recv_from() {
			Some(broadcast) => frame.broadcasts = vec![broadcast],
			None            => (),
		}

		output.render(&frame);

		sleep(Duration::milliseconds(10));
	}
}

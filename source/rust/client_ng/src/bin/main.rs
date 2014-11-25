#![feature(slicing_syntax)]


extern crate getopts;
extern crate libc;

extern crate client_ng;


use std::io::timer::sleep;
use std::time::Duration;
use std::comm::TryRecvError;

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
mod server;
mod termios;
mod output;


fn main() {
	let args   = Args::parse(std::os::args().as_slice());
	let input  = Input::new();
	let server = Server::new(args.port);

	if args.headless {
		run(input.receiver, server, HeadlessOutput::new())
	}
	else {
		run(input.receiver, server, PlayerOutput::new());
	}
}


fn run<O: Output>(
	    input : Receiver<String>,
	    server: Server,
	mut output: O
) {
	let mut frame = Frame {
		broadcasts: vec![],
	};

	loop {
		match input.try_recv() {
			Ok(_) => (),

			Err(error) => match error {
				TryRecvError::Empty        => (),
				TryRecvError::Disconnected => panic!("Channel disconnected"),
			}
		}
		match server.recv_from() {
			Some(broadcast) => frame.broadcasts = vec![broadcast],
			None            => (),
		}

		output.render(&frame);

		sleep(Duration::milliseconds(10));
	}
}

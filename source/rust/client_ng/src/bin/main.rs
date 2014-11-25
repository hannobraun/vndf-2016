#![feature(slicing_syntax)]


extern crate getopts;
extern crate libc;

extern crate client_ng;


use std::io::stdin;
use std::io::timer::sleep;
use std::time::Duration;
use std::comm::TryRecvError;

use args::Args;
use client_ng::Frame;
use output::{
	HeadlessOutput,
	Output,
	PlayerOutput,
};
use server::Server;


mod args;
mod server;
mod termios;
mod output;


fn main() {
	let args   = Args::parse(std::os::args().as_slice());
	let input  = input();
	let server = Server::new(args.port);

	if args.headless {
		run(input, server, HeadlessOutput::new())
	}
	else {
		run(input, server, PlayerOutput::new());
	}
}


fn input() -> Receiver<String> {
	let (sender, receiver) = channel();

	spawn(proc() {
		let mut stdin = stdin();

		loop {
			match stdin.read_line() {
				Ok(line)   => sender.send(line[.. line.len() - 1].to_string()),
				Err(error) => panic!("Error reading from stdint: {}", error),
			}
		}
	});

	receiver
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
		match server.receiver.try_recv() {
			Ok(broadcast) => frame.broadcasts = vec![broadcast],

			Err(error) => match error {
				TryRecvError::Empty        => (),
				TryRecvError::Disconnected => panic!("Channel disconnected"),
			}
		}

		output.render(&frame);

		sleep(Duration::milliseconds(200));
	}
}

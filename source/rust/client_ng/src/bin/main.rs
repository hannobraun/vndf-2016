#![feature(slicing_syntax)]


extern crate getopts;
extern crate libc;

extern crate client_ng;


use std::io::net::ip::Port;
use std::io::net::udp::UdpSocket;
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


mod args;
mod termios;
mod output;


fn main() {
	let args  = Args::parse(std::os::args().as_slice());
	let input = input();

	if args.headless {
		run(input, HeadlessOutput::new())
	}
	else {
		run(input, PlayerOutput::new());
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

fn run<O: Output>(input: Receiver<String>, mut output: O) {
	let frame = Frame {
		broadcasts: vec!["This is a broadcast.".to_string()],
	};

	loop {
		match input.try_recv() {
			Ok(_) => (),

			Err(error) => match error {
				TryRecvError::Empty        => (),
				TryRecvError::Disconnected => panic!("Channel disconnected"),
			}
		}

		output.render(&frame);

		sleep(Duration::milliseconds(200));
	}
}

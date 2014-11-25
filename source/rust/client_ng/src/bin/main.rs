#![feature(slicing_syntax)]


extern crate libc;

extern crate client_ng;


use std::io::stdin;
use std::io::timer::sleep;
use std::time::Duration;
use std::comm::TryRecvError;

use client_ng::Frame;
use output::{
	HeadlessOutput,
	Output,
	PlayerOutput,
};


mod termios;
mod output;


fn main() {
	let input  = input();

	let args = std::os::args();
	if args.len() > 1 && args[1] == "--headless".to_string() {
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

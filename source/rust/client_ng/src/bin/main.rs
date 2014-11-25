#![feature(slicing_syntax)]


extern crate libc;

extern crate client_ng;


use std::io::stdin;
use std::io::timer::sleep;
use std::time::Duration;
use std::comm::TryRecvError;

use client_ng::Frame;
use termios::Termios;


mod termios;


fn main() {
	// This is a hack to get headless mode to work as far as the test case is
	// concerned. Needs to be cleaned up later, obviously.
	let args = std::os::args();
	if args.len() > 1 && args[1] == "--headless".to_string() {
		let frame = Frame {
			broadcasts: vec!["This is a broadcast.".to_string()],
		};

		loop {
			print!("{}\n", frame.to_json());
		}
	}

	let mut termios = Termios::get(libc::STDIN_FILENO);
	termios.echo(false);
	termios.canonical_input(false);
	termios.set(libc::STDIN_FILENO);

	let input = input();

	let mut i = 0u8;

	print!("\n");
	loop {
		match input.try_recv() {
			Ok(line) => match from_str(line.as_slice()) {
				Some(n) => i = n,
				None    => (),
			},

			Err(error) => match error {
				TryRecvError::Empty        => (),
				TryRecvError::Disconnected => panic!("Channel disconnected"),
			}
		}

		write(i);

		i += 1;
		sleep(Duration::milliseconds(200));
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

fn write(i: u8) {
	print!("\x1b[1A\x1b[2K");
	print!("{}\n", i);
}

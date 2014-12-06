use libc;
use std::io::{
	stdout,
	IoResult,
	LineBufferedWriter,
};
use std::io::stdio::StdWriter;

use client::output::Frame;
use termios::Termios;


pub trait Output {
	fn render(&mut self, frame: &Frame) -> IoResult<()>;
}


pub struct PlayerOutput {
	stdout: LineBufferedWriter<StdWriter>,
}

impl PlayerOutput {
	pub fn new() -> PlayerOutput {
		let mut termios = Termios::get(libc::STDIN_FILENO);
		termios.echo(false);
		termios.canonical_input(false);
		termios.set(libc::STDIN_FILENO);

		PlayerOutput {
			stdout: stdout(),
		}
	}
}

impl Output for PlayerOutput {
	fn render(&mut self, frame: &Frame) -> IoResult<()> {
		try!(write!(&mut self.stdout, "\x1b[2J\x1b[H"));

		try!(write!(&mut self.stdout, "Your Comm ID: {}\n\n", frame.self_id));

		try!(write!(&mut self.stdout, "BROADCASTS\n"));
		if frame.broadcasts.len() == 0 {
			try!(write!(&mut self.stdout, "    none\n"));
		}
		for broadcast in frame.broadcasts.iter() {
			try!(write!(
				&mut self.stdout,
				"    {}: {}\n",
				broadcast.sender,
				broadcast.message
			));
		}

		try!(write!(&mut self.stdout, "\nCOMMANDS\n    "));
		if frame.commands.len() == 0 {
			try!(write!(&mut self.stdout, "none"));
		}
		for command in frame.commands.iter() {
			try!(write!(&mut self.stdout, "{}    ", command));
		}

		try!(write!(&mut self.stdout, "\n\n{}\n", frame.error));

		try!(write!(&mut self.stdout, "Enter command: {}", frame.input));
		try!(self.stdout.flush());


		Ok(())
	}
}


pub struct HeadlessOutput;

impl HeadlessOutput {
	pub fn new() -> HeadlessOutput {
		HeadlessOutput
	}
}

impl Output for HeadlessOutput {
	fn render(&mut self, frame: &Frame) -> IoResult<()> {
		print!("{}\n", frame.to_json());
		Ok(())
	}
}

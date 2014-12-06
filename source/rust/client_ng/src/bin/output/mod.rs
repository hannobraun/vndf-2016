use std::io::IoResult;

use client::output::Frame;

use self::screen::Screen;


mod screen;


pub trait Output {
	fn render(&mut self, frame: &Frame) -> IoResult<()>;
}


pub struct PlayerOutput {
	screen: Screen,
}

impl PlayerOutput {
	pub fn new() -> PlayerOutput {
		PlayerOutput {
			screen: Screen::new(),
		}
	}
}

impl Output for PlayerOutput {
	fn render(&mut self, frame: &Frame) -> IoResult<()> {
		try!(self.screen.clear());

		try!(write!(
			&mut self.screen.buffer(),
			"Your Comm ID: {}\n\n",
			frame.self_id
		));

		try!(write!(&mut self.screen.buffer(), "BROADCASTS\n"));
		if frame.broadcasts.len() == 0 {
			try!(write!(&mut self.screen.buffer(), "    none\n"));
		}
		for broadcast in frame.broadcasts.iter() {
			try!(write!(
				&mut self.screen.buffer(),
				"    {}: {}\n",
				broadcast.sender,
				broadcast.message
			));
		}

		try!(write!(&mut self.screen.buffer(), "\nCOMMANDS\n    "));
		if frame.commands.len() == 0 {
			try!(write!(&mut self.screen.buffer(), "none"));
		}
		for command in frame.commands.iter() {
			try!(write!(&mut self.screen.buffer(), "{}    ", command));
		}

		try!(write!(&mut self.screen.buffer(), "\n\n{}\n", frame.status));

		try!(write!(
			&mut self.screen.buffer(),
			"Enter command: {}",
			frame.input
		));

		try!(self.screen.submit());

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

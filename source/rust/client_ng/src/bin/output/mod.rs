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
		let mut y = 0;

		try!(write!(
			&mut self.screen.buffer_at(0, y),
			"Your Comm ID: {}",
			frame.self_id
		));
		y += 2;

		try!(write!(
			&mut self.screen.buffer_at(0, y),
			"BROADCASTS")
		);
		y += 1;

		if frame.broadcasts.len() == 0 {
			try!(write!(
				&mut self.screen.buffer_at(4, y),
				"none"
			));
			y += 1;
		}

		for broadcast in frame.broadcasts.iter() {
			try!(write!(
				&mut self.screen.buffer_at(4, y),
				"{}: {}\n",
				broadcast.sender, broadcast.message
			));
			y += 1;
		}
		y += 1;

		try!(write!(
			&mut self.screen.buffer_at(0, y),
			"COMMANDS"
		));
		y += 1;

		if frame.commands.len() == 0 {
			try!(write!(
				&mut self.screen.buffer_at(4, y),
				"none"
			));
		}
		y += 1;

		let mut x = 4;
		for command in frame.commands.iter() {
			try!(write!(
				&mut self.screen.buffer_at(x, y), "{}",
				command
			));
			x += 4 + command.len() as u16;
		}

		y += 2;
		try!(write!(
			&mut self.screen.buffer_at(0, y),
			"{}",
			frame.status
		));
		y += 1;

		try!(write!(
			&mut self.screen.buffer_at(0, y),
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

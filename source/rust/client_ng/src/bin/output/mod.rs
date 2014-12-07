use std::io::IoResult;

use client::output::Frame;

use self::screen::Screen;


mod screen;


pub trait Output {
	fn render(&mut self, frame: &Frame) -> IoResult<()>;
}


pub struct PlayerOutput {
	screen: Screen,

	x: u16,
	y: u16,
}

impl PlayerOutput {
	pub fn new() -> PlayerOutput {
		PlayerOutput {
			screen: Screen::new(),

			x: 0,
			y: 0,
		}
	}
}

impl Output for PlayerOutput {
	fn render(&mut self, frame: &Frame) -> IoResult<()> {
		try!(self.screen.clear());
		self.x = 0;
		self.y = 0;

		try!(self.render_comm_id(frame));
		try!(self.render_broadcasts(frame));
		try!(self.render_commands(frame));

		self.y += 2;
		try!(write!(
			&mut self.screen.buffer_at(0, self.y),
			"{}",
			frame.status
		));
		self.y += 1;

		try!(write!(
			&mut self.screen.buffer_at(0, self.y),
			"Enter command: {}",
			frame.input
		));

		try!(self.screen.submit());

		Ok(())
	}
}

impl PlayerOutput {
	fn render_comm_id(&mut self, frame: &Frame) -> IoResult<()> {
		try!(write!(
			&mut self.screen.buffer_at(0, self.y),
			"Your Comm ID: {}",
			frame.self_id
		));

		self.y += 2;
		Ok(())
	}

	fn render_broadcasts(&mut self, frame: &Frame) -> IoResult<()> {
		try!(write!(
			&mut self.screen.buffer_at(0, self.y),
			"BROADCASTS")
		);
		self.y += 1;

		if frame.broadcasts.len() == 0 {
			try!(write!(
				&mut self.screen.buffer_at(4, self.y),
				"none"
			));
			self.y += 1;
		}

		for broadcast in frame.broadcasts.iter() {
			try!(write!(
				&mut self.screen.buffer_at(4, self.y),
				"{}: {}\n",
				broadcast.sender, broadcast.message
			));
			self.y += 1;
		}
		self.y += 1;

		Ok(())
	}

	fn render_commands(&mut self, frame: &Frame) -> IoResult<()> {
		try!(write!(
			&mut self.screen.buffer_at(0, self.y),
			"COMMANDS"
		));
		self.y += 1;

		if frame.commands.len() == 0 {
			try!(write!(
				&mut self.screen.buffer_at(4, self.y),
				"none"
			));
		}
		self.y += 1;

		self.x = 4;
		for command in frame.commands.iter() {
			try!(write!(
				&mut self.screen.buffer_at(self.x, self.y), "{}",
				command
			));
			self.x += 4 + command.len() as u16;
		}

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

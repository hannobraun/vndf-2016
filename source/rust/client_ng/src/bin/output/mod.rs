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
	pub fn new() -> IoResult<PlayerOutput> {
		let screen = match Screen::new(80, 24) {
			Ok(screen) => screen,
			Err(error) => return Err(error),
		};

		Ok(PlayerOutput {
			screen: screen,

			x: 0,
			y: 0,
		})
	}
}

impl Output for PlayerOutput {
	fn render(&mut self, frame: &Frame) -> IoResult<()> {
		self.x = 0;
		self.y = 0;

		try!(self.render_ship_info(frame));
		try!(self.render_broadcasts(frame));
		try!(self.render_commands(frame));
		try!(self.render_status(frame));
		try!(self.render_input(frame));

		try!(self.screen.submit());

		Ok(())
	}
}

impl PlayerOutput {
	fn render_ship_info(&mut self, frame: &Frame) -> IoResult<()> {
		let screen_width = self.screen.width();

		try!(write!(
			&mut self.screen.buffer(0, self.y, screen_width),
			"Your Comm ID: {}",
			frame.self_id
		));

		self.y += 2;
		Ok(())
	}

	fn render_broadcasts(&mut self, frame: &Frame) -> IoResult<()> {
		let screen_width = self.screen.width();

		try!(write!(
			&mut self.screen.buffer(0, self.y, screen_width),
			"BROADCASTS")
		);
		self.y += 1;

		if frame.broadcasts.len() == 0 {
			try!(write!(
				&mut self.screen.buffer(4, self.y, screen_width),
				"none"
			));
			self.y += 1;
		}

		for broadcast in frame.broadcasts.iter() {
			try!(write!(
				&mut self.screen.buffer(4, self.y, screen_width),
				"{}: {}",
				broadcast.sender, broadcast.message
			));
			self.y += 1;
		}
		self.y += 1;

		Ok(())
	}

	fn render_commands(&mut self, frame: &Frame) -> IoResult<()> {
		let screen_width = self.screen.width();

		try!(write!(
			&mut self.screen.buffer(0, self.y, screen_width),
			"COMMANDS"
		));
		self.y += 1;

		if frame.commands.len() == 0 {
			try!(write!(
				&mut self.screen.buffer(4, self.y, screen_width),
				"none"
			));
		}

		self.x = 4;
		for command in frame.commands.iter() {
			try!(write!(
				&mut self.screen.buffer(self.x, self.y, 15),
				"{}",
				command
			));
			self.x += 4 + command.len() as u16;
		}

		self.y += 1;

		Ok(())
	}

	fn render_status(&mut self, frame: &Frame) -> IoResult<()> {
		let screen_width = self.screen.width();

		self.y += 2;
		try!(write!(
			&mut self.screen.buffer(0, self.y, screen_width),
			"{}",
			frame.status
		));
		self.y += 1;

		Ok(())
	}

	fn render_input(&mut self, frame: &Frame) -> IoResult<()> {
		let screen_width = self.screen.width();
		let input_prompt = format!("Enter command: {}", frame.input);

		try!(
			self.screen
				.buffer(0, self.y, screen_width)
				.write(input_prompt.as_bytes())
		);
		self.screen.set_cursor(input_prompt.len() as u16, self.y);

		Ok(())
	}
}


pub struct HeadlessOutput;

impl HeadlessOutput {
	pub fn new() -> IoResult<HeadlessOutput> {
		Ok(HeadlessOutput)
	}
}

impl Output for HeadlessOutput {
	fn render(&mut self, frame: &Frame) -> IoResult<()> {
		print!("{}\n", frame.to_json());
		Ok(())
	}
}

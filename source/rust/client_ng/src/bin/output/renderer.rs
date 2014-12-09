use std::io::IoResult;

use client::output::{
	Frame,
	Status,
};

use super::{
	Pos,
	Render,
	Screen,
};
use super::Color::Black;


pub struct Renderer {
	screen: Screen,

	x: Pos,
	y: Pos,
}

impl Renderer {
	pub fn new() -> IoResult<Renderer> {
		let screen = match Screen::new(80, 24) {
			Ok(screen) => screen,
			Err(error) => return Err(error),
		};

		Ok(Renderer {
			screen: screen,

			x: 0,
			y: 0,
		})
	}
}

impl Render for Renderer {
	fn render(&mut self, frame: &Frame) -> IoResult<()> {
		self.x = 0;
		self.y = 0;

		self.screen.bold(true);

		try!(self.render_communication(frame));
		try!(self.render_input(frame));
		try!(self.render_context_info(frame));

		try!(self.screen.submit());

		Ok(())
	}
}

impl Renderer {
	fn render_communication(&mut self, frame: &Frame) -> IoResult<()> {
		let screen_width = self.screen.buffer().width();

		try!(write!(
			&mut self.screen.writer(0, self.y, screen_width),
			"YOUR ID"
		));
		self.y += 1;

		try!(write!(
			&mut self.screen.writer(4, self.y, screen_width),
			"{}",
			frame.self_id
		));
		self.y += 2;

		try!(write!(
			&mut self.screen.writer(0, self.y, screen_width),
			"BROADCASTS")
		);
		self.y += 1;

		if frame.broadcasts.len() == 0 {
			try!(write!(
				&mut self.screen.writer(4, self.y, screen_width),
				"none"
			));
			self.y += 1;
		}

		let mut slots = if frame.broadcasts.len() > 5 {
			4
		}
		else {
			frame.broadcasts.len()
		};

		for broadcast in frame.broadcasts.iter() {
			if slots == 0 {
				break;
			}

			try!(write!(
				&mut self.screen.writer(4, self.y, screen_width),
				"{}: {}",
				broadcast.sender, broadcast.message
			));
			self.y += 1;

			slots -= 1;
		}

		if frame.broadcasts.len() > 5 {
			try!(write!(
				&mut self.screen.writer(4, self.y, screen_width),
				"(more)",
			));
			self.y += 1;
		}

		self.y += 1;

		Ok(())
	}

	fn render_input(&mut self, frame: &Frame) -> IoResult<()> {
		let screen_width = self.screen.buffer().width();
		let input_prompt = format!("Enter command: {}", frame.input);

		try!(
			self.screen
				.writer(0, self.y, screen_width)
				.write(input_prompt.as_bytes())
		);

		let cursor_position = input_prompt.len() as Pos;
		self.screen.cursor(cursor_position, self.y);

		if frame.commands.len() == 1 {
			let previous_bold  = self.screen.bold(true);
			let previous_color = self.screen.color(Black);

			let rest_of_command = frame.commands[0][frame.input.len() ..];
			try!(write!(
				&mut self.screen.writer(cursor_position, self.y, screen_width),
				"{}",
				rest_of_command,
			));

			self.screen.bold(previous_bold);
			self.screen.color(previous_color);
		}

		Ok(())
	}

	fn render_context_info(&mut self, frame: &Frame) -> IoResult<()> {
		let screen_width = self.screen.buffer().width();

		let status = match frame.status {
			Status::Notice(ref s) => s.as_slice(),
			Status::Error(ref s)  => s.as_slice(),
			Status::None          => "",
		};

		self.y += 2;
		try!(write!(
			&mut self.screen.writer(0, self.y, screen_width),
			"{}",
			status
		));
		self.y += 2;

		try!(write!(
			&mut self.screen.writer(0, self.y, screen_width),
			"COMMANDS"
		));
		self.y += 1;

		if frame.commands.len() == 0 {
			try!(write!(
				&mut self.screen.writer(4, self.y, screen_width),
				"none"
			));
		}

		self.x = 4;
		for command in frame.commands.iter() {
			try!(write!(
				&mut self.screen.writer(self.x, self.y, 15),
				"{}",
				command
			));
			self.x += 4 + command.len() as Pos;
		}

		self.y += 1;

		Ok(())
	}
}

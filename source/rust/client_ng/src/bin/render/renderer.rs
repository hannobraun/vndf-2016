use std::io::IoResult;

use client::render::{
	Frame,
	Status,
};

use super::{
	Pos,
	Render,
	Screen,
};
use super::Color::Black;
use super::util::Section;


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
		// TODO: Color static UI elements blue

		self.x = 0;
		self.y = 0;

		self.screen.buffer().bold(true);

		// TODO: Those shouldn't be methods on Renderer but rather on separate
		//       structs. The calls should return the height of the section.
		try!(self.render_communication(frame));
		try!(self.render_input(frame));
		try!(self.render_context_info(frame));

		try!(self.screen.submit());

		Ok(())
	}
}

impl Renderer {
	fn render_communication(&mut self, frame: &Frame) -> IoResult<()> {
		let     width   = self.screen.buffer().width();
		// TODO: Reuse section
		let mut section = Section::new(width, 12);

		section.buffer.bold(true);

		let y = self.y;

		try!(write!(
			&mut section.buffer.writer(0, y + 0, width),
			"YOUR ID"
		));

		try!(write!(
			&mut section.buffer.writer(4, y + 1, width),
			"{}",
			frame.self_id
		));

		try!(write!(
			&mut section.buffer.writer(0, y + 3, width),
			"BROADCASTS")
		);

		if frame.broadcasts.len() == 0 {
			try!(write!(
				&mut section.buffer.writer(4, y + 4, width),
				"none"
			));
		}

		let mut slots = if frame.broadcasts.len() > 5 {
			4
		}
		else {
			frame.broadcasts.len()
		};

		for (i, broadcast) in frame.broadcasts.iter().enumerate() {
			if slots == 0 {
				break;
			}

			try!(write!(
				&mut section.buffer.writer(4, y + 4 + i as Pos, width),
				"{}: {}",
				broadcast.sender, broadcast.message
			));

			slots -= 1;
		}

		if frame.broadcasts.len() > 5 {
			try!(write!(
				&mut section.buffer.writer(4, y + 4 + 4, width),
				"(more)",
			));
		}

		self.y += section.height;

		try!(section.write(0, 0, &mut self.screen));

		Ok(())
	}

	fn render_input(&mut self, frame: &Frame) -> IoResult<()> {
		let     screen_width = self.screen.buffer().width();
		// TODO: Reuse section
		let mut section      = Section::new(screen_width, 4);

		section.buffer.bold(true);

		try!(write!(
			&mut section.buffer.writer(0, 0, screen_width),
			"ENTER COMMAND",
		));

		try!(write!(
			&mut section.buffer.writer(4, 1, screen_width),
			"{}",
			frame.input,
		));

		let cursor_position = 1 + 4 + frame.input.len() as Pos;
		self.screen.cursor(cursor_position, self.y + 2);

		if frame.commands.len() == 1 {
			section.buffer.bold(true);
			section.buffer.color(Black);

			let rest_of_command = frame.commands[0][frame.input.len() ..];
			try!(write!(
				&mut section.buffer.writer(cursor_position, 1, screen_width),
				"{}",
				rest_of_command,
			));
		}

		try!(section.write(0, self.y, &mut self.screen));

		self.y += section.height;

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
			&mut self.screen.buffer().writer(0, self.y, screen_width),
			"{}",
			status
		));
		self.y += 2;

		try!(write!(
			&mut self.screen.buffer().writer(0, self.y, screen_width),
			"COMMANDS"
		));
		self.y += 1;

		if frame.commands.len() == 0 {
			try!(write!(
				&mut self.screen.buffer().writer(4, self.y, screen_width),
				"none"
			));
		}

		self.x = 4;
		for command in frame.commands.iter() {
			try!(write!(
				&mut self.screen.buffer().writer(self.x, self.y, self.x + 15),
				"{}",
				command
			));
			self.x += 4 + command.len() as Pos;
		}

		self.y += 1;

		Ok(())
	}
}

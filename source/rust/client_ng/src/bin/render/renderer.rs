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
use super::buffer::ScreenBuffer;
use super::Color::{
	Black,
	White,
};
use super::util::Section;


pub struct Renderer {
	screen: Screen,

	comm : Section,
	input: Section,
	info : Section,
}

impl Renderer {
	pub fn new() -> IoResult<Renderer> {
		let width = 80;

		let screen = match Screen::new(width, 24) {
			Ok(screen) => screen,
			Err(error) => return Err(error),
		};

		Ok(Renderer {
			screen: screen,

			comm : Section::new(width, 14),
			input: Section::new(width,  4),
			info : Section::new(width,  6),
		})
	}
}

impl Render for Renderer {
	fn render(&mut self, frame: &Frame) -> IoResult<()> {
		// TODO: Color static UI elements blue

		let mut y = 0;

		try!(self.render_comm(frame, &mut y));
		try!(self.render_input(frame, &mut y));
		try!(self.render_info(frame, &mut y));

		try!(self.screen.submit());

		Ok(())
	}
}

impl Renderer {
	fn render_comm(&mut self, frame: &Frame, y: &mut Pos) -> IoResult<()> {
		let width = self.screen.buffer().width();

		self.comm.buffer.clear();
		self.comm.buffer.bold(true);

		try!(write!(
			&mut self.comm.buffer.writer(0, 0, width),
			"YOUR ID"
		));

		try!(write!(
			&mut self.comm.buffer.writer(4, 1, width),
			"{}",
			frame.self_id
		));

		try!(write!(
			&mut self.comm.buffer.writer(0, 3, width),
			"SENDING"
		));

		let is_sending = frame.broadcasts.iter().any(|broadcast|
			broadcast.sender == frame.self_id
		);

		if is_sending {
			// TODO: Display broadcast
			// TODO: Display button to stop sending
		}
		else {
			try!(button(
				&mut self.comm.buffer,
				4, 4,
				"Send Broadcast",
			));
		}

		try!(write!(
			&mut self.comm.buffer.writer(0, 6, width),
			"RECEIVING",
		));

		if frame.broadcasts.len() == 0 {
			try!(write!(
				&mut self.comm.buffer.writer(4, 7, width),
				"none"
			));
		}

		let mut slots = if frame.broadcasts.len() > 5 {
			4
		}
		else {
			frame.broadcasts.len()
		};

		let mut broadcasts = frame.broadcasts.clone();
		broadcasts.sort_by(|a, b| a.sender.cmp(&b.sender));
		for (i, broadcast) in broadcasts.iter().enumerate() {
			if slots == 0 {
				break;
			}

			try!(write!(
				&mut self.comm.buffer.writer(4, 7 + i as Pos, width),
				"{}: {}",
				broadcast.sender, broadcast.message
			));

			slots -= 1;
		}

		if frame.broadcasts.len() > 5 {
			try!(write!(
				&mut self.comm.buffer.writer(4, 7 + 4, width),
				"(more)",
			));
		}

		try!(self.comm.write(0, *y, &mut self.screen));
		*y += self.comm.height;

		Ok(())
	}

	fn render_input(&mut self, frame: &Frame, y: &mut Pos) -> IoResult<()> {
		let width = self.screen.buffer().width();

		self.input.buffer.clear();
		self.input.buffer.bold(true);

		try!(write!(
			&mut self.input.buffer.writer(0, 0, width),
			"ENTER COMMAND",
		));

		try!(write!(
			&mut self.input.buffer.writer(4, 1, width),
			"{}",
			frame.input,
		));

		let cursor_position = 1 + 4 + frame.input.len() as Pos;
		self.screen.cursor(cursor_position, *y + 2);

		if frame.commands.len() == 1 {
			self.input.buffer.foreground_color(Black);

			let rest_of_command = frame.commands[0][frame.input.len() ..];
			try!(write!(
				&mut self.input.buffer.writer(cursor_position - 1, 1, width),
				"{}",
				rest_of_command,
			));
		}

		try!(self.input.write(0, *y, &mut self.screen));
		*y += self.input.height;

		Ok(())
	}

	fn render_info(&mut self, frame: &Frame, y: &mut Pos) -> IoResult<()> {
		let width = self.screen.buffer().width();

		self.info.buffer.clear();
		self.info.buffer.bold(true);

		let status = match frame.status {
			Status::Notice(ref s) => s.as_slice(),
			Status::Error(ref s)  => s.as_slice(),
			Status::None          => "",
		};

		try!(write!(
			&mut self.info.buffer.writer(0, 0, width),
			"{}",
			status
		));

		try!(write!(
			&mut self.info.buffer.writer(0, 2, width),
			"COMMANDS"
		));

		if frame.commands.len() == 0 {
			try!(write!(
				&mut self.info.buffer.writer(4, 3, width),
				"none"
			));
		}

		let mut x = 4;
		for command in frame.commands.iter() {
			try!(write!(
				&mut self.info.buffer.writer(x, 3, x + 15),
				"{}",
				command
			));
			x += 4 + command.len() as Pos;
		}

		try!(self.info.write(0, *y, &mut self.screen));
		*y += self.info.buffer.height();

		Ok(())
	}
}


fn button(b: &mut ScreenBuffer, x: Pos, y: Pos, text: &str) -> IoResult<()> {
	let width = b.width();

	let foreground_color = b.foreground_color(Black);
	let background_color = b.background_color(Some(White));

	try!(b.writer(x, y, width).write_str(text));

	b.foreground_color(foreground_color);
	b.background_color(background_color);

	Ok(())
}

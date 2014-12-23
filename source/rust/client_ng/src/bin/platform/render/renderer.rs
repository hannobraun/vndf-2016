use std::io::IoResult;

use client::platform::{
	Frame,
	Status,
};
use platform::ui::Ui;

use super::{
	Pos,
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
			info : Section::new(width,  6),
		})
	}

	pub fn render(&mut self, frame: &Frame, ui: &Ui) -> IoResult<()> {
		let mut y = 0;

		try!(self.render_comm(frame, ui, &mut y));
		try!(self.render_info(frame, &mut y));

		try!(self.screen.submit());

		Ok(())
	}

	fn render_comm(
		&mut self,
		frame: &Frame,
		ui   : &Ui,
		y    : &mut Pos
	) -> IoResult<()> {
		self.comm.buffer.clear();

		try!(write!(
			&mut self.comm.buffer.writer(0, 0),
			"YOUR ID"
		));

		try!(write!(
			&mut self.comm.buffer.writer(4, 1),
			"{}",
			frame.self_id
		));

		try!(write!(
			&mut self.comm.buffer.writer(0, 3),
			"SENDING"
		));

		let message = ui.input_text.as_slice();
		let button_text = if ui.input_active {
			"Send Broadcast"
		}
		else {
			"Stop Sending"
		};

		let width           = self.comm.buffer.width() - 4;
		let button_width    = button_text.len() as Pos;
		// TODO: The text input width should stay the same, regardless of which
		//       button is displayed next to it.
		let broadcast_width = width - 2 - button_width - 2;

		try!(text_input(
			&mut self.comm.buffer,
			4, 4, broadcast_width,
			message,
		));

		try!(button(
			&mut self.comm.buffer,
			4 + broadcast_width + 2, 4,
			button_text,
		));

		try!(write!(
			&mut self.comm.buffer.writer(0, 6),
			"RECEIVING",
		));

		if frame.broadcasts.len() == 0 {
			try!(write!(
				&mut self.comm.buffer.writer(4, 7),
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
				&mut self.comm.buffer.writer(4, 7 + i as Pos),
				"{}: {}",
				broadcast.sender, broadcast.message
			));

			slots -= 1;
		}

		if frame.broadcasts.len() > 5 {
			try!(write!(
				&mut self.comm.buffer.writer(4, 7 + 4),
				"(more)",
			));
		}

		try!(self.comm.write(0, *y, &mut self.screen));
		*y += self.comm.height;

		Ok(())
	}

	fn render_info(&mut self, frame: &Frame, y: &mut Pos) -> IoResult<()> {
		self.info.buffer.clear();

		let status = match frame.status {
			Status::Notice(ref s) => s.as_slice(),
			Status::Error(ref s)  => s.as_slice(),
			Status::None          => "",
		};

		try!(write!(
			&mut self.info.buffer.writer(0, 0),
			"{}",
			status
		));

		try!(write!(
			&mut self.info.buffer.writer(0, 2),
			"COMMANDS"
		));

		if frame.commands.len() == 0 {
			try!(write!(
				&mut self.info.buffer.writer(4, 3),
				"none"
			));
		}

		let mut x = 4;
		for command in frame.commands.iter() {
			try!(write!(
				&mut self.info.buffer.writer(x, 3).limit(x + 15),
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


fn text_input(b: &mut ScreenBuffer, x: Pos, y: Pos, width: Pos, text: &str) -> IoResult<()> {
	let limit = x + width;

	try!(
		b
			.writer(x, y)
			.limit(limit)
			.foreground_color(White)
			.background_color(Black)
			.write_str(text)
	);
	for x in range(x + text.len() as u16, limit) {
		try!(
			b
				.writer(x, y)
				.limit(limit)
				.foreground_color(White)
				.background_color(Black)
				.write_str(" ")
		);
	}

	Ok(())
}

fn button(b: &mut ScreenBuffer, x: Pos, y: Pos, text: &str) -> IoResult<()> {
	b
		.writer(x, y)
		.foreground_color(Black)
		.background_color(White)
		.write_str(text)
}

use std::cmp::min;
use std::io::IoResult;

use client::platform::{
	Frame,
	Status,
};
use render::{
	Pos,
	Screen,
	ScreenBuffer,
	Section,
};
use render::Color::{
	Green,
	Red,
};
use ui::Ui;
use ui::render::{
	Render,
	RenderBroadcastForm,
};


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

			comm : Section::new(width, 18),
			info : Section::new(width,  6),
		})
	}

	pub fn render(&mut self, frame: &Frame, ui: &Ui) -> IoResult<()> {
		let mut y = 0;

		self.screen.cursor(None);

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


		try!(RenderBroadcastForm.render(
			&mut self.comm.buffer,
			4, 4,
			&ui.broadcast_form,
			&!ui.element_active,
		));

		try!(write!(
			&mut self.comm.buffer.writer(0, 6),
			"RECEIVING",
		));

		let mut broadcasts: Vec<String> = frame.broadcasts
			.iter()
			.map(|broadcast|
				format!("{}: {}", broadcast.sender, broadcast.message)
			)
			.collect();
		broadcasts.sort();

		let width = self.comm.buffer.width();
		try!(list(
			&mut self.comm.buffer,
			4, 7,
			width - 4 - 4, 5,
			broadcasts.as_slice(),
			ui.broadcast_list.first,
		));

		try!(self.comm.write(0, *y, &mut self.screen));
		*y += self.comm.height;

		Ok(())
	}

	fn render_info(&mut self, frame: &Frame, y: &mut Pos) -> IoResult<()> {
		self.info.buffer.clear();

		{
			let status_writer = self.info.buffer.writer(0, 0);

			let (mut status_writer, status) = match frame.status {
				Status::Notice(ref s) =>
					(status_writer.foreground_color(Green), s.as_slice()),
				Status::Error(ref s) =>
					(status_writer.foreground_color(Red), s.as_slice()),
				Status::None =>
					(status_writer, ""),
			};

			try!(write!(
				&mut status_writer,
				"{}",
				status
			));
		}

		try!(self.info.write(0, *y, &mut self.screen));
		*y += self.info.buffer.height();

		Ok(())
	}
}


fn list(
	b     : &mut ScreenBuffer,
	x     : Pos,
	y     : Pos,
	width : Pos,
	height: Pos,
	items : &[String],
	first : usize,
) -> IoResult<()> {
	let limit = x + width;

	if items.len() == 0 {
		try!(write!(
			&mut b.writer(x, y).limit(limit),
			"none"
		));

		return Ok(());
	}

	let mut n = min(items.len(), height as usize);

	let mut iter = items
		.iter()
		.skip(first)
		.enumerate();

	for (i, item) in iter {
		if n == 0 {
			break;
		}

		try!(
			b
				.writer(x, y + i as Pos)
				.limit(limit)
				.write_str(item.as_slice())
		);

		n -= 1;
	}

	if first > 0 {
		try!(write!(
			&mut b.writer(limit - 1, y).limit(limit),
			"↑",
		));
	}

	if items.len() - first > height as usize {
		try!(write!(
			&mut b.writer(limit - 1, y + height - 1).limit(limit),
			"↓",
		));
	}

	Ok(())
}

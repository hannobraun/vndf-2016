use std::old_io::IoResult;

use client::platform::{
	Frame,
	Status,
};
use render::{
	Pos,
	Section,
};
use render::Color::{
	Green,
	Red,
};
use ui::Ui;
use ui::base::Render;
use ui::render;


// TODO: Merge into Ui
pub struct Renderer {
	main: Section,
	info: Section,
}

impl Renderer {
	pub fn new() -> IoResult<Renderer> {
		let width = 80;

		Ok(Renderer {

			main: Section::new(width, 18),
			info: Section::new(width,  6),
		})
	}

	pub fn render(&mut self, frame: &Frame, ui: &mut Ui) -> IoResult<()> {
		let mut y = 0;

		ui.screen.cursor(None);

		try!(self.render_main(frame, ui, &mut y));
		try!(self.render_info(frame, ui, &mut y));

		try!(ui.screen.submit());

		Ok(())
	}

	fn render_main(
		&mut self,
		frame: &Frame,
		ui   : &mut Ui,
		y    : &mut Pos
	) -> IoResult<()> {
		self.main.buffer.clear();

		let mut broadcasts: Vec<String> = frame.broadcasts
			.iter()
			.map(|broadcast|
				format!("{}: {}", broadcast.sender, broadcast.message)
			)
			.collect();
		broadcasts.sort();

		try!(ui.tab_switcher.render(
			&mut self.main.buffer,
			0, 0,
			&render::TabSwitcherArgs {
				self_id    : frame.self_id.as_slice(),
				broadcasts : broadcasts.as_slice(),
				list_height: ui.broadcast_list_height,
			},
		));

		try!(self.main.write(0, *y, &mut ui.screen));
		*y += self.main.height;

		Ok(())
	}

	fn render_info(&mut self,frame: &Frame, ui: &mut Ui, y: &mut Pos)
		-> IoResult<()>
	{
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

		try!(self.info.write(0, *y, &mut ui.screen));
		*y += self.info.buffer.height();

		Ok(())
	}
}

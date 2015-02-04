use std::cmp::max;
use std::old_io::IoResult;

use client::platform::{
	Frame,
	Status,
};
use render::{
	Pos,
	Screen,
	Section,
};
use render::Color::{
	Green,
	Red,
};
use ui::Ui;
use ui::base::Render;
use ui::base::Status::{
	Active,
	Passive,
	Selected,
};
use ui::render;


const START_BROADCAST: &'static str = "Send Broadcast";
const STOP_BROADCAST : &'static str = "Stop Sending";


// TODO: Merge into Ui
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

	pub fn render(&mut self, frame: &Frame, ui: &mut Ui) -> IoResult<()> {
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
		ui   : &mut Ui,
		y    : &mut Pos
	) -> IoResult<()> {
		self.comm.buffer.clear();

		let mut broadcasts: Vec<String> = frame.broadcasts
			.iter()
			.map(|broadcast|
				format!("{}: {}", broadcast.sender, broadcast.message)
			)
			.collect();
		broadcasts.sort();

		let is_sending = frame.broadcasts
			.iter()
			.any(|broadcast|
				broadcast.sender == frame.self_id
			);

		// TODO: Move into update method on BroadcastForm
		ui.comm_tab.broadcast_form.text_field_status =
			if ui.comm_tab.form_is_selected() {
				if is_sending {
					Selected
				}
				else {
					Active
				}
			}
			else {
				Passive
			};

		let broadcast_button_status = if ui.comm_tab.form_is_selected() {
			Active
		}
		else {
			Passive
		};

		let broadcast_button_text = if is_sending {
			STOP_BROADCAST
		}
		else {
			START_BROADCAST
		};

		let broadcast_button_width =
			max(
				START_BROADCAST.chars().count(),
				STOP_BROADCAST.chars().count()
			)
			as Pos;

		try!(ui.comm_tab.render(
			&mut self.comm.buffer,
			0, 0,
			&render::CommTabArgs {
				self_id                : frame.self_id.as_slice(),
				broadcasts             : broadcasts.as_slice(),
				is_sending             : is_sending,
				broadcast_button_status: broadcast_button_status,
				broadcast_button_text  : broadcast_button_text,
				broadcast_button_width : broadcast_button_width,
			},
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

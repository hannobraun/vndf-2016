use std::cmp::{
	max,
	min,
};
use std::old_io::IoResult;

use client::platform::Message;
use render::{
	Pos,
	ScreenBuffer,
};

use super::base::{
	Status,
	Update,
};
use super::render;
use super::state::{
	BroadcastForm,
	Button,
	CommTab,
	InfoSection,
	List,
	MainSection,
	TabHeader,
	TabSwitcher,
	TextField,
};


const START_BROADCAST: &'static str = "Send Broadcast";
const STOP_BROADCAST : &'static str = "Stop Sending";

pub struct BroadcastFormArgs {
	pub is_selected: bool,
	pub is_sending : bool,
}

impl Update for BroadcastForm {
	type Args = BroadcastFormArgs;

	fn update(&mut self, buffer: &mut ScreenBuffer, x: Pos, y: Pos, args: &BroadcastFormArgs) -> IoResult<()> {
		self.text_field_status =
			if args.is_selected {
				if args.is_sending {
					Status::Selected
				}
				else {
					Status::Active
				}
			}
			else {
				Status::Passive
			};

		self.button_status =
			if args.is_selected {
				Status::Active
			}
			else {
				Status::Passive
			};

		self.button_text =
			if args.is_sending {
				STOP_BROADCAST
			}
			else {
				START_BROADCAST
			};

		self.button_width =
			max(
				START_BROADCAST.chars().count(),
				STOP_BROADCAST.chars().count()
			)
			as Pos;

		let total_width      = buffer.width() - x;
		let text_field_width = total_width - 2 - self.button_width - 2;

		try!(self.text_field.update(
			buffer,
			x, y,
			&TextFieldArgs {
				width : text_field_width,
				status: self.text_field_status,
			},
		));

		self.button.update(
			buffer,
			x + text_field_width + 2, y,
			&ButtonArgs {
				text  : self.button_text,
				status: self.button_status,
			},
		)
	}
}


pub struct ButtonArgs<'a> {
	pub text  : &'a str,
	pub status: Status,
}

impl<'a> Update for Button {
	type Args = ButtonArgs<'a>;

	fn update(&mut self, buffer: &mut ScreenBuffer, x: Pos, y: Pos, args: &ButtonArgs) -> IoResult<()> {
		render::button(buffer, x, y, args.status, args.text)
	}
}


pub struct CommTabArgs<'a> {
	pub is_sending: bool,
	pub self_id   : &'a str,
	pub broadcasts: &'a [String],
}

impl<'a> Update for CommTab {
	type Args = CommTabArgs<'a>;

	fn update(&mut self, buffer: &mut ScreenBuffer, x: Pos, y: Pos, args: &CommTabArgs) -> IoResult<()> {
		try!(write!(
			&mut buffer.writer(x, y),
			"YOUR ID",
		));

		try!(write!(
			&mut buffer.writer(x + 4, y + 1),
			"{}",
			args.self_id,
		));

		try!(write!(
			&mut buffer.writer(x, y + 3),
			"SENDING",
		));

		try!(write!(
			&mut buffer.writer(x, y + 6),
			"RECEIVING",
		));

		let form_is_selected = self.form_is_selected();
		let list_is_selected = self.list_is_selected();

		try!(self.broadcast_form.update(
			buffer,
			x + 4, y + 4,
			&BroadcastFormArgs {
				is_selected: form_is_selected,
				is_sending : args.is_sending,
			},
		));

		let width = buffer.width();

		self.broadcast_list.update(
			buffer,
			x + 4, y + 7,
			&ListArgs {
				is_selected: list_is_selected,
				width      : width - 4 - 4,
				height     : 5,
				items      : args.broadcasts,
			},
		)
	}
}


impl Update for InfoSection {
	type Args = Message;

	fn update(&mut self, _: &mut ScreenBuffer, _: Pos, _: Pos, _: &Message) -> IoResult<()> {
		Ok(())
	}
}


pub struct ListArgs<'a> {
	pub is_selected: bool,
	pub width      : Pos,
	pub height     : Pos,
	pub items      : &'a [String],
}

impl<'a> Update for List {
	type Args = ListArgs<'a>;

	fn update(&mut self, buffer: &mut ScreenBuffer, x: Pos, y: Pos, args: &ListArgs) -> IoResult<()> {
		self.status = if args.is_selected {
			if self.activated {
				Status::Active
			}
			else {
				Status::Selected
			}
		}
		else {
			Status::Passive
		};

		let max_first = max(0, args.items.len() as isize - args.height as isize);
		self.first = min(self.first, max_first as usize);

		render::list(buffer, x, y, self.status, args.width, args.height, self.first, args.items)
	}
}


pub struct MainArgs<'a> {
	pub is_sending: bool,
	pub self_id   : &'a str,
	pub broadcasts: &'a [String],
}

impl<'a> Update for MainSection {
	type Args = MainArgs<'a>;

	fn update(&mut self, b: &mut ScreenBuffer, x: Pos, y: Pos, args: &MainArgs) -> IoResult<()> {
		self.tab_switcher.update(
			b,
			x + 1, y + 1,
			&TabSwitcherArgs {
				is_sending: args.is_sending,
				self_id   : args.self_id,
				broadcasts: args.broadcasts,
			},
		)
	}
}


pub struct TabHeaderArgs<'a> {
	pub label: &'a str,
}

impl<'a> Update for TabHeader {
	type Args = TabHeaderArgs<'a>;

	fn update(&mut self, buffer: &mut ScreenBuffer, x: Pos, y: Pos, args: &TabHeaderArgs) -> IoResult<()> {
		render::tab_header(buffer, x, y, self.status, args.label)
	}
}


pub struct TabSwitcherArgs<'a> {
	pub is_sending: bool,
	pub self_id   : &'a str,
	pub broadcasts: &'a [String],
}

impl<'a> Update for TabSwitcher {
	type Args = TabSwitcherArgs<'a>;

	fn update(&mut self, buffer: &mut ScreenBuffer, x: Pos, y: Pos, args: &TabSwitcherArgs) -> IoResult<()> {
		// TODO: Set currently selected TabHeader to active.

		let mut headers = [
			("Comm", &mut self.comm_header),
			("Nav" , &mut self.nav_header ),
		];
		let numer_of_headers = headers.len();
		let mut header_x = x;
		for (i, &mut (label, ref mut header)) in headers.iter_mut().enumerate() {
			try!(header.update(
				buffer,
				header_x,
				y,
				&TabHeaderArgs {
					label: label,
				},
			));
			header_x += label.chars().count() as Pos;

			if i + 1 < numer_of_headers {
				try!(
					buffer
						.writer(header_x, y)
						.write_str(" | ")
				);
				header_x += 3;
			}
		}

		self.comm_tab.update(
			buffer,
			x, y + 2,
			&CommTabArgs {
				is_sending: args.is_sending,
				self_id   : args.self_id,
				broadcasts: args.broadcasts,
			},
		)
	}
}


pub struct TextFieldArgs {
	pub width : Pos,
	pub status: Status,
}

impl Update for TextField {
	type Args = TextFieldArgs;

	fn update(&mut self, buffer: &mut ScreenBuffer, x: Pos, y: Pos, args: &TextFieldArgs) -> IoResult<()> {
		render::text_field(buffer, x, y, args.status, args.width, self.text.as_slice())
	}
}

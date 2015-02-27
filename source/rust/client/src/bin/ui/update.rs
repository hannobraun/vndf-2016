use std::cmp::{
	max,
	min,
};
use std::old_io::IoResult;

use nalgebra::Vec2;

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
	NavTab,
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

		let button_status =
			if args.is_selected {
				Status::Active
			}
			else {
				Status::Passive
			};

		let button_text =
			if args.is_sending {
				STOP_BROADCAST
			}
			else {
				START_BROADCAST
			};

		let button_width =
			max(
				START_BROADCAST.chars().count(),
				STOP_BROADCAST.chars().count()
			)
			as Pos;

		let total_width      = buffer.width() - x;
		let text_field_width = total_width - 2 - button_width - 2;

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
				text  : button_text,
				status: button_status,
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

	fn update(&mut self, buffer: &mut ScreenBuffer, x: Pos, y: Pos, message: &Message) -> IoResult<()> {
		render::info_section(buffer, x, y, self.width, self.height, message)
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
		let status = if args.is_selected {
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

		render::list(buffer, x, y, status, args.width, args.height, self.first, args.items)
	}
}


pub struct MainArgs<'a> {
	pub is_sending: bool,
	pub self_id   : &'a str,
	pub broadcasts: &'a [String],
}

impl<'a> Update for MainSection {
	type Args = MainArgs<'a>;

	fn update(&mut self, buffer: &mut ScreenBuffer, x: Pos, y: Pos, args: &MainArgs) -> IoResult<()> {
		try!(render::main_section(buffer, x, y, self.width, self.height));
		self.tab_switcher.update(
			buffer,
			x + 1, y + 1,
			&TabSwitcherArgs {
				is_sending: args.is_sending,
				self_id   : args.self_id,
				broadcasts: args.broadcasts,
				position  : Vec2::new(0.0, 0.0),
				velocity  : Vec2::new(0.0, 0.0),
			},
		)
	}
}


pub struct NavTabArgs {
	pub position: Vec2<f32>,
	pub velocity: Vec2<f32>,
}

impl Update for NavTab {
	type Args = NavTabArgs;

	fn update(&mut self, buffer: &mut ScreenBuffer, x: Pos, y: Pos, args: &NavTabArgs) -> IoResult<()> {
		try!(write!(
			&mut buffer.writer(x, y),
			"Position: ({}, {})",
			args.position.x, args.position.y,
		));
		try!(write!(
			&mut buffer.writer(x, y + 1),
			"Velocity: ({}, {})",
			args.velocity.x, args.velocity.y,
		));

		Ok(())
	}
}


pub struct TabHeaderArgs<'a> {
	pub label : &'a str,
	pub status: Status,
}

impl<'a> Update for TabHeader {
	type Args = TabHeaderArgs<'a>;

	fn update(&mut self, buffer: &mut ScreenBuffer, x: Pos, y: Pos, args: &TabHeaderArgs) -> IoResult<()> {
		render::tab_header(buffer, x, y, args.status, args.label)
	}
}


pub struct TabSwitcherArgs<'a> {
	pub is_sending: bool,
	pub self_id   : &'a str,
	pub broadcasts: &'a [String],
	pub position  : Vec2<f32>,
	pub velocity  : Vec2<f32>,
}

impl<'a> Update for TabSwitcher {
	type Args = TabSwitcherArgs<'a>;

	fn update(&mut self, buffer: &mut ScreenBuffer, x: Pos, y: Pos, args: &TabSwitcherArgs) -> IoResult<()> {
		{
			let headers = [
				("Comm", self.active_index % 2 == 0),
				("Nav" , self.active_index % 2 == 1),
			];
			let mut header_x = x;
			for (i, &(label, is_active)) in headers.iter().enumerate() {
				let status = if is_active {
					Status::Active
				}
				else {
					Status::Passive
				};

				try!(TabHeader.update(
					buffer,
					header_x,
					y,
					&TabHeaderArgs {
						label : label,
						status: status,
					},
				));
				header_x += label.chars().count() as Pos;

				if i + 1 < headers.len() {
					try!(
						buffer
							.writer(header_x, y)
							.write_str(" | ")
					);
					header_x += 3;
				}
			}
		}

		try!(render::tab_switcher(buffer, x, y));

		if self.comm_tab_is_active() {
			try!(self.comm_tab.update(
				buffer,
				x, y + 2,
				&CommTabArgs {
					is_sending: args.is_sending,
					self_id   : args.self_id,
					broadcasts: args.broadcasts,
				},
			));
		}

		if self.nav_tab_is_active() {
			try!(self.nav_tab.update(
				buffer,
				x, y + 2,
				&NavTabArgs {
					position: args.position,
					velocity: args.velocity,
				},
			));
		}

		Ok(())
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

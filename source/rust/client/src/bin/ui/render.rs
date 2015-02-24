use std::old_io::IoResult;

use client::platform::Status as InfoStatus;
use common::game::Broadcast;
use render::{
	draw_border,
	Pos,
	ScreenBuffer,
};
use render::C;
use render::Color::{
	Green,
	Red,
};

use super::base::{
	Render,
	Status,
};
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
use super::update::ButtonArgs;


impl Render for BroadcastForm {
	type Args = ();

	fn render(
		&self,
		buffer: &mut ScreenBuffer,
		x     : Pos,
		y     : Pos,
		_     : &(),
	)
		-> IoResult<()>
	{
		let total_width      = buffer.width() - x;
		let text_field_width = total_width - 2 - self.button_width - 2;

		try!(self.text_field.render(
			buffer,
			x, y,
			&TextFieldArgs {
				width : text_field_width,
				status: self.text_field_status,
			},
		));

		Ok(())
	}
}


impl<'a> Render for Button {
	type Args = ButtonArgs<'a>;

	fn render(
		&self,
		buffer: &mut ScreenBuffer,
		x     : Pos,
		y     : Pos,
		args  : &ButtonArgs,
	)
		-> IoResult<()>
	{
		let (foreground_color, background_color) = args.status.colors();

		buffer
			.writer(x, y)
			.foreground_color(foreground_color)
			.background_color(background_color)
			.write_str(args.text)
	}
}


pub struct CommTabArgs<'a> {
	pub self_id    : &'a str,
	pub broadcasts : &'a [String],
	pub list_height: Pos,
}

impl<'a> Render for CommTab {
	type Args = CommTabArgs<'a>;

	fn render(
		&self,
		buffer : &mut ScreenBuffer,
		x      : Pos,
		y      : Pos,
		args   : &CommTabArgs,
	)
		-> IoResult<()>
	{
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

		try!(self.broadcast_form.render(
			buffer,
			x + 4, y + 4,
			&(),
		));

		try!(write!(
			&mut buffer.writer(x, y + 6),
			"RECEIVING",
		));

		let width = buffer.width();
		try!(self.broadcast_list.render(
			buffer,
			x + 4, y + 7,
			&ListArgs {
				width : width - 4 - 4,
				height: args.list_height,
				items : args.broadcasts,
			},
		));

		Ok(())
	}
}


impl Render for InfoSection {
	type Args = InfoStatus;

	fn render(
		&self,
		buffer: &mut ScreenBuffer,
		x     : Pos,
		y     : Pos,
		status: &InfoStatus,
	)
		-> IoResult<()>
	{
		try!(draw_border(
			buffer,
			x, y,
			self.width,
			self.height
		));

		{
			let status_writer = buffer.writer(x + 1, y + 1);

			let (mut status_writer, status) = match *status {
				InfoStatus::Notice(ref s) =>
					(status_writer.foreground_color(Green), s.as_slice()),
				InfoStatus::Error(ref s) =>
					(status_writer.foreground_color(Red  ), s.as_slice()),
				InfoStatus::None =>
					(status_writer, ""),
			};

			try!(write!(
				&mut status_writer,
				"{}",
				status
			));
		}

		Ok(())
	}
}


pub struct ListArgs<'a> {
	pub width : Pos,
	pub height: Pos,
	pub items : &'a [String],
}

impl<'a> Render for List {
	type Args = ListArgs<'a>;

	fn render(
		&self,
		buffer: &mut ScreenBuffer,
		x     : Pos,
		y     : Pos,
		args  : &ListArgs,
	)
		-> IoResult<()>
	{
		let limit = x + args.width;

		let (foreground_color, background_color) = self.status.colors();

		let items: Vec<String> = if args.items.len() == 0 {
			vec!["none".to_string()]
		}
		else {
			args.items
				.iter()
				.map(|s| s.clone())
				.collect()
		};

		let mut iter = items
			.iter()
			.skip(self.first);

		for i in range(0, args.height) {
			let item_length = match iter.next() {
				Some(item) => {
					try!(
						buffer
							.writer(x, y + i as Pos)
							.limit(limit)
							.foreground_color(foreground_color)
							.background_color(background_color)
							.write_str(item.as_slice())
					);

					item.chars().count()
				},
				None =>
					0,
			};

			for x in range(x + item_length as Pos, limit - 1) {
				try!(
					buffer
						.writer(x, y + i as Pos)
						.limit(limit)
						.foreground_color(foreground_color)
						.background_color(background_color)
						.write_char(' ')
				);
			}
		}

		if self.first > 0 {
			try!(write!(
				&mut buffer.writer(limit - 1, y).limit(limit),
				"↑",
			));
		}

		if items.len() - self.first > args.height as usize {
			try!(write!(
				&mut buffer.writer(limit - 1, y + args.height - 1).limit(limit),
				"↓",
			));
		}

		Ok(())
	}
}


pub struct MainSectionArgs<'a> {
	pub self_id              : &'a str,
	pub broadcasts           : &'a [Broadcast],
	pub broadcast_list_height: Pos,
}

impl<'a> Render for MainSection {
	type Args = MainSectionArgs<'a>;

	fn render(
		&self,
		buffer: &mut ScreenBuffer,
		x     : Pos,
		y     : Pos,
		args  : &MainSectionArgs,
	)
		-> IoResult<()>
	{
		try!(draw_border(
			buffer,
			x, y,
			self.width,
			self.height,
		));

		let mut broadcasts: Vec<String> = args.broadcasts
			.iter()
			.map(|broadcast|
				format!("{}: {}", broadcast.sender, broadcast.message)
			)
			.collect();
		broadcasts.sort();

		try!(self.tab_switcher.render(
			buffer,
			x + 1, y + 1,
			&TabSwitcherArgs {
				self_id    : args.self_id,
				broadcasts : broadcasts.as_slice(),
				list_height: args.broadcast_list_height,
			},
		));

		Ok(())
	}
}


pub struct TabHeaderArgs<'a> {
	pub label: &'a str,
}

impl<'a> Render for TabHeader {
	type Args = TabHeaderArgs<'a>;

	fn render(
		&self,
		buffer: &mut ScreenBuffer,
		x     : Pos,
		y     : Pos,
		args  : &TabHeaderArgs,
	)
		-> IoResult<()>
	{
		let (foreground_color, background_color) = self.status.colors();

		buffer
			.writer(x, y)
			.foreground_color(foreground_color)
			.background_color(background_color)
			.write_str(args.label)
	}
}


pub struct TabSwitcherArgs<'a> {
	pub self_id    : &'a str,
	pub broadcasts : &'a [String],
	pub list_height: Pos,
}

impl<'a> Render for TabSwitcher {
	type Args = TabSwitcherArgs<'a>;

	fn render(
		&self,
		buffer : &mut ScreenBuffer,
		x      : Pos,
		y      : Pos,
		args   : &TabSwitcherArgs,
	)
		-> IoResult<()>
	{
		let headers = [
			("Comm", &self.comm_header),
			("Nav" , &self.nav_header ),
		];
		let mut header_x = x;
		for (i, &(label, header)) in headers.iter().enumerate() {
			try!(header.render(
				buffer,
				header_x,
				y,
				&TabHeaderArgs {
					label: label,
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

		let mut c = C::new();
		c.c = '─';
		for x in range(x, buffer.width()) {
			try!(buffer.set(x, y + 1, c));
		}

		self.comm_tab.render(
			buffer,
			x,
			y + 2,
			&CommTabArgs {
				self_id    : args.self_id,
				broadcasts : args.broadcasts,
				list_height: args.list_height,
			},
		)
	}
}


pub struct TextFieldArgs {
	pub width : Pos,
	pub status: Status,
}

impl Render for TextField {
	type Args = TextFieldArgs;

	fn render(
		&self,
		buffer : &mut ScreenBuffer,
		x      : Pos,
		y      : Pos,
		args   : &TextFieldArgs,
	)
		-> IoResult<()>
	{
		let text  = self.text.as_slice();
		let limit = x + args.width;

		let (foreground_color, background_color) = args.status.colors();

		try!(
			buffer
				.writer(x, y)
				.limit(limit)
				.foreground_color(foreground_color)
				.background_color(background_color)
				.write_str(text)
		);
		for x in range(x + text.chars().count() as Pos, limit) {
			try!(
				buffer
					.writer(x, y)
					.limit(limit)
					.foreground_color(foreground_color)
					.background_color(background_color)
					.write_str(" ")
			);
		}

		buffer.cursor = if args.status == Status::Active {
			Some((1 + x + text.chars().count() as Pos, 1 + y))
		}
		else {
			None
		};

		Ok(())
	}
}

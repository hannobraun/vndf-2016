use std::cmp::max;
use std::old_io::IoResult;

use render::{
	Pos,
	ScreenBuffer,
};
use render::Color::{
	Black,
	White,
	Yellow,
};

use super::data::{
	BroadcastForm,
	Button,
	CommTab,
	List,
	TextField,
};


pub trait Render {
	type Args;

	fn render(&self, b: &mut ScreenBuffer, x: Pos, y: Pos, args: &Self::Args)
		-> IoResult<()>;
}


const START_BROADCAST: &'static str = "Send Broadcast";
const STOP_BROADCAST : &'static str = "Stop Sending";

impl Render for BroadcastForm {
	type Args = bool;

	fn render(
		&self,
		buffer    : &mut ScreenBuffer,
		x         : Pos,
		y         : Pos,
		is_active: &bool,
	)
		-> IoResult<()>
	{
		let button_text = if *is_active {
			START_BROADCAST
		}
		else {
			STOP_BROADCAST
		};

		let width = buffer.width() - x;
		let button_width =
			max(
				START_BROADCAST.chars().count(),
				STOP_BROADCAST.chars().count()
			)
			as Pos;
		let broadcast_width = width - 2 - button_width - 2;

		try!(self.text_field.render(
			buffer,
			x, y,
			&TextFieldData {
				width : broadcast_width,
				active: *is_active,
			},
		));

		try!(self.button.render(
			buffer,
			x + broadcast_width + 2, y,
			&ButtonArgs { text: button_text },
		));

		Ok(())
	}
}


pub struct ButtonArgs<'a> {
	pub text: &'a str,
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
		buffer
			.writer(x, y)
			.foreground_color(Black)
			.background_color(White)
			.write_str(args.text)
	}
}


pub struct CommTabArgs<'a> {
	pub self_id   : &'a str,
	pub broadcasts: &'a [String],
}

impl<'a> Render for CommTab {
	type Args = CommTabArgs<'a>;

	fn render(
		&self,
		buffer : &mut ScreenBuffer,
		x      : Pos,
		y      : Pos,
		data   : &CommTabArgs,
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
			data.self_id,
		));

		try!(write!(
			&mut buffer.writer(x, y + 3),
			"SENDING",
		));


		try!(self.broadcast_form.render(
			buffer,
			x + 4, y + 4,
			&self.element_active,
		));

		try!(write!(
			&mut buffer.writer(x, y + 6),
			"RECEIVING",
		));

		let width = buffer.width();
		try!(self.broadcast_list.render(
			buffer,
			x + 4, y + 7,
			&ListData {
				width : width - 4 - 4,
				height: 5,
				items : data.broadcasts,
			},
		));

		Ok(())
	}
}


pub struct ListData<'a> {
	pub width : Pos,
	pub height: Pos,
	pub items : &'a [String],
}

impl<'a> Render for List {
	type Args = ListData<'a>;

	fn render(
		&self,
		buffer : &mut ScreenBuffer,
		x      : Pos,
		y      : Pos,
		data   : &ListData,
	)
		-> IoResult<()>
	{
		let limit = x + data.width;

		let (foreground_color, background_color) = (White, Black);

		let items: Vec<String> = if data.items.len() == 0 {
			vec!["none".to_string()]
		}
		else {
			data.items
				.iter()
				.map(|s| s.clone())
				.collect()
		};

		let mut iter = items
			.iter()
			.skip(self.first);

		for i in range(0, data.height) {
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

		if items.len() - self.first > data.height as usize {
			try!(write!(
				&mut buffer.writer(limit - 1, y + data.height - 1).limit(limit),
				"↓",
			));
		}

		Ok(())
	}
}


pub struct TextFieldData {
	pub width : Pos,
	pub active: bool,
}

impl Render for TextField {
	type Args = TextFieldData;

	fn render(
		&self,
		buffer : &mut ScreenBuffer,
		x      : Pos,
		y      : Pos,
		data   : &TextFieldData,
	)
		-> IoResult<()>
	{
		let text  = self.text.as_slice();
		let limit = x + data.width;

		let (foreground_color, background_color) = if data.active {
			(Black, Yellow)
		}
		else {
			(White, Black)
		};

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

		buffer.cursor = if data.active {
			Some((1 + x + text.chars().count() as Pos, 1 + y))
		}
		else {
			None
		};

		Ok(())
	}
}

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

pub struct BroadcastFormArgs {
	pub active : bool,
	pub sending: bool,
}

impl Render for BroadcastForm {
	type Args = BroadcastFormArgs;

	fn render(
		&self,
		buffer: &mut ScreenBuffer,
		x     : Pos,
		y     : Pos,
		args  : &BroadcastFormArgs,
	)
		-> IoResult<()>
	{
		let button_text = if args.sending {
			STOP_BROADCAST
		}
		else {
			START_BROADCAST
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
			&TextFieldArgs {
				width : broadcast_width,
				active: args.active && !args.sending,
			},
		));

		try!(self.button.render(
			buffer,
			x + broadcast_width + 2, y,
			&ButtonArgs {
				text  : button_text,
				active: args.active,
			},
		));

		Ok(())
	}
}


pub struct ButtonArgs<'a> {
	pub text  : &'a str,
	pub active: bool,
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
		let (foreground_color, background_color) = if args.active {
			(Black, Yellow)
		}
		else {
			(Black, White)
		};

		buffer
			.writer(x, y)
			.foreground_color(foreground_color)
			.background_color(background_color)
			.write_str(args.text)
	}
}


pub struct CommTabArgs<'a> {
	pub self_id   : &'a str,
	pub broadcasts: &'a [String],
	pub is_sending: bool,
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
			&BroadcastFormArgs {
				active : self.form_is_active(),
				sending: args.is_sending,
			},
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
				height: 5,
				items : args.broadcasts,
			},
		));

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
		buffer : &mut ScreenBuffer,
		x      : Pos,
		y      : Pos,
		args   : &ListArgs,
	)
		-> IoResult<()>
	{
		let limit = x + args.width;

		let (foreground_color, background_color) = (White, Black);

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


pub struct TextFieldArgs {
	pub width : Pos,
	pub active: bool,
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

		let (foreground_color, background_color) = if args.active {
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

		buffer.cursor = if args.active {
			Some((1 + x + text.chars().count() as Pos, 1 + y))
		}
		else {
			None
		};

		Ok(())
	}
}

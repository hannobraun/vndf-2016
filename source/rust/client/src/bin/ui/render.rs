use std::old_io::IoResult;

use render::{
	Pos,
	ScreenBuffer,
};

use super::base::{
	Render,
	Status,
};
use super::state::{
	BroadcastForm,
	Button,
	CommTab,
	List,
	TextField,
};


pub struct BroadcastFormArgs<'a> {
	pub button_text : &'a str,
	pub button_width: Pos,
}

impl<'a> Render for BroadcastForm {
	type Args = BroadcastFormArgs<'a>;

	fn render(
		&self,
		buffer: &mut ScreenBuffer,
		x     : Pos,
		y     : Pos,
		args  : &BroadcastFormArgs,
	)
		-> IoResult<()>
	{
		let total_width      = buffer.width() - x;
		let text_field_width = total_width - 2 - args.button_width - 2;

		try!(self.text_field.render(
			buffer,
			x, y,
			&TextFieldArgs {
				width : text_field_width,
				status: self.text_field_status,
			},
		));

		try!(self.button.render(
			buffer,
			x + text_field_width + 2, y,
			&ButtonArgs {
				text  : args.button_text,
				status: self.button_status,
			},
		));

		Ok(())
	}
}


pub struct ButtonArgs<'a> {
	pub text  : &'a str,
	pub status: Status,
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
	pub self_id               : &'a str,
	pub broadcasts            : &'a [String],
	pub is_sending            : bool,
	pub broadcast_button_text : &'a str,
	pub broadcast_button_width: Pos,
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
				button_text  : args.broadcast_button_text,
				button_width : args.broadcast_button_width,
			},
		));

		try!(write!(
			&mut buffer.writer(x, y + 6),
			"RECEIVING",
		));

		let list_status = if self.list_is_selected() && self.element_active {
			Status::Active
		}
		else if self.list_is_selected() {
			Status::Selected
		}
		else {
			Status::Passive
		};

		let width = buffer.width();
		try!(self.broadcast_list.render(
			buffer,
			x + 4, y + 7,
			&ListArgs {
				width : width - 4 - 4,
				height: 5,
				items : args.broadcasts,
				status: list_status,
			},
		));

		Ok(())
	}
}


pub struct ListArgs<'a> {
	pub width : Pos,
	pub height: Pos,
	pub items : &'a [String],
	pub status: Status,
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

		let (foreground_color, background_color) = args.status.colors();

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

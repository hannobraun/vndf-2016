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


pub trait Render<E, D> {
	fn render(
		&mut self,
		buffer : &mut ScreenBuffer,
		x      : Pos,
		y      : Pos,
		element: &E,
		data   : &D
	)
		-> IoResult<()>;
}


pub struct RenderBroadcastForm;

const START_BROADCAST: &'static str = "Send Broadcast";
const STOP_BROADCAST : &'static str = "Stop Sending";

impl Render<BroadcastForm, bool> for RenderBroadcastForm {
	fn render(
		&mut self,
		buffer    : &mut ScreenBuffer,
		x         : Pos,
		y         : Pos,
		element   : &BroadcastForm,
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

		try!(RenderTextField.render(
			buffer,
			x, y,
			&element.text_field,
			&TextFieldData {
				width : broadcast_width,
				active: *is_active,
			},
		));

		try!(RenderButton.render(
			buffer,
			x + broadcast_width + 2, y,
			&element.button,
			&ButtonData { text: button_text },
		));

		Ok(())
	}
}


pub struct RenderButton;

pub struct ButtonData<'a> {
	pub text: &'a str,
}

impl<'a> Render<Button, ButtonData<'a>> for RenderButton {
	fn render(
		&mut self,
		buffer: &mut ScreenBuffer,
		x     : Pos,
		y     : Pos,
		_     : &Button,
		data  : &ButtonData,
	)
		-> IoResult<()>
	{
		buffer
			.writer(x, y)
			.foreground_color(Black)
			.background_color(White)
			.write_str(data.text)
	}
}


pub struct RenderCommTab;

pub struct CommTabData<'a> {
	pub self_id   : &'a str,
	pub broadcasts: &'a [String],
}

impl<'a> Render<CommTab, CommTabData<'a>> for RenderCommTab {
	fn render(
		&mut self,
		buffer : &mut ScreenBuffer,
		x      : Pos,
		y      : Pos,
		element: &CommTab,
		data   : &CommTabData,
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


		try!(RenderBroadcastForm.render(
			buffer,
			x + 4, y + 4,
			&element.broadcast_form,
			&element.element_active,
		));

		try!(write!(
			&mut buffer.writer(x, y + 6),
			"RECEIVING",
		));

		let width = buffer.width();
		try!(RenderList.render(
			buffer,
			x + 4, y + 7,
			&element.broadcast_list,
			&ListData {
				width : width - 4 - 4,
				height: 5,
				items : data.broadcasts,
			},
		));

		Ok(())
	}
}


pub struct RenderList;

pub struct ListData<'a> {
	pub width : Pos,
	pub height: Pos,
	pub items : &'a [String],
}

impl<'a> Render<List, ListData<'a>> for RenderList {
	fn render(
		&mut self,
		buffer : &mut ScreenBuffer,
		x      : Pos,
		y      : Pos,
		element: &List,
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
			.skip(element.first);

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

		if element.first > 0 {
			try!(write!(
				&mut buffer.writer(limit - 1, y).limit(limit),
				"↑",
			));
		}

		if items.len() - element.first > data.height as usize {
			try!(write!(
				&mut buffer.writer(limit - 1, y + data.height - 1).limit(limit),
				"↓",
			));
		}

		Ok(())
	}
}


pub struct RenderTextField;

pub struct TextFieldData {
	pub width : Pos,
	pub active: bool,
}

impl Render<TextField, TextFieldData> for RenderTextField {
	fn render(
		&mut self,
		buffer : &mut ScreenBuffer,
		x      : Pos,
		y      : Pos,
		element: &TextField,
		data   : &TextFieldData,
	)
		-> IoResult<()>
	{
		let text  = element.text.as_slice();
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

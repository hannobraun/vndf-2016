use std::cmp::max;
use std::io::IoResult;

use render::{
	Pos,
	ScreenBuffer,
};
use render::Color::{
	Black,
	White,
};

use super::data::{
	BroadcastForm,
	Button,
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
		is_sending: &bool,
	)
		-> IoResult<()>
	{
		let button_text = if *is_sending {
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

		try!(RenderTextField.render(
			buffer,
			x, y,
			&element.text_field,
			&TextFieldData {
				width : broadcast_width,
				active: !is_sending,
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

		try!(
			buffer
				.writer(x, y)
				.limit(limit)
				.foreground_color(White)
				.background_color(Black)
				.write_str(text)
		);
		for x in range(x + text.chars().count() as Pos, limit) {
			try!(
				buffer
					.writer(x, y)
					.limit(limit)
					.foreground_color(White)
					.background_color(Black)
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

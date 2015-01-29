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

pub struct TextFieldData<'a> {
	pub width : Pos,
	pub active: bool,
}

impl<'a> Render<TextField, TextFieldData<'a>> for RenderTextField {
	fn render(
		&mut self,
		buffer : &mut ScreenBuffer,
		x      : Pos,
		y      : Pos,
		element: &TextField,
		data   : &TextFieldData
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

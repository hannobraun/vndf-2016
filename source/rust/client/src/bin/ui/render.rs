use std::io::IoResult;

use render::{
	Pos,
	ScreenBuffer,
};
use render::Color::{
	Black,
	White,
};

use super::data::TextField;


pub trait Render<E, D> {
	fn render(&mut self, b: &mut ScreenBuffer, element: &E, data: &D)
		-> IoResult<()>;
}


pub struct RenderTextField;

pub struct TextFieldData<'a> {
	pub x    : Pos,
	pub y    : Pos,
	pub width: Pos,
}

impl<'a> Render<TextField, TextFieldData<'a>> for RenderTextField {
	fn render(
		&mut self,
		buffer : &mut ScreenBuffer,
		element: &TextField,
		data   : &TextFieldData
	)
		-> IoResult<()>
	{
		let text  = element.text.as_slice();
		let limit = data.x + data.width;

		try!(
			buffer
				.writer(data.x, data.y)
				.limit(limit)
				.foreground_color(White)
				.background_color(Black)
				.write_str(text)
		);
		for x in range(data.x + text.chars().count() as u16, limit) {
			try!(
				buffer
					.writer(x, data.y)
					.limit(limit)
					.foreground_color(White)
					.background_color(Black)
					.write_str(" ")
			);
		}

		Ok(())
	}
}

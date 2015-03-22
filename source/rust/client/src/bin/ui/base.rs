use std::io;

use render::{
	Pos,
	ScreenBuffer,
};
use render::Color::{
	self,
	Black,
	Blue,
	White,
	Yellow,
};


pub trait ProcessInput {
	fn process_events(&mut self, events: &[InputEvent]);
}

#[derive(Copy)]
pub enum InputEvent {
	Char(char),

	CursorLeft,
	CursorRight,
	CursorUp,
	CursorDown,

	Backspace,
	Enter,
}


pub trait Update {
	type Args;

	fn update(
		&mut self,
		b   : &mut ScreenBuffer,
		x   : Pos,
		y   : Pos,
		args: &Self::Args
	)
	-> io::Result<()>;
}


#[derive(Copy, Eq, PartialEq)]
pub enum Status {
	Passive,
	Selected,
	Active,
}

impl Status {
	pub fn colors(&self) -> (Color, Color) {
		match *self {
			Status::Passive  => (White, Black ),
			Status::Selected => (White, Blue  ),
			Status::Active   => (Black, Yellow),
		}
	}
}

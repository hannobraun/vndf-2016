use std::old_io::IoResult;

use render::{
	Pos,
	ScreenBuffer,
};


pub enum InputEvent {
	Char(char),

	CursorLeft,
	CursorRight,
	CursorUp,
	CursorDown,

	Backspace,
	Enter,
}


pub trait ProcessInput {
	fn process_event(&mut self, event: InputEvent);
}


pub trait Render {
	type Args;

	fn render(&self, b: &mut ScreenBuffer, x: Pos, y: Pos, args: &Self::Args)
		-> IoResult<()>;
}

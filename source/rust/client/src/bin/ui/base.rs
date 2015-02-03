pub enum InputEvent {
	Char(char),
	CursorLeft,
	CursorRight,
	CursorUp,
	CursorDown,
}


pub trait ProcessInput {
	fn process_char(&mut self, c: char);
	fn process_cursor(&mut self, direction: Direction);
}

pub enum Direction { Up, Down, Right, Left }

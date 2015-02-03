pub enum InputEvent {
	Char(char),
	CursorLeft,
	CursorRight,
	CursorUp,
	CursorDown,
}


pub trait ProcessInput {
	fn process_event(&mut self, event: InputEvent);
}

pub enum Direction { Up, Down, Right, Left }

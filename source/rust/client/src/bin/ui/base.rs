pub enum InputEvent {
	Char(char),

	CursorLeft,
	CursorRight,
	CursorUp,
	CursorDown,

	Enter,
}


pub trait ProcessInput {
	fn process_event(&mut self, event: InputEvent);
}

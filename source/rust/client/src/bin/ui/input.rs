use super::data::TextField;


pub trait ProcessInput<T> {
	fn process_char(&mut self, element: &mut T, c: char);
	fn process_cursor(&mut self, element: &mut T, direction: Direction);
}

pub enum Direction { Up, Down, Right, Left }


pub struct TextFieldProcessor;

impl ProcessInput<TextField> for TextFieldProcessor {
	fn process_char(&mut self, element: &mut TextField, c: char) {
		if c == '\x7f' { // Backspace
			element.text.pop();
		}
		else {
			element.text.push(c);
		}
	}

	fn process_cursor(&mut self, _element: &mut TextField, _d: Direction) {
		// TODO: Add support cursor movement
	}
}

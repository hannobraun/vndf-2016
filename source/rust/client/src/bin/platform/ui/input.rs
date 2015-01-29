use super::TextField;


pub trait ProcessInput<T> {
	fn process_char(&mut self, element: &mut T, c: char);
}


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
}

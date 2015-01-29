use super::data::{
	CommTab,
	TextField,
};


pub trait ProcessInput<T> {
	fn process_char(&mut self, element: &mut T, c: char);
	fn process_cursor(&mut self, element: &mut T, direction: Direction);
}

pub enum Direction { Up, Down, Right, Left }


pub struct CommTabProcessor;

impl ProcessInput<CommTab> for CommTabProcessor {
	fn process_char(&mut self, element: &mut CommTab, c: char) {
		if c == '\n' {
			element.is_sending = !element.is_sending;

			if !element.is_sending {
				element.broadcast_form.text_field.text.clear();
			}
		}
		else if !element.is_sending {
			TextFieldProcessor.process_char(
				&mut element.broadcast_form.text_field,
				c,
			);
		}
	}

	fn process_cursor(&mut self, element: &mut CommTab, direction: Direction) {
		if element.is_sending {
			match direction {
				Direction::Up =>
					TextFieldProcessor.process_char(
						&mut element.broadcast_form.text_field,
						'↑',
					),
				Direction::Down =>
					TextFieldProcessor.process_char(
						&mut element.broadcast_form.text_field,
						'↓',
					),

				_ => (),
			}
		}
	}
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

		// TODO: Add support for delete key (requires cursor movement)
	}

	fn process_cursor(&mut self, _element: &mut TextField, _d: Direction) {
		// TODO: Add support cursor movement
	}
}

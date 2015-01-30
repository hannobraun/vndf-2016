use super::data::{
	BroadcastForm,
	CommTab,
	TextField,
};


pub trait ProcessInput<T> {
	fn process_char(&mut self, element: &mut T, c: char);
	fn process_cursor(&mut self, element: &mut T, direction: Direction);
}

pub enum Direction { Up, Down, Right, Left }


pub struct BroadcastFormProcessor;

impl ProcessInput<BroadcastForm> for BroadcastFormProcessor {
	fn process_char(&mut self, element: &mut BroadcastForm, c: char) {
		TextFieldProcessor.process_char(
			&mut element.text_field,
			c,
		)
	}

	fn process_cursor(&mut self, _: &mut BroadcastForm, _: Direction) {}
}


pub struct CommTabProcessor;

impl ProcessInput<CommTab> for CommTabProcessor {
	fn process_char(&mut self, element: &mut CommTab, c: char) {
		if c == '\n' {
			element.element_active = !element.element_active;

			if element.element_active {
				element.broadcast_form.text_field.text.clear();
			}
		}
		else if element.element_active {
			BroadcastFormProcessor.process_char(
				&mut element.broadcast_form,
				c,
			);
		}
	}

	fn process_cursor(&mut self, element: &mut CommTab, direction: Direction) {
		if !element.element_active {
			match direction {
				Direction::Up =>
					BroadcastFormProcessor.process_char(
						&mut element.broadcast_form,
						'↑',
					),
				Direction::Down =>
					BroadcastFormProcessor.process_char(
						&mut element.broadcast_form,
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

use super::data::{
	BroadcastForm,
	CommTab,
	List,
	TextField,
};


pub trait ProcessInput {
	fn process_char(&mut self, c: char);
	fn process_cursor(&mut self, direction: Direction);
}

pub enum Direction { Up, Down, Right, Left }


impl ProcessInput for BroadcastForm {
	fn process_char(&mut self, c: char) {
		self.text_field.process_char(c)
	}

	fn process_cursor(&mut self, _: Direction) {}
}


impl ProcessInput for CommTab {
	fn process_char(&mut self, c: char) {
		if c == '\n' {
			self.element_active = !self.element_active;

			if self.element_active && self.form_is_selected() {
				self.broadcast_form.text_field.text.clear();
			}
		}
		else if self.element_active {
			self.selected_element_mut().process_char(c);
		}
	}

	fn process_cursor(&mut self, direction: Direction) {
		match direction {
			Direction::Up   => self.selected_index -= 1,
			Direction::Down => self.selected_index -= 1,

			_ => if self.element_active {
				self.selected_element_mut().process_cursor(direction)
			},
		}
	}
}


impl ProcessInput for List {
	fn process_char(&mut self, _: char) {}
	fn process_cursor(&mut self, direction: Direction) {
		match direction {
			Direction::Up   => self.first -= 1,
			Direction::Down => self.first += 1,
			_               => (),
		}
	}
}


impl ProcessInput for TextField {
	fn process_char(&mut self, c: char) {
		if c == '\x7f' { // Backspace
			// TODO(87369840): Take cursor position into account.
			self.text.pop();
		}
		else {
			self.text.push(c);
		}

		// TODO(87369840): Add support for delete key (requires cursor movement)
	}

	fn process_cursor(&mut self, _d: Direction) {
		// TODO(87369840): Add support cursor movement
	}
}

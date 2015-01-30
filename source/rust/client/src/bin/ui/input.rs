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

			if self.element_active {
				self.broadcast_form.text_field.text.clear();
			}
		}
		else if self.element_active {
			self.active_element_mut().process_char(c);
		}
	}

	fn process_cursor(&mut self, direction: Direction) {
		if self.element_active {
			self.active_element_mut().process_cursor(direction)
		}
		else {
			match direction {
				Direction::Up   => self.selected_index -= 1,
				Direction::Down => self.selected_index -= 1,
				_               => (),
			}
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
			self.text.pop();
		}
		else {
			self.text.push(c);
		}

		// TODO: Add support for delete key (requires cursor movement)
	}

	fn process_cursor(&mut self, _d: Direction) {
		// TODO: Add support cursor movement
	}
}

use super::base::{
	InputEvent,
	ProcessInput,
};
use super::base::InputEvent::{
	Char,
	CursorDown,
	CursorUp,
};
use super::state::{
	BroadcastForm,
	Button,
	CommTab,
	List,
	TextField,
};


impl ProcessInput for BroadcastForm {
	fn process_event(&mut self, event: InputEvent) {
		match event {
			Char(c) =>
				// TODO: Enter should be a separate variant of InputEvent.
				if c == '\n' {
					self.button.process_event(event)
				}
				else {
					self.text_field.process_event(event)
				},

			_ => (),
		}
	}
}


impl ProcessInput for Button {
	fn process_event(&mut self, event: InputEvent) {
		match event {
			Char(c) =>
				if c == '\n' {
					self.was_activated = true;
				},
			_ => (),
		}
	}
}


impl ProcessInput for CommTab {
	fn process_event(&mut self, event: InputEvent) {
		match event {
			Char(c) =>
				// TODO: Enter should be a separate variant of InputEvent
				if c == '\n' {
					self.element_active = !self.element_active;

					if self.element_active && self.form_is_selected() {
						self.broadcast_form.text_field.text.clear();
					}

					if self.form_is_selected() {
						self.broadcast_form.process_event(event);
					}
				}
				else if self.element_active {
					self.selected_element_mut().process_event(event);
				},

			CursorUp   => self.selected_index -= 1,
			CursorDown => self.selected_index += 1,

			_ => if self.element_active {
				self.selected_element_mut().process_event(event)
			},
		}
	}
}


impl ProcessInput for List {
	fn process_event(&mut self, event: InputEvent) {
		match event {
			CursorUp   => self.first -= 1,
			CursorDown => self.first += 1,
			_          => (),
		}
	}
}


impl ProcessInput for TextField {
	fn process_event(&mut self, event: InputEvent) {
		match event {
			Char(c) =>
				// TODO: Backspace should be a separate variant of InputEvent.
				if c == '\x7f' { // Backspace
					// TODO(87369840): Take cursor position into account.
					self.text.pop();
				}
				else {
					self.text.push(c);
				},

			_ => (),
		}

		// TODO(87369840): Add support cursor movement
		// TODO(87369840): Add support for delete key (requires cursor movement)
	}
}

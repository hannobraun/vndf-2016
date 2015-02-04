use super::base::{
	InputEvent,
	ProcessInput,
	Status,
};
use super::base::InputEvent::{
	Backspace,
	Char,
	CursorDown,
	CursorUp,
	Enter,
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
			Enter => {
				if self.text_field_status != Status::Active {
					self.text_field.text.clear();
				}

				self.button.process_event(event)
			},
			_ => self.text_field.process_event(event),
		}
	}
}


impl ProcessInput for Button {
	fn process_event(&mut self, event: InputEvent) {
		match event {
			Enter => self.was_activated = true,
			_     => (),
		}
	}
}


impl ProcessInput for CommTab {
	fn process_event(&mut self, event: InputEvent) {
		match event {
			CursorUp   => self.selected_index -= 1,
			CursorDown => self.selected_index += 1,

			_ => self.selected_element_mut().process_event(event),
		}
	}
}


impl ProcessInput for List {
	fn process_event(&mut self, event: InputEvent) {
		match event {
			Enter      => self.activated = !self.activated,
			CursorUp   => self.first -= 1,
			CursorDown => self.first += 1,
			_          => (),
		}
	}
}


impl ProcessInput for TextField {
	fn process_event(&mut self, event: InputEvent) {
		match event {
			// TODO(87369840): Take cursor position into account.
			Backspace => { self.text.pop(); },
			Char(c)   => self.text.push(c),
			_         => (),
		}

		// TODO(87369840): Add support cursor movement
		// TODO(87369840): Add support for delete key (requires cursor movement)
	}
}

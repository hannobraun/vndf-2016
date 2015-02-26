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
	MainSection,
	TabSwitcher,
	TextField,
};


impl ProcessInput for BroadcastForm {
	fn process_events(&mut self, events: &[InputEvent]) {
		for &event in events {
			match event {
				Enter => {
					if self.text_field_status != Status::Active {
						self.text_field.text.clear();
					}

					self.button.process_events(Some(event).as_slice())
				},
				_ => self.text_field.process_events(Some(event).as_slice()),
			}
		}
	}
}


impl ProcessInput for Button {
	fn process_events(&mut self, events: &[InputEvent]) {
		for &event in events {
			match event {
				Enter => self.was_activated = true,
				_     => (),
			}
		}
	}
}


impl ProcessInput for CommTab {
	fn process_events(&mut self, events: &[InputEvent]) {
		for &event in events {
			match event {
				CursorUp   if !self.broadcast_list.activated =>
					self.selected_index -= 1,
				CursorDown if !self.broadcast_list.activated =>
					self.selected_index += 1,

				_ => self.selected_element_mut().process_events(Some(event).as_slice()),
			}
		}
	}
}


impl ProcessInput for List {
	fn process_events(&mut self, events: &[InputEvent]) {
		for &event in events {
			match event {
				Enter      => self.activated = !self.activated,
				CursorUp   => if self.first > 0 { self.first -= 1 },
				CursorDown => self.first += 1,
				_          => (),
			}
		}
	}
}


impl ProcessInput for MainSection {
	fn process_events(&mut self, events: &[InputEvent]) {
		self.tab_switcher.process_events(events)
	}
}


impl ProcessInput for TabSwitcher {
	fn process_events(&mut self, events: &[InputEvent]) {
		// TODO: If the left or right cursor keys are pressed, update the
		//       currently selected tab.
		self.comm_tab.process_events(events)
	}
}


impl ProcessInput for TextField {
	fn process_events(&mut self, events: &[InputEvent]) {
		for &event in events {
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
}

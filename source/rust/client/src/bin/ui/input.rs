use super::base::{
	InputEvent,
	ProcessInput,
	Status,
};
use super::base::InputEvent::{
	Backspace,
	Char,
	CursorDown,
	CursorLeft,
	CursorRight,
	CursorUp,
	Enter,
};
use super::state::{
	BroadcastForm,
	Button,
	CommTab,
	List,
	MainSection,
	NavTab,
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

impl ProcessInput for NavTab {
	fn process_events(&mut self, _: &[InputEvent]) {}
}


impl ProcessInput for TabSwitcher {
	fn process_events(&mut self, events: &[InputEvent]) {
		for &event in events {
			match event {
				CursorLeft  => self.active_index -= 1,
				CursorRight => self.active_index += 1,
				_           => self.active_tab().process_events(Some(event).as_slice()),
			}
		}
	}
}


impl ProcessInput for TextField {
	fn process_events(&mut self, events: &[InputEvent]) {
		for &event in events {
			match event {
				Backspace => { self.text.pop(); },
				Char(c)   => self.text.push(c),
				_         => (),
			}
		}
	}
}

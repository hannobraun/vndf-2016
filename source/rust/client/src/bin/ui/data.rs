use super::input::ProcessInput;


pub trait Element: ProcessInput {}


pub struct BroadcastForm {
	pub text_field: TextField,
	pub button    : Button,
}

impl BroadcastForm {
	pub fn new() -> BroadcastForm {
		BroadcastForm {
			text_field: TextField::new(),
			button    : Button,
		}
	}
}

impl Element for BroadcastForm {}


pub struct Button;


pub struct CommTab {
	pub element_active: bool,
	pub active_index  : u8,
	pub broadcast_form: BroadcastForm,
	pub broadcast_list: List,
}

impl CommTab {
	pub fn new() -> CommTab {
		CommTab {
			element_active: true,
			active_index  : 0,
			broadcast_form: BroadcastForm::new(),
			broadcast_list: List::new(),
		}
	}

	pub fn active_element_mut(&mut self) -> &mut Element {
		match self.active_index % 2 {
			0 => &mut self.broadcast_form,
			1 => &mut self.broadcast_list,
			_ => panic!("This should not happen"),
		}
	}

	pub fn form_is_active(&self) -> bool {
		self.active_index % 2 == 0
	}

	pub fn list_is_active(&self) -> bool {
		self.active_index % 2 == 1
	}
}


pub struct List {
	pub first: usize,
}

impl List {
	pub fn new() -> List {
		List {
			first: 0,
		}
	}
}

impl Element for List {}


pub struct TextField {
	pub text: String,
}

impl TextField {
	pub fn new() -> TextField {
		TextField {
			text: String::new(),
		}
	}
}

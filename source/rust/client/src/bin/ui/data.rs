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


pub struct Button;


pub struct CommTab {
	pub element_active: bool,
	pub broadcast_form: BroadcastForm,
	pub broadcast_list: List,
}

impl CommTab {
	pub fn new() -> CommTab {
		CommTab {
			element_active: true,
			broadcast_form: BroadcastForm::new(),
			broadcast_list: List::new(),
		}
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

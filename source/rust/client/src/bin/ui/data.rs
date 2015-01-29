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

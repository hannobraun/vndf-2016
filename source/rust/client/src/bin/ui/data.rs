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

use client::platform::Input;


pub struct Ui {
	pub input_active   : bool,
	pub broadcast_field: TextField,
}

impl Ui {
	pub fn new() -> Ui {
		Ui {
			input_active   : true,
			broadcast_field: TextField::new(),
		}
	}

	pub fn process_input(&mut self, chars: &[char]) -> Input {
		for &c in chars.iter() {
			if c == '\n' {
				self.input_active = !self.input_active;

				if self.input_active {
					self.broadcast_field.activate();
				}
			}
			else if self.input_active {
				self.broadcast_field.process_char(c);
			}
		}

		let mut input = Input::new();
		input.broadcast = if !self.input_active {
			Some(self.broadcast_field.text().to_string())
		}
		else {
			None
		};

		input
	}
}


pub struct TextField {
	text: String,
}

impl TextField {
	pub fn new() -> TextField {
		TextField {
			text: String::new(),
		}
	}

	pub fn text(&self) -> &str {
		self.text.as_slice()
	}

	pub fn activate(&mut self) {
		self.text.clear();
	}

	pub fn process_char(&mut self, c: char) {
		if c == '\x7f' { // Backspace
			self.text.pop();
		}
		else {
			self.text.push(c);
		}
	}
}

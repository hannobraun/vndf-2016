pub struct Ui {
	pub input_active: bool,
	pub input_text  : String,
}

impl Ui {
	pub fn new() -> Ui {
		Ui {
			input_active: false,
			input_text  : String::new(),
		}
	}

	pub fn process_input(&mut self, chars: &[char]) {
		for &c in chars.iter() {
			if c == '\n' {
				self.input_active = !self.input_active;

				if !self.input_active {
					self.input_text.clear();
				}
			}
			else if c == '\x7f' { // Backspace
				self.input_text.pop();
			}
			else {
				self.input_text.push(c);
			}
		}
	}
}

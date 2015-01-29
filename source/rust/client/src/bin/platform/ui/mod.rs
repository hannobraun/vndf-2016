pub mod data;
pub mod input;


use client::platform::Input;

use self::data::List;
use self::input::{
	ProcessInput,
	TextFieldProcessor,
};


pub struct Ui {
	pub element_active : bool,
	pub broadcast_field: TextField,
	pub broadcast_list : List,

	mode: TextInputMode,
}

impl Ui {
	pub fn new() -> Ui {
		Ui {
			element_active : true,
			broadcast_field: TextField::new(),
			broadcast_list : List::new(),
			mode           : TextInputMode::Regular,
		}
	}

	pub fn process_input(&mut self, chars: &[char]) -> Input {
		for &c in chars.iter() {
			match self.mode {
				TextInputMode::Regular => {
					if c == '\n' {
						self.element_active = !self.element_active;

						if self.element_active {
							self.broadcast_field.text.clear();
						}
					}
					else if c == '\x1b' { // Escape
						self.mode = TextInputMode::Escape;
					}
					else if self.element_active {
						TextFieldProcessor.process_char(
							&mut self.broadcast_field,
							c,
						);
					}
				},
				TextInputMode::Escape => {
					if c == '[' {
						self.mode = TextInputMode::Cursor;
					}
					else {
						// Unexpected character. Fall back to regular mode.
						self.mode = TextInputMode::Regular;
					}
				},
				TextInputMode::Cursor => {
					if !self.element_active {
						match c {
							'A' => TextFieldProcessor.process_char(&mut self.broadcast_field, '↑'), // up
							'B' => TextFieldProcessor.process_char(&mut self.broadcast_field, '↓'), // down
							'C' => (), // right
							'D' => (), // left
							_   => (), // Unexpected character
						}
					}
					else {
						// Ignore, the text field won't know what to do with it
					}

					self.mode = TextInputMode::Regular;
				},
			}
		}

		let mut input = Input::new();
		input.broadcast = if !self.element_active {
			Some(self.broadcast_field.text.clone())
		}
		else {
			None
		};

		input
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


enum TextInputMode {
	Regular,
	Escape,
	Cursor,
}

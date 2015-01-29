pub mod data;
pub mod input;
pub mod render;


use client::platform::Input;

use self::data::{
	BroadcastForm,
	List,
};
use self::input::{
	ProcessInput,
	TextFieldProcessor,
};


pub struct Ui {
	pub is_sending    : bool,
	pub broadcast_form: BroadcastForm,
	pub broadcast_list: List,

	mode: TextInputMode,
}

impl Ui {
	pub fn new() -> Ui {
		Ui {
			is_sending    : false,
			broadcast_form: BroadcastForm::new(),
			broadcast_list: List::new(),
			mode          : TextInputMode::Regular,
		}
	}

	pub fn process_input(&mut self, chars: &[char]) -> Input {
		for &c in chars.iter() {
			match self.mode {
				TextInputMode::Regular => {
					if c == '\n' {
						self.is_sending = !self.is_sending;

						if !self.is_sending {
							self.broadcast_form.text_field.text.clear();
						}
					}
					else if c == '\x1b' { // Escape
						self.mode = TextInputMode::Escape;
					}
					else if !self.is_sending {
						TextFieldProcessor.process_char(
							&mut self.broadcast_form.text_field,
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
					if self.is_sending {
						match c {
							'A' => TextFieldProcessor.process_char(&mut self.broadcast_form.text_field, '↑'), // up
							'B' => TextFieldProcessor.process_char(&mut self.broadcast_form.text_field, '↓'), // down
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
		input.broadcast = if self.is_sending {
			Some(self.broadcast_form.text_field.text.clone())
		}
		else {
			None
		};

		input
	}
}


enum TextInputMode {
	Regular,
	Escape,
	Cursor,
}

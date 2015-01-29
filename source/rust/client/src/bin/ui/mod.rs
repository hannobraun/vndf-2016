pub mod data;
pub mod input;
pub mod render;


use client::platform::Input;

use self::data::CommTab;
use self::input::{
	CommTabProcessor,
	Direction,
	ProcessInput,
	TextFieldProcessor,
};


pub struct Ui {
	pub comm_tab: CommTab,

	mode: TextInputMode,
}

impl Ui {
	pub fn new() -> Ui {
		Ui {
			comm_tab: CommTab::new(),
			mode    : TextInputMode::Regular,
		}
	}

	pub fn process_input(&mut self, chars: &[char]) -> Input {
		// TODO: Split this function up into multiple layered input processors.

		for &c in chars.iter() {
			match self.mode {
				TextInputMode::Regular => {
					if c == '\n' {
						self.comm_tab.is_sending = !self.comm_tab.is_sending;

						if !self.comm_tab.is_sending {
							self.comm_tab.broadcast_form.text_field.text.clear();
						}
					}
					else if c == '\x1b' { // Escape
						self.mode = TextInputMode::Escape;
					}
					else if !self.comm_tab.is_sending {
						TextFieldProcessor.process_char(
							&mut self.comm_tab.broadcast_form.text_field,
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
					let direction = match c {
						'A' => Some(Direction::Up),
						'B' => Some(Direction::Down),
						'C' => Some(Direction::Right),
						'D' => Some(Direction::Left),
						_   => None, // Unexpected character
					};

					if let Some(direction) = direction {
						CommTabProcessor.process_cursor(
							&mut self.comm_tab,
							direction,
						);
					}

					self.mode = TextInputMode::Regular;
				},
			}
		}

		let mut input = Input::new();
		input.broadcast = if self.comm_tab.is_sending {
			Some(self.comm_tab.broadcast_form.text_field.text.clone())
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

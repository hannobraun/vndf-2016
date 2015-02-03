pub mod base;
pub mod input;
pub mod render;
pub mod state;


use std::vec::Drain;

use client::platform::{
	Frame,
	InputEvent,
};

use self::base::ProcessInput;
use self::base::InputEvent::{
	Backspace,
	Char,
	CursorDown,
	CursorLeft,
	CursorRight,
	CursorUp,
	Enter,
};
use self::state::CommTab;


pub struct Ui {
	pub comm_tab: CommTab,

	mode  : TextInputMode,
	events: Vec<InputEvent>,
}

impl Ui {
	pub fn new() -> Ui {
		Ui {
			comm_tab: CommTab::new(),
			mode    : TextInputMode::Regular,
			events  : Vec::new(),
		}
	}

	pub fn update(&mut self, frame: &Frame, chars: &[char])
		-> Drain<InputEvent>
	{
		for &c in chars.iter() {
			match self.mode {
				TextInputMode::Regular => {
					if c == '\x1b' { // Escape
						self.mode = TextInputMode::Escape;
					}
					else if c == '\x7f' { // Backspace
						self.comm_tab.process_event(Backspace);
					}
					else if c == '\n' {
						self.comm_tab.process_event(Enter);
					}
					else {
						self.comm_tab.process_event(Char(c));
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
					let event = match c {
						'A' => Some(CursorUp),
						'B' => Some(CursorDown),
						'C' => Some(CursorRight),
						'D' => Some(CursorLeft),
						_   => None, // Unexpected character
					};

					if let Some(event) = event {
						self.comm_tab.process_event(event);
					}

					self.mode = TextInputMode::Regular;
				},
			}
		}

		let is_sending = frame.broadcasts
			.iter()
			.any(|broadcast|
				broadcast.sender == frame.self_id
			);

		if self.comm_tab.broadcast_form.button.was_activated {
			self.comm_tab.broadcast_form.button.was_activated = false;

			if is_sending {
				self.events.push(InputEvent::StopBroadcast);
			}
			else {
				let message =
					self.comm_tab.broadcast_form.text_field.text.clone();
				self.events.push(InputEvent::StartBroadcast(message));
			}
		}

		self.events.drain()
	}
}


enum TextInputMode {
	Regular,
	Escape,
	Cursor,
}

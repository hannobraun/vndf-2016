pub mod base;
pub mod input;
pub mod reader;
pub mod render;
pub mod state;
pub mod update;


use std::old_io::IoResult;
use std::vec::Drain;

use client::interface::{
	Frame,
	InputEvent,
};
use render::Screen;

use self::base::{
	ProcessInput,
	Update,
};
use self::base::InputEvent::{
	Backspace,
	Char,
	CursorDown,
	CursorLeft,
	CursorRight,
	CursorUp,
	Enter,
};
use self::reader::InputReader;
use self::state::{
	InfoSection,
	MainSection,
};
use self::update::MainSectionArgs;


pub struct Ui {
	screen: Screen,
	reader: InputReader,
	chars : Vec<char>,
	main  : MainSection,
	info  : InfoSection,
	mode  : TextInputMode,
	events: Vec<InputEvent>,
}

impl Ui {
	pub fn new() -> IoResult<Ui> {
		let width = 80;

		let screen = try!(Screen::new(width, 24));

		Ok(Ui {
			screen: screen,
			reader: InputReader::new(),
			chars : Vec::new(),
			main  : MainSection::new(width, 18),
			info  : InfoSection::new(width,  6),
			mode  : TextInputMode::Regular,
			events: Vec::new(),
		})
	}

	pub fn update(&mut self, frame: &Frame)
		-> IoResult<Drain<InputEvent>>
	{
		self.process_input();
		try!(self.generate_events(frame));

		Ok(self.events.drain())
	}

	fn process_input(&mut self) {
		self.reader.input(&mut self.chars);

		for c in self.chars.drain() {
			match self.mode {
				TextInputMode::Regular => {
					if c == '\x1b' { // Escape
						self.mode = TextInputMode::Escape;
					}
					else if c == '\x7f' { // Backspace
						self.main.process_events(Some(Backspace).as_slice());
					}
					else if c == '\n' {
						self.main.process_events(Some(Enter).as_slice());
					}
					else {
						self.main.process_events(Some(Char(c)).as_slice());
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
						self.main.process_events(Some(event).as_slice());
					}

					self.mode = TextInputMode::Regular;
				},
			}
		}
	}

	fn generate_events(&mut self, frame: &Frame) -> IoResult<()> {
		let is_sending = frame.broadcasts
			.iter()
			.any(|broadcast|
				broadcast.sender == frame.self_id
			);

		let mut broadcasts: Vec<String> = frame.broadcasts
			.iter()
			.map(|broadcast|
				format!("{}: {}", broadcast.sender, broadcast.message)
			)
			.collect();
		broadcasts.sort();

		self.screen.cursor(None);

		try!(self.main.update(
			self.screen.buffer(),
			0, 0,
			&MainSectionArgs {
				is_sending: is_sending,
				self_id   : frame.self_id.as_slice(),
				broadcasts: broadcasts.as_slice(),
				position  : frame.position,
				velocity  : frame.velocity,
			}
		));
		try!(self.info.update(self.screen.buffer(), 0, self.main.height, &frame.status));

		try!(self.screen.submit());

		if self.main.tab_switcher.comm_tab.broadcast_form.button.was_activated {
			self.main.tab_switcher.comm_tab.broadcast_form.button.was_activated = false;

			if is_sending {
				self.events.push(InputEvent::StopBroadcast);
			}
			else {
				let message =
					self.main.tab_switcher.comm_tab.broadcast_form.text_field.text.clone();
				self.events.push(InputEvent::StartBroadcast(message));
			}
		}

		Ok(())
	}
}


enum TextInputMode {
	Regular,
	Escape,
	Cursor,
}

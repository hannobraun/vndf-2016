mod input;
mod render;
mod ui;


use std::io::{
	stdin,
	IoResult,
};
use std::sync::mpsc::{
	channel,
	Receiver,
	TryRecvError,
};
use std::thread::Thread;

use client::platform::{
	Frame,
	Input,
};

use self::input::InputReader;
use self::render::Renderer;
use self::ui::Ui;


pub type Pos = u16;


pub trait PlatformIo {
	fn new() -> IoResult<Self>;
	fn input(&mut self) -> Input;
	fn render(&mut self, frame: &Frame) -> IoResult<()>;
}


pub struct PlayerIo {
	input_reader: InputReader,
	ui          : Ui,
	renderer    : Renderer,
	chars       : Vec<char>,
}

impl PlatformIo for PlayerIo {
	fn new() -> IoResult<PlayerIo> {
		let renderer = match Renderer::new() {
			Ok(renderer) => renderer,
			Err(error)   => return Err(error),
		};

		Ok(PlayerIo {
			input_reader: InputReader::new(),
			ui          : Ui::new(),
			renderer    : renderer,
			chars       : Vec::new(),
		})
	}

	fn input(&mut self) -> Input {
		self.chars.clear();
		self.input_reader.input(&mut self.chars);
		self.ui.process_input(self.chars.as_slice())
	}

	fn render(&mut self, frame: &Frame) -> IoResult<()> {
		self.renderer.render(frame, &self.ui)
	}
}


pub struct HeadlessIo {
	last_input: Input,
	receiver  : Receiver<Input>,
}

impl PlatformIo for HeadlessIo {
	fn new() -> IoResult<HeadlessIo> {
		let (sender, receiver) = channel();

		Thread::spawn(move || -> () {
			let mut stdin = stdin();

			loop {
				// TODO(83541252): This operation should time out to ensure
				//                 panic propagation between tasks.
				match stdin.read_line() {
					Ok(line) => match Input::from_json(line.as_slice()) {
						Ok(input) =>
							match sender.send(input) {
								Ok(()) =>
									(),
								Err(error) =>
									panic!("Error sending input: {:?}", error),
							},
						Err(error) =>
							panic!("Error decoding input: {:?}", error),
					},
					Err(error) =>
						panic!("Error reading from stdin: {}", error),
				}
			}
		});

		Ok(HeadlessIo {
			receiver  : receiver,
			last_input: Input::new(),
		})
	}

	fn input(&mut self) -> Input {
		match self.receiver.try_recv() {
			Ok(input) => {
				self.last_input = input.clone();
				input
			},
			Err(error) => match error {
				TryRecvError::Empty        => self.last_input.clone(),
				TryRecvError::Disconnected => panic!("Channel disconnected"),
			}
		}
	}

	fn render(&mut self, frame: &Frame) -> IoResult<()> {
		print!("{}\n", frame.to_json());
		Ok(())
	}
}

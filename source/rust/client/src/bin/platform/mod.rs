mod input_reader;
mod renderer;


use std::old_io::{
	stdin,
	IoResult,
};
use std::sync::mpsc::{
	channel,
	Receiver,
	TryRecvError,
};
use std::thread::Thread;
use std::vec::Drain;

use client::platform::{
	Frame,
	InputEvent,
};
use ui::Ui;

use self::input_reader::InputReader;
use self::renderer::Renderer;


pub trait PlatformIo {
	fn new() -> IoResult<Self>;
	fn update(&mut self, frame: &Frame) -> Drain<InputEvent>;
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

	fn update(&mut self, frame: &Frame) -> Drain<InputEvent> {
		self.chars.clear();
		self.input_reader.input(&mut self.chars);
		self.ui.process_input(frame, self.chars.as_slice())
	}

	fn render(&mut self, frame: &Frame) -> IoResult<()> {
		self.renderer.render(frame, &self.ui)
	}
}


pub struct HeadlessIo {
	events  : Vec<InputEvent>,
	receiver: Receiver<InputEvent>,
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
					Ok(line) => match InputEvent::from_json(line.as_slice()) {
						Ok(event) =>
							match sender.send(event) {
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
			events  : Vec::new(),
			receiver: receiver,
		})
	}

	fn update(&mut self, _: &Frame) -> Drain<InputEvent> {
		loop {
			match self.receiver.try_recv() {
				Ok(event) =>
					self.events.push(event),
				Err(error) => match error {
					TryRecvError::Empty        => break,
					TryRecvError::Disconnected => panic!("Channel disconnected"),
				}
			}
		}

		self.events.drain()
	}

	fn render(&mut self, frame: &Frame) -> IoResult<()> {
		print!("{}\n", frame.to_json());
		Ok(())
	}
}

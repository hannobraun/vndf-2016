use std::io::{BufferedReader, PipeStream, Process};
use std::os;
use std::str;

use common::protocol::{Create, Message, Remove, SelfInfo, Update};
use common::physics::Vec2;

use entities::Entities;


// This module starts the vndf-core executable and reads its messages, updating
// the ships. The approach used here to do this is certainly not ideal:
// - The I/O used to read the messages is blocking.
// - We're reading this without the use of any tasks or anything else that would
//   introduce asynchronicity.
// - Ergo, we're binding the rendering rate to the rate of messages received
//   from vndf-core.
// I've tried to prevent just that by reading from vndf-core in a task and
// reading from the task without blocking. This didn't work. There were a lot of
// timing issues that I was unable to track down.
// For now, this ghetto solution works well enough. However, this code should be
// revisited, once task support is more solid, or, if it is already solid, I've
// learned how to use it correctly, or until Rust grows asynch I/O.


pub struct Core {
	process: Process,
	stdout : BufferedReader<PipeStream>,
	stderr : BufferedReader<PipeStream>
}

impl Core {
	pub fn start(server: ~str) -> ~Core {
		let mut path =
			os::self_exe_path().expect("Failed to get executable path");

		path.push("vndf-client-core");

		let path_str = path.as_str().expect("Failed to convert path to string");
		let args = [server, ~"34481"];

		let mut process = match Process::new(path_str, args) {
			Ok(process) => process,
			Err(error)  => fail!("Failed to create process: {}", error)
		};

		let stdout = BufferedReader::new(
			process.stdout.take().expect("Expected stdout to be set"));
		let stderr = BufferedReader::new(
			process.stderr.take().expect("Expected stderr to be set"));

		~Core {
			process: process,
			stdout : stdout,
			stderr : stderr }
	}

	pub fn get_self_id(&mut self) -> uint {
		let message = self.read_message();

		match Message::from_str(message) {
			SelfInfo(self_info) => self_info.id,

			_  => fail!("unexpected message ({})", message)
		}
	}

	pub fn update_positions(&mut self, entities: &mut Entities) {
		let message = self.read_message();

		match Message::from_str(message) {
			Create(create) =>
				entities.create_ship(
					create.id),

			Update(update) =>
				entities.update_ship(
					update.id,
					Vec2 {
						x: update.body.position.x,
						y: update.body.position.y
					}),

			Remove(remove) =>
				entities.remove_ship(
					remove.id),

			_ => fail!("unexpected message ({})", message)
		}
	}

	fn read_message(&mut self) -> ~str {
		match self.stdout.read_line() {
			Ok(message) => message,
			Err(error)  => {
				print!("Failed to read message from client-core: {}\n", error);
				self.handle_error()
			}
		}
	}

	fn handle_error(&mut self) -> ! {
		print!("Outputs of core:\n");
		print!("stdout:\n{}\n", reader_to_string(&mut self.stdout));
		print!("stderr:\n{}\n", reader_to_string(&mut self.stderr));
		fail!();
	}
}

impl Drop for Core {
	fn drop(&mut self) {
		// Make sure the process is killed, when Core drops out of scope.
		// Sometimes client-core is killed automatically on exit, sometimes it
		// hangs around and prevents the process from exiting. This seems to
		// depend on what I do with the PipeStreams. What exactly causes this
		// behavior is beyond my current understanding and killing it
		// explicitely works fine, so there.
		let _ = self.process.signal_kill();
	}
}


fn reader_to_string(reader: &mut BufferedReader<PipeStream>) -> ~str {
	str::from_utf8(reader.read_to_end().unwrap().as_slice()).unwrap().to_owned()
}

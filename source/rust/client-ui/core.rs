use std::from_str;
use std::io::{BufferedReader, PipeStream, Process};
use std::os;
use std::str;

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
		let mut path = match os::self_exe_path() {
			Some(path) => path,
			None       => fail!("Failed to get executable path.")
		};

		path.push("vndf-client-core");
		let args = [server, ~"34481"];

		let mut process = match Process::new(path.as_str().unwrap(), args) {
			Ok(process) => process,
			Err(error)  => fail!("Failed to create process: {}", error)
		};

		let stdout = BufferedReader::new(process.stdout.take().unwrap());
		let stderr = BufferedReader::new(process.stderr.take().unwrap());

		~Core {
			process: process,
			stdout : stdout,
			stderr : stderr }
	}

	pub fn get_self_id(&mut self) -> int {
		let message = match self.stdout.read_line() {
			Ok(message) => message,
			Err(error)  => {
				print!("Failed to read message from client-core: {}\n", error);
				self.handle_error()
			}
		};

		let words = message.words().to_owned_vec();
		if words[0] == "SELF_ID" {
			from_str::from_str(
				words[1]).unwrap_or_else(|| { fail!() })
		}
		else {
			fail!("expected self id");
		}
	}

	pub fn update_positions(&mut self, entities: &mut Entities) {
		let message = match self.stdout.read_line() {
			Ok(message) => message,
			Err(error)  => {
				print!("Failed to read message from client-core: {}\n", error);
				self.handle_error()
			}
		};

		let words = message.words().to_owned_vec();
		if words[0] == "UPDATE" {
			let id = from_str::from_str(
				words[1]).unwrap_or_else(|| { fail!() });
			let x = from_str::from_str(
				words[2]).unwrap_or_else(|| { fail!() });
			let y = from_str::from_str(
				words[3]).unwrap_or_else(|| { fail!() });
			let z = from_str::from_str(
				words[4]).unwrap_or_else(|| { fail!() });

			entities.update_ship(id, x, y, z);
		}
		if words[0] == "REMOVE" {
			let id = from_str::from_str(
				words[1]).unwrap_or_else(|| { fail!() });

			entities.remove_ship(id);
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
		match self.process.signal_kill() {
			Ok(_)      => (),
			Err(error) => fail!("error killing core process: {}", error)
		}
	}
}

fn reader_to_string(reader: &mut BufferedReader<PipeStream>) -> ~str {
	str::from_utf8(reader.read_to_end().unwrap()).unwrap().to_owned()
}

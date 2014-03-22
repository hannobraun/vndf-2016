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
	process: Process
}

impl Core {
	pub fn start(server: ~str) -> ~Core {
		let mut path = match os::self_exe_path() {
			Some(path) => path,
			None       => fail!("Failed to get executable path.")
		};

		path.push("vndf-client-core");
		let args = [server, ~"34481"];

		let process = match Process::new(path.as_str().unwrap(), args) {
			Ok(process) => process,
			Err(error)  => fail!("Failed to create process: {}", error)
		};

		~Core {
			process: process }
	}

	pub fn update_positions(&mut self, entities: &mut Entities) {
		let mut stdout = BufferedReader::new(
			self.process.stdout.clone().unwrap());
		let stderr = BufferedReader::new(
			self.process.stderr.clone().unwrap());

		let message = match stdout.read_line() {
			Ok(message) => message,
			Err(error)  => {
				print!("Failed to read message from client-core: {}\n", error);
				handle_error(stdout, stderr)
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
}

fn handle_error(
	mut stdout: BufferedReader<PipeStream>,
	mut stderr: BufferedReader<PipeStream>) -> ! {

	print!("Outputs of core:\n");
	print!("stdout:\n{}\n", reader_to_string(&mut stdout));
	print!("stderr:\n{}\n", reader_to_string(&mut stderr));
	fail!();
}

fn reader_to_string(reader: &mut BufferedReader<PipeStream>) -> ~str {
	str::from_utf8(reader.read_to_end().unwrap()).unwrap().to_owned()
}

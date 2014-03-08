use std::io;
use std::io::{BufferedReader, PipeStream};


pub struct Process {
	process: io::Process,
	stdout : BufferedReader<PipeStream>
}

impl Process {
	pub fn start(path: &str, args: &[~str]) -> Process {
		let process = match io::Process::new(path, args) {
			Ok(process) => process,
			Err(error)  => fail!("Failed to start process {}: {}", path, error)
		};

		let stdout = BufferedReader::new(
			process.stdout.clone().unwrap());

		Process {
			process: process,
			stdout : stdout
		}
	}

	pub fn kill(&mut self) {
		match self.process.signal_kill() {
			Ok(_)      => (), // nothing to do
			Err(error) => print!("ERROR Failed to kill process: {}\n", error)
		}

		print!(
			"stdout: {}\n",
			self.process.stdout.clone().unwrap().read_to_str().unwrap());
		print!(
			"stderr: {}\n",
			self.process.stderr.clone().unwrap().read_to_str().unwrap());
	}
}

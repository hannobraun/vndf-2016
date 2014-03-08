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

		let stdout_opt = process.stdout.clone();

		Process {
			process: process,
			stdout : to_reader(stdout_opt)
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

fn to_reader(pipe_opt: Option<PipeStream>) -> BufferedReader<PipeStream> {
	let pipe = match pipe_opt {
		Some(pipe) => pipe,
		None       => fail!()
	};

	BufferedReader::new(pipe)
}

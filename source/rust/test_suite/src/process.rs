use std::old_io;
use std::old_io::{
	BufferedReader,
	PipeStream,
};


pub struct Process {
	process: old_io::Process,
	path   : String,
	stdout : BufferedReader<PipeStream>,
	stderr : BufferedReader<PipeStream>,
	stdin  : PipeStream,
}

impl Process {
	pub fn start(path: &str, args: &[&str]) -> Process {
		let mut process = match old_io::Command::new(path).args(args).spawn() {
			Ok(process) => process,
			Err(error)  => panic!("Failed to start process {}: {}", path, error)
		};

		let stdout_opt = process.stdout.take();
		let stderr_opt = process.stderr.take();
		let stdin_opt  = process.stdin.take();

		Process {
			process: process,
			path   : path.to_string(),
			stdout : to_reader(stdout_opt),
			stderr : to_reader(stderr_opt),
			stdin  : stdin_opt.expect("Expected stdin"),
		}
	}

	pub fn kill(&mut self) {
		if let Err(error) = self.process.signal_kill() {
			print!("Error killing process: {}\n", error);
		}
	}

	pub fn read_stdout_line(&mut self) -> String {
		match self.stdout.read_line() {
			Ok(line)   => line,
			Err(error) => panic!("Failed to read line from stdout: {}", error)
		}
	}

	pub fn write_stdin(&mut self, input: &str) {
		if let Err(error) = self.stdin.write_str(input) {
			panic!("Failed to write to stdin: {}", error);
		}
	}

	pub fn write_stdin_line(&mut self, line: &str) {
		if let Err(error) = self.stdin.write_line(line) {
			panic!("Failed to write to stdin: {}", error);
		}
	}
}

impl Drop for Process {
	fn drop(&mut self) {
		self.kill();

		print!("Output for process {}\n", self.path);
		print!(
			"stdout:\n{}\n",
			self.stdout.read_to_string().unwrap(),
		);
		print!(
			"stderr:\n{}\n",
			self.stderr.read_to_string().unwrap(),
		);
	}
}

fn to_reader(pipe_opt: Option<PipeStream>) -> BufferedReader<PipeStream> {
	let pipe = match pipe_opt {
		Some(pipe) => pipe,
		None       => panic!(),
	};

	BufferedReader::new(pipe)
}

use std::io::prelude::*;
use std::io::BufReader;
use std::process::{
	Child,
	ChildStderr,
	ChildStdin,
	ChildStdout,
	Command,
	Stdio,
};


pub struct Process {
	process   : Child,
	path      : String,
	stdout    : BufReader<ChildStdout>,
	stderr    : BufReader<ChildStderr>,
	stdin     : ChildStdin,
	stdout_buf: String,
}

impl Process {
	pub fn start(path: &str, args: &[&str]) -> Process {
		let command = Command::new(path)
			.args(args)
			.stdin(Stdio::piped())
			.stdout(Stdio::piped())
			.stderr(Stdio::piped())
			.spawn();

		let mut process = match command {
			Ok(process) => process,
			Err(error)  => panic!("Failed to start process {}: {}", path, error)
		};

		let stdout = process.stdout.take();
		let stderr = process.stderr.take();
		let stdin  = process.stdin.take();

		Process {
			process   : process,
			path      : path.to_string(),
			stdout    : to_buffered(stdout),
			stderr    : to_buffered(stderr),
			stdin     : stdin.expect("Expected stdin"),
			stdout_buf: String::new(),
		}
	}

	pub fn kill(&mut self) {
		if let Err(error) = self.process.kill() {
			print!("Error killing process: {}\n", error);
		}
	}

	pub fn read_stdout_line(&mut self) -> String {
		let mut line = String::new();
		match self.stdout.read_line(&mut line) {
			Ok(_) => {
				self.stdout_buf.extend(line.chars());
				line
			},
			Err(error) => panic!("Failed to read line from stdout: {}", error)
		}
	}

	pub fn write_stdin(&mut self, input: &str) {
		if let Err(error) = write!(&mut self.stdin, "{}", input) {
			panic!("Failed to write to stdin: {}", error);
		}
	}

	pub fn write_stdin_line(&mut self, line: &str) {
		if let Err(error) = write!(&mut self.stdin, "{}\n", line) {
			panic!("Failed to write to stdin: {}", error);
		}
	}
}

impl Drop for Process {
	fn drop(&mut self) {
		self.kill();

		self.stdout
			.read_to_string(&mut self.stdout_buf)
			.ok()
			.expect("Error reading from stdout");

		let mut stderr = String::new();
		self.stderr
			.read_to_string(&mut stderr)
			.ok()
			.expect("Error reading from stderr");

		print!("Output for process {}\n", self.path);
		print!("stdout:\n{}\n", self.stdout_buf);
		print!("stderr:\n{}\n", stderr);
	}
}


fn to_buffered<R: Read>(reader: Option<R>) -> BufReader<R> {
	BufReader::new(reader.expect("Expected reader"))
}

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
	process: Child,
	path   : String,
	stdout : BufReader<ChildStdout>,
	stderr : BufReader<ChildStderr>,
	stdin  : ChildStdin,
}

impl Process {
	pub fn start(path: &str, args: &[&str]) -> Process {
		let command = Command::new(path)
			.args(args)
			.stdin(Stdio::capture())
			.stdout(Stdio::capture())
			.stderr(Stdio::capture())
			.spawn();

		let mut process = match command {
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
		if let Err(error) = self.process.kill() {
			print!("Error killing process: {}\n", error);
		}
	}

	pub fn read_stdout_line(&mut self) -> String {
		let mut line = String::new();
		match self.stdout.read_line(&mut line) {
			Ok(())     => line,
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

		let mut stdout = String::new();
		self.stdout
			.read_to_string(&mut stdout)
			.ok()
			.expect("Error reading from stdout");

		let mut stderr = String::new();
		self.stderr
			.read_to_string(&mut stderr)
			.ok()
			.expect("Error reading from stderr");

		print!("Output for process {}\n", self.path);
		print!("stdout:\n{}\n", stdout);
		print!("stderr:\n{}\n", stderr);
	}
}

fn to_reader<R: Read>(pipe_opt: Option<R>) -> BufReader<R> {
	let pipe = match pipe_opt {
		Some(pipe) => pipe,
		None       => panic!(),
	};

	BufReader::new(pipe)
}

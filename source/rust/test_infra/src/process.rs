use std::io;
use std::io::{BufferedReader, PipeStream};


pub struct Process {
	process: io::Process,
	stdout : BufferedReader<PipeStream>,
	stderr : BufferedReader<PipeStream>,
	stdin  : PipeStream
}

impl Process {
	pub fn start(path: &str, args: &[&str]) -> Process {
		let mut process = match io::Command::new(path).args(args).spawn() {
			Ok(process) => process,
			Err(error)  => fail!("Failed to start process {}: {}", path, error)
		};

		let stdout_opt = process.stdout.take();
		let stderr_opt = process.stderr.take();
		let stdin_opt  = process.stdin.take();

		Process {
			process: process,
			stdout : to_reader(stdout_opt),
			stderr : to_reader(stderr_opt),
			stdin  : stdin_opt.expect("Expected stdin")
		}
	}

	pub fn kill(&mut self) {
		match self.process.signal_kill() {
			Ok(())     => (),
			Err(error) => print!("Error killing process: {}\n", error)
		}
	}

	pub fn read_stdout_line(&mut self) -> String {
		match self.stdout.read_line() {
			Ok(line)   => line,
			Err(error) => fail!("Failed to read line from stdout: {}", error)
		}
	}

	pub fn write_stdin_line(&mut self, line: &str) {
		match self.stdin.write_line(line) {
			Ok(())     => (),
			Err(error) => fail!("Failed to write to stdin: {}", error)
		}
	}
}

impl Drop for Process {
	fn drop(&mut self) {
		self.kill();

		print!(
			"stdout:\n{}\n",
			self.stdout.read_to_string().unwrap());
		print!(
			"stderr:\n{}\n",
			self.stderr.read_to_string().unwrap());
	}
}

fn to_reader(pipe_opt: Option<PipeStream>) -> BufferedReader<PipeStream> {
	let pipe = match pipe_opt {
		Some(pipe) => pipe,
		None       => fail!()
	};

	BufferedReader::new(pipe)
}

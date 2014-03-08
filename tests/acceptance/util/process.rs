use std::io;


pub struct Process;

impl Process {
	pub fn start(path: &str, args: &[~str]) -> io::Process {
		match io::Process::new(path, args) {
			Ok(process) => process,
			Err(error)  => fail!("Failed to start process {}: {}", path, error)
		}
	}

	pub fn kill(process: &mut io::Process) {
		match process.signal_kill() {
			Ok(_)      => (), // nothing to do
			Err(error) => print!("ERROR Failed to kill process: {}\n", error)
		}

		print!(
			"stdout: {}\n",
			process.stdout.clone().unwrap().read_to_str().unwrap());
		print!(
			"stderr: {}\n",
			process.stderr.clone().unwrap().read_to_str().unwrap());
	}
}

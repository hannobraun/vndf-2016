use std::io;


pub struct Process;

impl Process {
	pub fn start(path: &str, args: &[~str]) -> io::Process {
		match io::Process::new(path, args) {
			Ok(process) => process,
			Err(error)  => fail!("Failed to start process {}: {}", path, error)
		}
	}
}

use std::io::{BufferedReader, Process};

#[test]
fn it_should_connect_and_receive_updates() {
	let mut core_service = start_process("output/bin/vndf-core-service", []);
	let mut client_core  = start_process(
		"output/vndf-client/vndf-client-core",
		[~"localhost"]);

	let     stdout = client_core.stdout.clone().unwrap();
	let mut reader = BufferedReader::new(stdout);

	let message = reader.read_line().unwrap();
	if !message.starts_with("UPDATE ") {
		fail!("Unpexected message: {}", message);
	}

	kill_process(&mut core_service);
	kill_process(&mut client_core);
}

fn start_process(path: &str, args: &[~str]) -> Process {
	match Process::new(path, args) {
		Ok(process) => process,
		Err(error)  => fail!("Failed to start process {}: {}", path, error)
	}
}

fn kill_process(process: &mut Process) {
	match process.signal_kill() {
		Ok(_)      => (), // nothing to do
		Err(error) => print!("ERROR Failed to kill process: {}\n", error)
	}

	print!(
		"stdout: {}\n", process.stdout.clone().unwrap().read_to_str().unwrap());
	print!(
		"stderr: {}\n", process.stderr.clone().unwrap().read_to_str().unwrap());
}

use std::from_str;
use std::io::{BufferedReader, Process};

mod util;

#[test]
fn it_should_connect_and_receive_updates() {
	let mut core_service = util::Process::start(
		"output/bin/vndf-core-service", []);
	let mut client_core  = util::Process::start(
		"output/bin/vndf-client-core",
		[~"localhost"]);

	let     stdout = client_core.stdout.clone().unwrap();
	let mut reader = BufferedReader::new(stdout);

	let message        = reader.read_line().unwrap();
	let words: ~[&str] = message.words().collect();

	assert!(words[0] == "UPDATE");

	let id: Option<uint> = from_str::from_str(words[1]);
	let x : Option<f64>  = from_str::from_str(words[2]);
	let y : Option<f64>  = from_str::from_str(words[3]);
	let z : Option<f64>  = from_str::from_str(words[4]);
	assert!(id != None);
	assert!(x != None);
	assert!(y != None);
	assert!(z != None);

	kill_process(&mut core_service);
	kill_process(&mut client_core);
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

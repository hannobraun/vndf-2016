use std::from_str;

use util::Process;

mod util;

#[test]
fn it_should_connect_and_receive_updates() {
	let mut core_service = Process::start(
		"output/bin/vndf-core-service", []);
	let mut client_core  = Process::start(
		"output/bin/vndf-client-core", [~"localhost"]);

	let message        = client_core.read_stdout_line();
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

	core_service.kill();
	client_core.kill();
}

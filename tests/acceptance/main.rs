extern crate common;


use util::{Process, Update};


mod util;

#[test]
fn it_should_connect_and_receive_updates() {
	let mut core_service = Process::start(
		"output/bin/vndf-core-service", []);
	let mut client_core  = Process::start(
		"output/bin/vndf-client-core", [~"localhost"]);

	let message = client_core.read_stdout_line();
	assert_is_update(message);

	core_service.kill();
	client_core.kill();
}

fn assert_is_update(message: ~str) {
	let update = Update::from_str(message);
	assert!(update != None);
}

use util::{Process, Update};


#[test]
fn it_should_connect_and_receive_updates() {
	let mut core_service = Process::start(
		"output/bin/vndf-core-service", []);
	let mut client_core  = Process::start(
		"output/bin/vndf-client-core", [~"localhost"]);

	let message = client_core.read_stdout_line();
	let update = Update::from_str(message);
	assert!(update != None);

	core_service.kill();
	client_core.kill();
}

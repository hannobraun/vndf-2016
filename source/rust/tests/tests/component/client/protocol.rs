use shared::shared::protocol::{
	client,
	server,
};
use tests::{
	rc,
	mock,
};


#[test]
fn it_should_display_an_error_if_connection_to_server_is_lost() {
	let mut server = mock::Server::start();
	let mut client = rc::Client::start(server.port());

	let event = server.wait_until(|event|
		if let &mut Some((_, ref event)) = event {
			event == &client::Event::Public(client::event::Public::Login)
		}
		else {
			false
		}
	);

	let (address, _) = if let Some(event) = event {
		event
	}
	else {
		panic!("Expected event");
	};

	server.send(address, server::Event::Heartbeat);

	client.wait_until(|frame| frame.message.is_error());
}

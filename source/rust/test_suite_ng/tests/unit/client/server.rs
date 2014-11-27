use client_ng::Server;


#[test]
fn it_should_not_block_forever_on_receive() {
	// We don't actually need a server running on that address unless we
	// explicitely send something.
	Server::new(("localhost", 34481));

	// Server is dropped immediately, but its task won't notice if it blocks on
	// the receive operation forever.
}

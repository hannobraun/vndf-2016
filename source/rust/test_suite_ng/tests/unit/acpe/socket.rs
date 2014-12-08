use acpe::network::Socket;


#[test]
fn it_should_not_block_forever_on_receive() {
	// TODO: Use random_port using acceptance::random_port. This is complicated
	//       by the fact that the acceptance name is shadowed by the module for
	//       acceptance tests. Maybe it's finally time to think about a real
	//       name for acceptance-rs?
	Socket::new(34481);

	// Socket is dropped immediately, but its task won't notice if it blocks on
	// the receive operation forever.
}

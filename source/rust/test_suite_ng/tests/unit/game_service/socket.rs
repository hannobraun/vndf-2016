use game_service_ng::Socket;


#[test]
fn it_should_not_block_forever_on_receive() {
	Socket::new(34481);

	// Socket is dropped immediately, but its task won't notice if it blocks on
	// the receive operation forever.
}

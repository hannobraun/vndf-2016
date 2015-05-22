use common::util::is_point_on_line;
use test_suite::rc;


#[test]
fn it_should_send_navigation_data() {
	let     game_service = rc::GameService::start();
	let mut client       = rc::Client::start(game_service.port());

	let frame_1 = client.frame();

	let frame_1 = client.wait_until(|frame|
		frame.position != frame_1.position
	);
	let frame_2 = client.wait_until(|frame|
		frame.position != frame_1.position
	);

	assert!(is_point_on_line(
		frame_2.position,
		frame_1.position, frame_1.velocity,
	));
}

use acceptance::TreeBuilder;
use hyper::status;
use hyper::header::common::location::Location;

use infra::Rocks;


#[test]
fn it_should_return_a_custom_response() {
	let test_response = "
		code     = 301
		location = \"/other-directory\"
	";

	let tree = TreeBuilder::new()
		.with_file("source/test.response", test_response)
		.build();

	let rocks = Rocks::start(tree);

	let response = rocks.get("/test");

	assert_eq!(status::MovedPermanently, response.status);
	assert_eq!(
		&Location(format!("http://localhost:{}/other-directory", rocks.port)),
		response.headers.get::<Location>().unwrap()
	);
}

#[test]
fn it_should_return_a_custom_response_for_a_directory() {
	let test_response = "
		code     = 301
		location = \"/other-directory\"
	";

	let tree = TreeBuilder::new()
		.with_file("source/test/.response", test_response)
		.build();

	let rocks = Rocks::start(tree);

	let response = rocks.get("/test");

	assert_eq!(status::MovedPermanently, response.status);
	assert_eq!(
		&Location(format!("http://localhost:{}/other-directory", rocks.port)),
		response.headers.get::<Location>().unwrap()
	);
}

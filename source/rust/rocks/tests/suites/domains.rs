use acceptance::TreeBuilder;
use hyper::status;
use hyper::header::common::Host;

use infra::Rocks;


#[test]
fn it_should_use_domain_folder_according_to_host_header() {
	let file_contents = "this is a file";
	let tree = TreeBuilder::new()
		.with_file("www.example.com/public/test", file_contents)
		.build();

	let rocks = Rocks::start(tree);
	let mut response = rocks
		.request("/test")
		.with_header(Host(format!("www.example.com:{}", rocks.port)))
		.send();

	assert_eq!(status::Ok, response.status);
	assert_eq!(
		file_contents.to_string(),
		response.read_to_string().unwrap()
	);
}
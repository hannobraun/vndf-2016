use std::io::File;
use std::io::fs::PathExtensions;

use iron::{
	status,
	Handler,
	IronResult,
	Request,
	Response,
};
use static_file::StaticWithCache;
use toml::Parser;


pub struct RocksHandler {
	root_path: Path,
}

impl RocksHandler {
	pub fn new(root_path: Path) -> RocksHandler {
		let root_path = root_path.join("localhost");

		RocksHandler {
			root_path: root_path,
		}
	}
}

impl Handler for RocksHandler {
	fn call(&self, request: &mut Request) -> IronResult<Response> {
		let public_path = self.root_path.join("public");
		let source_path = self.root_path.join("source");

		let static_handler = StaticWithCache::new(public_path);

		match static_handler.call(request) {
			Ok(response) => {
				match response.status {
					Some(status::NotFound) => {
						match run_plugin(&source_path, request) {
							Some(result) => result,
							None         => Ok(response),
						}
					},

					_ => Ok(response)
				}
			},
			Err(error) => Err(error),
		}

	}
}


fn run_plugin(
	source_path: &Path,
	request    : &Request
) -> Option<IronResult<Response>> {
	let requested_path =
		source_path.join_many(request.url.path.as_slice());

	let path_candidates = [
		requested_path.with_filename(
			format!("{}.response", requested_path.filename_str().unwrap()),
		),
		requested_path.join(".response"),
	];

	let mut source_file = None;
	for candidate in path_candidates.iter() {
		if candidate.exists() {
			source_file = Some(candidate);
		}
	}

	let source_file = match source_file {
		Some(path) => path,
		None       => return None,
	};

	let data =
		Parser::new(
			File::open(source_file).read_to_string().unwrap().as_slice()
		)
		.parse()
		.unwrap();

	let code     = data["code".to_string()].as_integer().unwrap();
	let location = data["location".to_string()].as_str().unwrap();

	let mut url = request.url.clone();
	url.path = location
		.split('/')
		.filter(|&s| s != "")
		.map(|s| s.to_string())
		.collect();

	let response = Response::redirect(
		FromPrimitive::from_i64(code).unwrap(),
		url,
	);

	Some(Ok(response))
}

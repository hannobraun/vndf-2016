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
	static_handler: StaticWithCache,
	source_path   : Path,
}

impl RocksHandler {
	pub fn new(root_path: Path) -> RocksHandler {
		let root_path   = root_path.join("localhost");
		let public_path = root_path.join("public");

		RocksHandler {
			static_handler: StaticWithCache::new(public_path),
			source_path   : root_path.join("source"),
		}
	}

	fn run_plugin(&self, request: &Request) -> Option<IronResult<Response>> {
		let requested_path =
			self.source_path.join_many(request.url.path.as_slice());

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
}

impl Handler for RocksHandler {
	fn call(&self, request: &mut Request) -> IronResult<Response> {
		match self.static_handler.call(request) {
			Ok(response) => {
				match response.status {
					Some(status::NotFound) => {
						match self.run_plugin(request) {
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

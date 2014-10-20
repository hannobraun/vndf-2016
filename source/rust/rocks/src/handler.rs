use iron::{
	status,
	Handler,
	IronResult,
	Request,
	Response,
};
use static_file::StaticWithCache;


pub struct RocksHandler {
	static_handler: StaticWithCache,
}

impl RocksHandler {
	pub fn new(root_path: Path) -> RocksHandler {
		let public_path = root_path.join("public");

		RocksHandler {
			static_handler: StaticWithCache::new(public_path),
		}
	}

	fn run_plugin(&self, _: &Request) -> Option<IronResult<Response>> {
		None
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

use iron::{
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
}

impl Handler for RocksHandler {
	fn call(&self, request: &mut Request) -> IronResult<Response> {
		self.static_handler.call(request)
	}
}

extern crate time;

extern crate iron;
extern crate static_file;


use std::io::net::ip::Ipv4Addr;

use iron::{
	Handler,
	Iron,
	IronResult,
	Request,
	Response,
};
use static_file::StaticWithCache;


struct RocksHandler {
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


fn main() {
	Iron::new(
		RocksHandler::new(
			Path::new("/home/hanno/Projects/vndf/source/http")
		)
	)
	.listen(Ipv4Addr(127, 0, 0, 1), 3000);

	print!("Listening on port 3000\n");
}

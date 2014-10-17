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
use static_file::Static;


struct Cache<H: Handler> {
	cached_handler: H,
}

impl<H: Handler> Cache<H> {
	pub fn new(cached_handler: H) -> Cache<H> {
		Cache {
			cached_handler: cached_handler,
		}
	}
}

impl<H: Handler> Handler for Cache<H> {
	fn call(&self, request: &mut Request) -> IronResult<Response> {
		match self.cached_handler.call(request) {
			Ok(mut response) => {
				response.headers.cache_control =
					Some("public, max-age=31536000".to_string());
				response.headers.last_modified = Some(time::now_utc());

				Ok(response)
			},
			Err(error) => Err(error),
		}
	}
}


fn main() {
	Iron::new(
		Cache::new(
			Static::new(
				Path::new("/home/hanno/Projects/vndf/source/http/public")
			)
		)
	)
	.listen(Ipv4Addr(127, 0, 0, 1), 3000);

	print!("Listening on port 3000\n");
}

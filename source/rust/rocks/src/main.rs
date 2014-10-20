extern crate time;

extern crate iron;
extern crate static_file;


use std::io::net::ip::Ipv4Addr;

use iron::Iron;
use static_file::StaticWithCache;


fn main() {
	Iron::new(
		StaticWithCache::new(
			Path::new("/home/hanno/Projects/vndf/source/http/public")
		)
	)
	.listen(Ipv4Addr(127, 0, 0, 1), 3000);

	print!("Listening on port 3000\n");
}

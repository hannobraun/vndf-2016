extern crate iron;
extern crate static_file;


use std::io::net::ip::Ipv4Addr;

use iron::Iron;
use static_file::Static;


fn main() {
	Iron::new(
		Static::new(Path::new("/home/hanno/Projects/vndf/source/http/public"))
	)
	.listen(Ipv4Addr(127, 0, 0, 1), 3000);
}

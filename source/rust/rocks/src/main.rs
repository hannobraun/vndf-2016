extern crate getopts;
extern crate time;

extern crate iron;
extern crate static_file;


use std::io::net::ip::Ipv4Addr;

use iron::Iron;

use handler::RocksHandler;


mod args;
mod handler;


fn main() {
	let root_path = args::parse();

	Iron::new(
		RocksHandler::new(Path::new(root_path))
	)
	.listen(Ipv4Addr(127, 0, 0, 1), 3000);

	print!("Listening on port 3000\n");
}

extern crate getopts;
extern crate time;

extern crate iron;
extern crate static_file;
extern crate toml;


use std::io::net::ip::Ipv4Addr;

use iron::Iron;

use args::Args;
use handler::RocksHandler;


mod args;
mod handler;


fn main() {
	let args = Args::parse();

	Iron::new(
		RocksHandler::new(args.root_path)
	)
	.listen(Ipv4Addr(127, 0, 0, 1), args.port);

	print!("Listening on port {}\n", args.port);
}

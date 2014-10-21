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
		RocksHandler::new(Path::new(args.root_path))
	)
	.listen(Ipv4Addr(127, 0, 0, 1), 3000);

	print!("Listening on port 3000\n");
}

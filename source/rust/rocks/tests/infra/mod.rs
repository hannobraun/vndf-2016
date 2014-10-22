use std::io::net::ip::Port;

use acceptance::{
	random_port,
	Process,
	Tree,
};
use hyper::Url;
use hyper::client::{
	Request,
	Response,
};


pub struct Rocks {
	pub port: Port,

	_process: Process,
}

impl Rocks {
	pub fn start(tree: Tree) -> Rocks {
		let port = random_port(8000, 9000);

		let mut process = Process::start("rocks", [
			format!("--port={}", port).as_slice(),
			format!("--root={}", tree.root().display()).as_slice(),
		]);

		process.read_stdout_line(); // Make sure it's ready

		Rocks {
			port    : port,
			_process: process,
		}
	}

	pub fn request(&self, path: &str) -> RocksRequest {
		RocksRequest::new(self.port, path)
	}
}


struct RocksRequest {
	url: Url,
}

impl RocksRequest {
	fn new(port: Port, path: &str) -> RocksRequest {
		let url =
			Url::parse(
				format!("http://localhost:{}{}", port, path).as_slice()
			)
			.unwrap();

		RocksRequest {
			url: url,
		}
	}

	pub fn send(self) -> Response {
		Request::get(self.url)
			.unwrap_or_else(|e| fail!("get failed: {}", e))
			.start()
			.unwrap_or_else(|e| fail!("start failed: {}", e))
			.send()
			.unwrap_or_else(|e| fail!("send failed: {}", e))
	}
}
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
use hyper::header::Header;
use hyper::net::Fresh;


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
	request: Request<Fresh>,
}

impl RocksRequest {
	fn new(port: Port, path: &str) -> RocksRequest {
		let url =
			Url::parse(
				format!("http://localhost:{}{}", port, path).as_slice()
			)
			.unwrap();

		let request = Request::get(url.clone())
			.unwrap_or_else(|e| fail!("get failed: {}", e));

		RocksRequest {
			request: request,
		}
	}

	pub fn with_header<H: Header>(mut self, header: H) -> RocksRequest {
		self.request.headers_mut().set(header);
		self
	}

	pub fn send(self) -> Response {
		self.request
			.start()
			.unwrap_or_else(|e| fail!("start failed: {}", e))
			.send()
			.unwrap_or_else(|e| fail!("send failed: {}", e))
	}
}
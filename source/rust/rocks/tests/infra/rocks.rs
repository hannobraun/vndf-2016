use std::io::net::ip::Port;
use std::io::timer::sleep;
use std::time::Duration;

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
use hyper::header::{
	Header,
	HeaderFormat,
};
use hyper::net::Fresh;


pub struct Rocks {
	pub port: Port,

	_process: Process,
}

impl Rocks {
	pub fn start(tree: Tree) -> Rocks {
		let port = random_port(8000, 9000);

		let process = Process::start("rocks", [
			format!("--port={}", port).as_slice(),
			format!("--root={}", tree.root().display()).as_slice(),
		]);

		// Sleep to make sure that Rocks is listening. A better solution would
		// be for Rocks to print a line once it is listening, but that is not
		// possible at the moment. The Iron::listen method returns after it has
		// spawned the listening thread, not once that thread is actually
		// listening. This is an issue best fixed in Iron.
		sleep(Duration::milliseconds(100));

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
			.unwrap_or_else(|e| panic!("get failed: {}", e));

		RocksRequest {
			request: request,
		}
	}

	pub fn with_header<H: Header + HeaderFormat>(
		mut self,
		header: H
	) -> RocksRequest {
		self.request.headers_mut().set(header);
		self
	}

	pub fn send(self) -> Response {
		self.request
			.start()
			.unwrap_or_else(|e| panic!("start failed: {}", e))
			.send()
			.unwrap_or_else(|e| panic!("send failed: {}", e))
	}
}

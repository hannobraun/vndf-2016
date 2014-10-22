use std::io::net::ip::Port;

use acceptance::{
	random_port,
	Process,
	Tree,
};
use hyper::client::{
	Request,
	Response,
};
use hyper::Url;


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

	pub fn get(&self, path: &str) -> Response {
		let url = format!("http://localhost:{}{}", self.port, path);

		Request::get(Url::parse(url.as_slice()).unwrap())
			.unwrap_or_else(|e| fail!("get failed: {}", e))
			.start()
			.unwrap_or_else(|e| fail!("start failed: {}", e))
			.send()
			.unwrap_or_else(|e| fail!("send failed: {}", e))
	}
}
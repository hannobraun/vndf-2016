use std::io::net::ip::Port;

use client_ng::Frame;


pub struct Client;

impl Client {
	pub fn start(_port: Port) -> Client {
		Client
	}

	pub fn command(&mut self, _command: &str) {

	}

	pub fn wait_while(&mut self, _condition: |Frame| -> bool) -> Frame {
		Frame {
			broadcasts: vec!["This is a broadcast.".to_string()],
		}
	}
}

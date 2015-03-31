use std::net::{
	SocketAddr,
	TcpListener,
};
use std::sync::mpsc::{
	channel,
	Receiver,
};
use std::sync::mpsc::TryRecvError::{
	Disconnected,
	Empty,
};
use std::thread::spawn;
use std::vec::Drain;

use rustc_serialize::Decodable;

use super::Connection;


pub struct Acceptor<R: Send> {
	receiver   : Receiver<(SocketAddr, Connection<R>)>,
	connections: Vec<(SocketAddr, Connection<R>)>,
}

impl<R> Acceptor<R> where R: Decodable + Send + 'static {
	pub fn new(port: u16) -> Acceptor<R> {
		let (sender, receiver) = channel();

		spawn(move || {
			let listener = match TcpListener::bind(&("0.0.0.0", port)) {
				Ok(listener) => listener,
				Err(error)   => panic!("Error binding listener: {}", error),
			};

			print!("Listening on port {}\n", port);

			loop {
				let (stream, address) = match listener.accept() {
					Ok(result) => result,
					Err(error) => panic!("Error accepting stream: {}", error),
				};

				let connection = Connection::from_stream(stream);

				if let Err(_) = sender.send((address, connection)) {
					panic!("Acceptor channel disconnected");
				}
			}
		});

		Acceptor {
			connections: Vec::new(),
			receiver   : receiver,
		}
	}

	pub fn accept(&mut self) -> Drain<(SocketAddr, Connection<R>)> {
		loop {
			match self.receiver.try_recv() {
				Ok(connection) =>
					self.connections.push(connection),
				Err(error) => match error {
					Empty        => break,
					Disconnected => panic!("Acceptor channel disconnected"),
				},
			}
		}

		self.connections.drain()
	}
}

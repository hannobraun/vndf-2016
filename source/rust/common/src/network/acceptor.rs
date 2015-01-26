use std::io::Acceptor as IoAcceptor;
use std::io::Listener;
use std::io::net::ip::{
	Port,
	SocketAddr,
};
use std::io::net::tcp::{
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
use std::thread::Thread;
use std::vec::Drain;

use rustc_serialize::Decodable;

use super::Connection;


pub struct Acceptor<R> {
	receiver   : Receiver<(SocketAddr, Connection<R>)>,
	connections: Vec<(SocketAddr, Connection<R>)>,
}

impl<R> Acceptor<R> where R: Decodable + Send {
	pub fn new(port: Port) -> Acceptor<R> {
		let (sender, receiver) = channel();
		Thread::spawn(move || {
			let listener = match TcpListener::bind(("0.0.0.0", port)) {
				Ok(listener) => listener,
				Err(error)   => panic!("Error binding listener: {}", error),
			};
			let mut acceptor = match listener.listen() {
				Ok(acceptor) => acceptor,
				Err(error)   => panic!("Error creating acceptor: {}", error),
			};

			loop {
				let mut stream = match acceptor.accept() {
					Ok(stream) => stream,
					Err(error) => panic!("Error accepting stream: {}", error),
				};
				let address = match stream.peer_name() {
					Ok(address) => address,
					Err(error)  => panic!("Error getting address: {}", error),
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

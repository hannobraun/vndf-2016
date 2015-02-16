use std::io::prelude::*;
use std::io::{
	self,
	BufReader,
};
use std::net::{
	SocketAddr,
	TcpStream,
	ToSocketAddrs,
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

use rustc_serialize::{
	json,
	Decodable,
	Encodable,
};


pub struct Connection<R> {
	events  : Vec<R>,
	stream  : TcpStream,
	receiver: Receiver<R>,
}

impl<R> Connection<R> where R: Decodable + Send {
	pub fn new<T: ToSocketAddrs>(to_address: T) -> Connection<R> {
		let addresses = to_address.to_socket_addrs();
		let stream = match TcpStream::connect(&to_address) {
			Ok(stream) => stream,
			Err(error) => panic!(
				"Error connecting to {:?}: {}",
				// TODO: This is crap. It wouldn't have to be crap, if Debug was
				//       implemented for std::option::IntoIter, so maybe I
				//       should do that.
				addresses.unwrap().collect::<Vec<SocketAddr>>(), error,
			),
		};

		Connection::from_stream(stream)
	}

	pub fn from_stream(stream: TcpStream) -> Connection<R> {
		let (sender, receiver) = channel();

		let stream_2 = match stream.try_clone() {
			Ok(stream) => stream,
			Err(error) => panic!("Failed to clone stream: {}", error),
		};

		let connection = Connection {
			events  : Vec::new(),
			stream  : stream_2,
			receiver: receiver,
		};

		Thread::spawn(move || {
			let mut reader = BufReader::new(stream);

			loop {
				let mut line = String::new();
				if let Err(error) = reader.read_line(&mut line) {
					print!("Error reading line: {}\n", error);
					break;
				}

				// TODO: Before porting to std::net, there would be no zero-
				//       length reads. Instead, the EndOfFile error would be
				//       returned. This happened during normal operation, but
				//       also when the connection was closed.
				//       To handle all situations somewhat correctly, there was
				//       code here to count how many EndOfFile errors happened
				//       in a row, and assumed the connection was closed when it
				//       happened too often.
				//       I'm not sure what the situation is with std::net. I've
				//       written the following code with the assumption that a
				//       closed connection will eventually result in an error.
				//       If this assumption is false (as it was with the old
				//       API), this needs some special handling.
				if line.len() == 0 {
					// Nothing received for now, start loop from the top to try
					// again.
					continue;
				}

				let event = match json::decode(line.as_slice()) {
					Ok(event)  => event,
					Err(error) => {
						print!("Error decoding \"{}\": {}\n", line, error);
						continue;
					},
				};

				if let Err(_) = sender.send(event) {
					panic!("Connection channel disconnected");
				}
			}
		});

		connection
	}

	pub fn send<Es, E>(&mut self, events: Es) -> io::Result<()>
		where
			Es: Iterator<Item=E>,
			E : Encodable,
	{
		for event in events {
			let event = match json::encode(&event) {
				Ok(event)  => event,
				Err(error) => panic!("Encoding error: {}", error),
			};

			try!(write!(&mut self.stream, "{}\n", event));
		}

		Ok(())
	}

	pub fn receive(&mut self) -> Result<Drain<R>, ()> {
		loop {
			match self.receiver.try_recv() {
				Ok(event) =>
					self.events.push(event),
				Err(error) => match error {
					Empty        => break,
					Disconnected => return Err(()),
				},
			}
		}

		Ok(self.events.drain())
	}
}

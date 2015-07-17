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
	Sender,
};
use std::sync::mpsc::TryRecvError::{
	Disconnected,
	Empty,
};
use std::thread::spawn;
use std::vec::Drain;

use rustc_serialize::{
	json,
	Decodable,
	Encodable,
};


pub struct Connection<R: Send> {
	events  : Vec<R>,
	stream  : TcpStream,
	messages: Receiver<R>,
	errors  : Sender<()>,
}

impl<R> Connection<R> where R: Decodable + Send + 'static {
	pub fn new<T: ToSocketAddrs>(to_address: T) -> Connection<R> {
		let addresses = to_address.to_socket_addrs();
		let stream = match TcpStream::connect(&to_address) {
			Ok(stream) => stream,
			Err(error) => panic!(
				"Error connecting to {:?}: {}",
				addresses.unwrap().collect::<Vec<SocketAddr>>(), error,
			),
		};

		Connection::from_stream(stream)
	}

	pub fn from_stream(stream: TcpStream) -> Connection<R> {
		let (messages_sender, messages_receiver) = channel();
		let (error_sender   , error_receiver   ) = channel();

		let stream_2 = match stream.try_clone() {
			Ok(stream) => stream,
			Err(error) => panic!("Failed to clone stream: {}", error),
		};

		spawn(move || {
			// TODO: I've seen the following error here once:
			//       "Error determining peer address: Transport endpoint is not
			//       connected (os error 107)".
			//       This looks like a legitimate error that could be handled
			//       better than by panicking.
			let address = stream.peer_addr().unwrap_or_else(|e|
				panic!("Error determining peer address: {}", e)
			);

			let mut reader = BufReader::new(stream);
			let mut line   = String::new();

			loop {
				trace!("Start connection loop iteration: {}", address);

				line.clear();
				if let Err(error) = reader.read_line(&mut line) {
					print!("Error reading line: {}\n", error);
					break;
				}

				if line.len() > 0 {
					let event = match json::decode(line.as_ref()) {
						Ok(event)  => event,
						Err(error) => {
							print!("Error decoding \"{}\": {}\n", line, error);
							continue;
						},
					};

					if let Err(_) = messages_sender.send(event) {
						// The receiver has been dropped, which means this
						// connection is no longer needed. Time to quietly die.
						break;
					}
				}

				match error_receiver.try_recv() {
					Ok(()) =>
						// An error has occured while sending. Time to die.
						break,
					Err(error) => match error {
						Empty        => continue,
						Disconnected => break,
					},
				}
			}
		});

		Connection {
			events  : Vec::new(),
			stream  : stream_2,
			messages: messages_receiver,
			errors  : error_sender,
		}
	}

	pub fn send<Events, Event>(&mut self, events: Events) -> io::Result<()>
		where
			Events: Iterator<Item=Event>,
			Event : Encodable,
	{
		for event in events {
			let event = match json::encode(&event) {
				Ok(event)  => event,
				Err(error) => panic!("Encoding error: {}", error),
			};

			match write!(&mut self.stream, "{}\n", event) {
				Ok(()) =>
					(),
				Err(error) => {
					if let Err(_) = self.errors.send(()) {
						// Nothing to do. We're telling the receive thread to
						// die, but this error can only mean that it is already
						// dead.
					}

					return Err(error);
				},
			}
		}

		Ok(())
	}

	pub fn receive(&mut self) -> Result<Drain<R>, ()> {
		loop {
			match self.messages.try_recv() {
				Ok(message) =>
					self.events.push(message),
				Err(error) => match error {
					Empty        => break,
					Disconnected => return Err(()),
				},
			}
		}

		Ok(self.events.drain(..))
	}
}

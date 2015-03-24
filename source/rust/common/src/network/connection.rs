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
use std::thread::spawn;
use std::vec::Drain;

use rustc_serialize::{
	json,
	Decodable,
	Encodable,
};


pub struct Connection<R> {
	events  : Vec<R>,
	stream  : TcpStream,
	messages: Receiver<R>,
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

		let stream_2 = match stream.try_clone() {
			Ok(stream) => stream,
			Err(error) => panic!("Failed to clone stream: {}", error),
		};

		spawn(move || {
			let mut reader     = BufReader::new(stream);
			let mut line       = String::new();
			let mut zero_reads = 0;

			loop {
				line.clear();
				if let Err(error) = reader.read_line(&mut line) {
					print!("Error reading line: {}\n", error);
					break;
				}

				// A read of length zero can mean one of two things:
				// - No data available for now. I don't know why it still
				//   returns (shouldn't it block?), but that's what it does, so
				//   we need to handle it.
				// - Connection is closed. I don't know how we would find out
				//   reliably that this is the case, but simply counting the
				//   number of zero-length reads seems to work well.
				//
				// Please note that this solution doesn't need to be perfect.
				// Eventually, we'll use UDP and there will be no connection to
				// take care of, nor a per-connection thread that could go into
				// an endless loop.
				if line.len() == 0 {
					if zero_reads < 128 {
						// Nothing received for now, start loop from the top to
						// try again.
						zero_reads += 1;
						continue;
					}
					else {
						print!(
							"Too many zero-length reads. Closing connection.\n"
						);
						break;
					}
				}

				zero_reads = 0;

				let event = match json::decode(line.as_slice()) {
					Ok(event)  => event,
					Err(error) => {
						print!("Error decoding \"{}\": {}\n", line, error);
						continue;
					},
				};

				if let Err(_) = messages_sender.send(event) {
					panic!("Connection channel disconnected");
				}
			}
		});

		Connection {
			events  : Vec::new(),
			stream  : stream_2,
			messages: messages_receiver,
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

			// TODO(WNVegmhs): When an error occurs, tell the reading thread
			//                 about it. At least do that when we know the
			//                 connection has been closed (broken pipe). Sending
			//                 a notification to the receiving thread could
			//                 replace the error-counting hack in there.
			try!(write!(&mut self.stream, "{}\n", event));
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

		Ok(self.events.drain())
	}
}

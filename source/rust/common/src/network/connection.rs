use std::io::{
	BufferedReader,
	IoResult,
};
use std::io::IoErrorKind::EndOfFile;
use std::io::net::ip::ToSocketAddr;
use std::io::net::tcp::TcpStream;
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
	pub fn new<T: ToSocketAddr>(address: T) -> Connection<R> {
		let stream = match TcpStream::connect(address) {
			Ok(stream) => stream,
			Err(error) => panic!("Error connecting stream: {}", error),
		};

		Connection::from_stream(stream)
	}

	pub fn from_stream(stream: TcpStream) -> Connection<R> {
		let (sender, receiver) = channel();

		let connection = Connection {
			events  : Vec::new(),
			stream  : stream.clone(),
			receiver: receiver,
		};

		Thread::spawn(move || {
			let mut reader = BufferedReader::new(stream);

			loop {
				let event = match reader.read_line() {
					Ok(event)  => event,
					Err(error) => {
						if error.kind != EndOfFile {
							print!("Error reading line: {}\n", error);
							break;
						}
						else {
							// The end of file error regularly occurs during
							// normal operation, when nothing is available to be
							// read. I don't know why this would happen in a
							// blocking API, but it does, so we need to handle
							// it.

							// TODO(87104828): Is it possible that after a
							//                 connection is closed, we keep
							//                 getting this error? In that case,
							//                 we'll continue here and never
							//                 notice that the channel was
							//                 blocked, creating an endless
							//                 loop.
							continue;
						}
					},
				};

				let event = match json::decode(event.as_slice()) {
					Ok(event)  => event,
					Err(error) => {
						print!("Decoding error: {}\n", error);
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

	pub fn send<Es, E>(&mut self, mut events: Es) -> IoResult<()>
		where
			Es: Iterator<Item=E>,
			E : Encodable,
	{
		for event in events {
			let event = match json::encode(&event) {
				Ok(event)  => event,
				Err(error) => panic!("Encoding error: {}", error),
			};

			try!(self.stream.write_line(event.as_slice()))
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

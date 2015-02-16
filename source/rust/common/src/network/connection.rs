use std::old_io::{
	BufferedReader,
	IoResult,
};
use std::old_io::IoErrorKind::EndOfFile;
use std::old_io::net::ip::ToSocketAddr;
use std::old_io::net::tcp::TcpStream;
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
	pub fn new<T: ToSocketAddr>(to_address: T) -> Connection<R> {
		let address = to_address.to_socket_addr();
		let stream = match TcpStream::connect(to_address) {
			Ok(stream) => stream,
			Err(error) => panic!(
				"Error connecting to {:?}: {}",
				address, error,
			),
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
			let mut errors = 0;

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
							// There's one problem though: End of file also
							// occurs when the connection has been closed. My
							// theory: If it happens too many times in a row,
							// the connection is gone and this thread should
							// die.
							// This is not pretty, but probably not worth
							// putting too much effort into. The real solution
							// would be to switch to a non-blocking API once a
							// compelling one becomes available.

							if errors > 10 {
								print!("Too many end of file errors.\n");
								break;
							}
							else {
								errors += 1;
								continue;
							}
						}
					},
				};

				errors = 0;

				let event = match json::decode(event.as_slice()) {
					Ok(event)  => event,
					Err(error) => {
						print!("Error decoding \"{}\": {}\n", event, error);
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

	pub fn send<Es, E>(&mut self, events: Es) -> IoResult<()>
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

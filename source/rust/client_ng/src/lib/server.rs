use std::comm::TryRecvError;
use std::io::IoErrorKind;
use std::io::net::ip::SocketAddr;
use std::io::net::udp::UdpSocket;

use protocol_ng::{
	Action,
	Perception,
};


pub struct Server {
	receiver: Receiver<Option<String>>,
	address : SocketAddr,
	socket  : UdpSocket,
}

impl Server {
	pub fn new(address: SocketAddr) -> Server {
		let (sender, receiver) = channel();

		let mut socket = UdpSocket::bind(("0.0.0.0", 0)).unwrap();

		let socket_field = socket.clone();

		spawn(proc() {
			let mut should_run = true;
			let mut buffer     = [0u8, ..512];

			while should_run {
				socket.set_read_timeout(Some(100));
				let message = match socket.recv_from(&mut buffer) {
					Ok((len, _)) =>
						Some(
							String::from_utf8(
								buffer[.. len].to_vec()
							)
							.unwrap_or_else(|error|
								panic!(
									"Message from server is no valid UTF-8: {}",
									error
								)
							)
						),

					Err(error) => match error.kind {
						IoErrorKind::TimedOut =>
							None,
						_ =>
							panic!("Error receiving message: {}", error),
					},
				};

				match sender.send_opt(message) {
					Ok(()) => (),
					Err(_) => should_run = false, // other end disconnected
				}
			}
		});

		Server {
			receiver: receiver,
			address : address,
			socket  : socket_field,
		}
	}

	pub fn recv_from(&self) -> Option<Perception> {
		let message = match self.receiver.try_recv() {
			Ok(message) => match message {
				Some(message) => message,
				None          => return None,
			},

			Err(error) => match error {
				TryRecvError::Empty        => return None,
				TryRecvError::Disconnected => panic!("Channel disconnected"),
			}
		};

		let message =
			Perception::from_json(message.as_slice())
			.unwrap_or_else(
				|error|
					panic!(
						"Error decoding message from server: {}",
						error,
					)
			);

		Some(message)
	}

	pub fn send_to(&mut self, message: Action) {
		// TODO(83501684): We have no way of knowing, if this message actually
		//                 arrives.
		// TODO: Do we need to set a timeout here?
		self.socket.send_to(
			message.to_json().as_bytes(),
			self.address
		)
		.unwrap_or_else(|error| panic!("Error sending message: {}\n", error));
	}
}

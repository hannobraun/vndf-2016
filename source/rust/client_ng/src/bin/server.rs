use std::comm::TryRecvError;
use std::io::net::ip::SocketAddr;
use std::io::net::udp::UdpSocket;

use protocol_ng::{
	Action,
	Perception,
};


pub struct Server {
	receiver: Receiver<String>,
	address : SocketAddr,
	socket  : UdpSocket,
}

impl Server {
	pub fn new(address: SocketAddr) -> Server {
		let (sender, receiver) = channel();

		let mut socket = UdpSocket::bind(("0.0.0.0", 0)).unwrap();

		let socket_field = socket.clone();

		spawn(proc() {
			let mut buffer = [0u8, ..512];

			loop {
				let message = match socket.recv_from(&mut buffer) {
					Ok((len, _)) =>
						buffer[.. len],
					Err(error) =>
						panic!("Error receiving message: {}", error),
				};

				sender.send(String::from_utf8(message.to_vec()).unwrap());
			}
		});

		Server {
			receiver: receiver,
			address : address,
			socket  : socket_field,
		}
	}

	pub fn recv_from(&self) -> Option<Perception> {
		match self.receiver.try_recv() {
			Ok(message) => {
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
			},

			Err(error) => match error {
				TryRecvError::Empty        => None,
				TryRecvError::Disconnected => panic!("Channel disconnected"),
			}
		}
	}

	pub fn send_to(&mut self, message: Action) {
		// TODO(83501684): We have no way of knowing, if this message actually
		//                 arrives.
		self.socket.send_to(
			message.to_json().as_bytes(),
			self.address
		)
		.unwrap_or_else(|error| panic!("Error sending message: {}\n", error));
	}
}

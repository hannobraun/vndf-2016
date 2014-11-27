use std::comm::TryRecvError;
use std::io::net::ip::{
	Port,
	SocketAddr,
};
use std::io::net::udp::UdpSocket;

use protocol_ng::Action;


pub struct Socket {
	receiver  : Receiver<(Action, SocketAddr)>,
	pub socket: UdpSocket,
}

impl Socket {
	pub fn new(port: Port) -> Socket {
		let (sender, receiver) = channel();

		let mut socket       = UdpSocket::bind(("0.0.0.0", port)).unwrap();
		let     socket_field = socket.clone();

		print!("Listening on port {}\n", port);

		spawn(proc() {
			let mut buffer  = [0u8, ..512];

			loop {
				// TODO: Add receive timeout.
				match socket.recv_from(&mut buffer) {
					// TODO(83503278): Handle decoding errors.
					Ok((len, address)) => {
						let action =
							Action::from_json(
								String::from_utf8(
									buffer[.. len].to_vec()
								)
								.unwrap()
								.as_slice()
							)
							.unwrap();

						sender.send((action, address));
					},

					Err(error) => {
						print!("Error receiving data: {}\n", error);
						continue;
					},
				}
			}
		});

		Socket {
			receiver: receiver,
			socket  : socket_field,
		}
	}

	pub fn send_to(&mut self, perception: &[u8], address: SocketAddr) {
		match self.socket.send_to(perception, address) {
			Ok(())     => (),
			Err(error) =>
				print!(
					"Error sending data to {}: {}",
					address, error
				),
		}
	}

	pub fn recv_from(&self) -> Option<(Action, SocketAddr)> {
		match self.receiver.try_recv() {
			Ok((address, action)) =>
				Some((address, action)),

			Err(error) => match error {
				TryRecvError::Empty        => return None,
				TryRecvError::Disconnected => panic!("Channel disconnected"),
			}
		}
	}
}

use std::comm::TryRecvError;
use std::io::IoErrorKind;
use std::io::net::ip::{
	Port,
	SocketAddr,
};
use std::io::net::udp::UdpSocket;

use protocol_ng::Action;


pub struct Socket {
	pub sender  : SocketSender,
	    receiver: SocketReceiver,
}

impl Socket {
	pub fn new(port: Port) -> Socket {
		let socket   = UdpSocket::bind(("0.0.0.0", port)).unwrap();
		let sender   = SocketSender::new(socket.clone());
		let receiver = SocketReceiver::new(socket);

		Socket {
			sender  : sender,
			receiver: receiver,
		}
	}

	pub fn send_to(&mut self, message: &[u8], address: SocketAddr) {
		self.sender.send(message, address)
	}

	pub fn recv_from(&self) -> Option<(Action, SocketAddr)> {
		self.receiver.recv()
	}
}


#[deriving(Clone)]
pub struct SocketSender {
	socket: UdpSocket,
}

impl SocketSender {
	fn new(socket: UdpSocket) -> SocketSender {
		SocketSender {
			socket: socket,
		}
	}

	pub fn send(&mut self, message: &[u8], address: SocketAddr) {
		match self.socket.send_to(message, address) {
			Ok(())     => (),
			Err(error) =>
				print!(
					"Error sending data to {}: {}",
					address, error
				),
		}
	}
}


struct SocketReceiver {
	receiver: Receiver<Option<(Action, SocketAddr)>>,
}

impl SocketReceiver {
	fn new(mut socket: UdpSocket) -> SocketReceiver {
		let (sender, receiver) = channel();

		print!(
			"Listening on port {}\n",
			socket.socket_name().unwrap().port
		);

		spawn(proc() {
			let mut should_run = true;
			let mut buffer     = [0u8, ..512];

			while should_run {
				socket.set_read_timeout(Some(20));
				let message = match socket.recv_from(&mut buffer) {
					Ok((len, address)) => {
						let action =
							Action::from_json(
								String::from_utf8(
									buffer[.. len].to_vec()
								)
								// TODO(83503278): Handle decoding errors.
								.unwrap()
								.as_slice()
							)
							// TODO(83503278): Handle decoding errors.
							.unwrap();

						Some((action, address))
					},

					Err(error) => {
						match error.kind {
							IoErrorKind::TimedOut =>
								(),
							_ =>
								print!("Error receiving data: {}\n", error),
						}

						None
					},
				};

				match sender.send_opt(message) {
					Ok(()) => (),
					Err(_) => should_run = false,
				}
			}
		});

		SocketReceiver {
			receiver: receiver,
		}
	}

	fn recv(&self) -> Option<(Action, SocketAddr)> {
		match self.receiver.try_recv() {
			Ok(message) => message,

			Err(error) => match error {
				TryRecvError::Empty        => return None,
				TryRecvError::Disconnected => panic!("Channel disconnected"),
			}
		}
	}
}

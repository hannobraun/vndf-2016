use std::comm::TryRecvError;
use std::io::net::ip::{
	Port,
	SocketAddr,
	ToSocketAddr,
};
use std::io::net::udp::UdpSocket;

use protocol_ng::{
	Action,
	Perception,
};


pub struct Server {
	receiver: Receiver<String>,
	server  : SocketAddr,
	socket  : UdpSocket,
}

impl Server {
	pub fn new(port: Port) -> Server {
		let (sender, receiver) = channel();

		let mut socket = UdpSocket::bind(("0.0.0.0", 0)).unwrap();
		// TODO: This only works if server runs on localhost. We need an actual
		//       address here.
		let     server = ("127.0.0.1", port).to_socket_addr().unwrap();

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
			server  : server,
			socket  : socket_field,
		}
	}

	pub fn recv_from(&self) -> Option<Perception> {
		let message = match self.receiver.try_recv() {
			Ok(message) => message,

			Err(error) => match error {
				TryRecvError::Empty        => return None,
				TryRecvError::Disconnected => panic!("Channel disconnected"),
			}
		};

		// TODO: Just setting the received broadcast as the only one will not be
		//       enough.
		let perception = Perception {
			broadcasts: vec![message],
		};

		Some(perception)
	}

	pub fn send_to(&mut self, message: Action) {
		// TODO: We have no way of knowing, if this message actually arrives.
		// TODO: Replace unwrap with proper error handling.
		self.socket.send_to(message.to_json().as_bytes(), self.server).unwrap();
	}
}

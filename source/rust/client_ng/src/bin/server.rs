use std::comm::TryRecvError;
use std::io::net::ip::Port;
use std::io::net::udp::UdpSocket;


pub struct Server {
	receiver: Receiver<String>,
}

impl Server {
	pub fn new(port: Port) -> Server {
		let (sender, receiver) = channel();

		spawn(proc() {
			let mut buffer = [0u8, ..512];
			let mut socket = UdpSocket::bind(("0.0.0.0", 0)).unwrap();

			// TODO: The server has no way of distinguishing this initial
			//       message from a broadcast.
			socket.send_to(
				"Please send broadcasts.\n".as_bytes(),
				("127.0.0.1", port)
			).unwrap();

			loop {
				let message = match socket.recv_from(&mut buffer) {
					Ok((len, _)) => buffer[.. len],
					Err(error)   => panic!("Error receiving message: {}", error),
				};

				sender.send(String::from_utf8(message.to_vec()).unwrap());
			}
		});

		Server {
			receiver: receiver,
		}
	}

	pub fn recv_from(&self) -> Option<String> {
		match self.receiver.try_recv() {
			Ok(message) => Some(message),

			Err(error) => match error {
				TryRecvError::Empty        => None,
				TryRecvError::Disconnected => panic!("Channel disconnected"),
			}
		}
	}
}

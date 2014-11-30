use std::comm::TryRecvError;
use std::io::IoErrorKind;
use std::io::net::ip::{
	Port,
	SocketAddr,
};
use std::io::net::udp::UdpSocket;

use protocol_ng::{
	MAX_PACKET_SIZE,
	Action,
};


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

	pub fn recv_from(&self) -> Vec<ReceiveResult> {
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
		// Whether a UDP send blocks or just drops the packet is implementation-
		// specific. I'd say with a low timeout, we're on the safe side.
		self.socket.set_write_timeout(Some(10));
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
	receiver: Receiver<Option<ReceiveResult>>,
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
			let mut buffer     = [0u8, ..MAX_PACKET_SIZE];

			while should_run {
				socket.set_read_timeout(Some(20));
				let result = match socket.recv_from(&mut buffer) {
					Ok((len, address)) => Some(
						decode_message(buffer.as_mut_slice(), len, address)
					),

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

				match sender.send_opt(result) {
					Ok(()) => (),
					Err(_) => should_run = false,
				}
			}
		});

		SocketReceiver {
			receiver: receiver,
		}
	}

	fn recv(&self) -> Vec<ReceiveResult> {
		let mut results = Vec::new();

		loop {
			match self.receiver.try_recv() {
				Ok(result) => match result {
					Some(result) => results.push(result),
					None         => (),
				},

				Err(error) => match error {
					TryRecvError::Empty        => break,
					TryRecvError::Disconnected => panic!("Channel disconnected"),
				}
			}
		}

		results
	}
}


fn decode_message(
	buffer : &mut [u8],
	len    : uint,
	address: SocketAddr
) -> ReceiveResult {
	let message = buffer[.. len].to_vec();

	let message = match String::from_utf8(message) {
		Ok(message) =>
			message,
		Err(message) =>
			return Err((
				format!("Received invalid UTF-8 string: {}", message),
				address,
			)),
	};

	let message = match Action::from_json(message.as_slice()) {
		Ok(message) =>
			message,
		Err(error) =>
			return Err((
				format!(
					"Error decoding JSON. Error: {}; JSON: {}",
					error, message
				),
				address,
			)),
	};

	Ok((message, address))
}


pub type ReceiveResult = Result<(Action, SocketAddr), (String, SocketAddr)>;

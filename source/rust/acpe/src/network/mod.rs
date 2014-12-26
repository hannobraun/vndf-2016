use std::comm::TryRecvError;
use std::io::IoErrorKind;
use std::io::net::ip::{
	Port,
	SocketAddr,
};
use std::io::net::udp::UdpSocket;
use std::thread::Thread;

use root::MAX_PACKET_SIZE;


pub type Message = (Vec<u8>, SocketAddr);


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

	pub fn recv_from(&self) -> Vec<Message> {
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
	receiver: Receiver<Option<Message>>,
}

impl SocketReceiver {
	fn new(mut socket: UdpSocket) -> SocketReceiver {
		let (sender, receiver) = channel();

		let guard = Thread::spawn(move || {
			let mut should_run = true;
			let mut buffer     = [0u8, ..MAX_PACKET_SIZE];

			while should_run {
				socket.set_read_timeout(Some(20));
				let result = match socket.recv_from(&mut buffer) {
					Ok((len, address)) =>
						Some((buffer[.. len].to_vec(), address)),
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

		guard.detach();

		SocketReceiver {
			receiver: receiver,
		}
	}

	fn recv(&self) -> Vec<Message> {
		let mut results = Vec::new();

		loop {
			match self.receiver.try_recv() {
				Ok(result) => match result {
					Some((vec, address)) =>
						results.push((vec, address)),
					None =>
						(),
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


#[cfg(test)]
mod test {
	use std::io::net::ip::Port;
	use std::rand::random;

	use network::Socket;


	#[test]
	fn it_should_not_block_forever_on_receive() {
		Socket::new(random::<Port>() % 10000 + 40000);

		// Socket is dropped immediately, but its task won't notice if it blocks
		// on the receive operation forever.
	}
}

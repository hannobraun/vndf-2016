use std::io::net::ip::{
	Port,
	SocketAddr,
};
use std::io::net::udp::UdpSocket;


pub struct Socket {
	pub socket: UdpSocket,
}

impl Socket {
	pub fn new(port: Port) -> Socket {
		Socket {
			socket: UdpSocket::bind(("0.0.0.0", port)).unwrap(),
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
}

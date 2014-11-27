use std::io::net::ip::Port;
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
}

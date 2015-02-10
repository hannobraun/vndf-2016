use std::old_io::net::ip::Port;

use rand::random;


pub fn random_port(min: Port, max: Port) -> Port {
	random() % (max - min) + min
}

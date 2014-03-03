extern crate common;

pub mod args;
pub mod net;
pub mod protocol;


struct ProtocolHandler;

impl protocol::Handler for ProtocolHandler {
	fn update_ship(&mut self, id: int, x: f64, y: f64) {
		print!("UPDATE {} {} {}\n", id, x, y);
	}

	fn remove_ship(&mut self, id: int) {
		print!("REMOVE {}\n", id);
	}
}


fn main() {
	let     server_address = args::get_server_address();
	let     socket_fd      = net::connect(server_address, ~"34481");
	let mut connection     = protocol::init(socket_fd);

	let mut handler = ProtocolHandler;

	loop {
		protocol::receive_positions(
			&mut connection,
			&mut handler);
	}
}

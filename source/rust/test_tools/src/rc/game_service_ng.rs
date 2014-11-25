use std::io::net::ip::Port;


pub struct GameService;

impl GameService {
	pub fn start() -> GameService {
		GameService
	}

	pub fn port(&self) -> Port {
		0
	}
}

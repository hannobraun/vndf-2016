extern crate time;

extern crate acceptance;
extern crate client_ng;


pub use self::rc::client::Client;
pub use self::rc::gameservice::GameService;


pub mod rc {
	pub mod client;
	pub mod game_service;
}

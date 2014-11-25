extern crate acceptance;

extern crate client_ng;
extern crate game;
extern crate game_service;
extern crate net;
extern crate platform;
extern crate protocol;
extern crate rustecs;


pub use self::mock::gameservice::MockGameService;
pub use self::rc::client::Client;
pub use self::rc::gameservice::GameService;


pub mod mock {
	pub mod gameservice;
}
pub mod rc {
	pub mod client;
	pub mod client_ng;
	pub mod gameservice;
	pub mod game_service_ng;
}

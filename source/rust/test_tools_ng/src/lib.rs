extern crate time;

extern crate acceptance;
extern crate client_ng;
extern crate protocol_ng;


pub use self::mock::client::Client as MockClient;
pub use self::rc::client::Client;
pub use self::rc::game_service::GameService;


pub mod mock {
	pub mod client;
}
pub mod rc {
	pub mod client;
	pub mod game_service;
}

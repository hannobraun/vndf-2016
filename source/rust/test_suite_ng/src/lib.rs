#![allow(unstable)]


extern crate time;

extern crate acceptance;

extern crate client;
extern crate common;
extern crate game_service;


pub use self::mock::client::Client as MockClient;
pub use self::mock::game_service::GameService as MockGameService;
pub use self::rc::client::Client;
pub use self::rc::game_service::GameService;


pub mod mock {
	pub mod client;
	pub mod game_service;
}
pub mod rc {
	pub mod client;
	pub mod game_service;
}

extern crate game;
extern crate net;
extern crate platform;
extern crate protocol;
extern crate rustecs;
extern crate test_infra;


pub use self::mock::gameservice::MockGameService;
pub use self::rc::client::Client;
pub use self::rc::gameservice::GameService;


pub mod mock {
	pub mod gameservice;
}
pub mod rc {
	pub mod client;
	pub mod gameservice;
}

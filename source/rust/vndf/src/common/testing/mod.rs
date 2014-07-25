pub use self::generic::process::Process;
pub use self::specific::mock::gameservice::MockGameService;
pub use self::specific::rc::client::Client;
pub use self::specific::rc::gameservice::GameService;

mod generic {
	pub mod macros;
	pub mod process;
}
mod specific {
	pub mod mock {
		pub mod gameservice;
	}
	pub mod rc {
		pub mod client;
		pub mod gameservice;
	}
}

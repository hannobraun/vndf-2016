#![feature(slicing_syntax)]
// TODO: This is required to derive RustcEncodable/RustcDecodable. I'm not sure
//       what's up there, but I do know that this feature gate will be removed
//       soon, so some other fix is required.
#![feature(old_orphan_check)]


extern crate "rustc-serialize" as rustc_serialize;


pub use constants::MAX_PACKET_SIZE;


pub mod network;
pub mod protocol;


mod constants {
	pub const MAX_PACKET_SIZE: uint = 512;

	pub const DESTROY: &'static str = "DESTROY";
	pub const UPDATE : &'static str = "UPDATE";
}

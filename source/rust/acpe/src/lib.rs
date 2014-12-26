#![feature(slicing_syntax)]


extern crate "rustc-serialize" as rustc_serialize;


pub use root::MAX_PACKET_SIZE;


pub mod network;
pub mod protocol;


mod root {
	pub const MAX_PACKET_SIZE: uint = 512;
}

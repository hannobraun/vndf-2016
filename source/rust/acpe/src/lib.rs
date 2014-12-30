#![feature(slicing_syntax)]


extern crate "rustc-serialize" as rustc_serialize;


pub use root::{
	DESTROY,
	HEADER,
	MAX_PACKET_SIZE,
	UPDATE,
};


pub mod network;
pub mod protocol;


mod root {
	pub const MAX_PACKET_SIZE: uint = 512;

	pub const HEADER : &'static str = "HEADER";
	pub const DESTROY: &'static str = "DESTROY";
	pub const UPDATE : &'static str = "UPDATE";
}

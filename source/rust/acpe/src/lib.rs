#![feature(slicing_syntax)]


pub use root::MAX_PACKET_SIZE;


pub mod network;
pub mod protocol;


mod root {
	pub const MAX_PACKET_SIZE: uint = 512;
}

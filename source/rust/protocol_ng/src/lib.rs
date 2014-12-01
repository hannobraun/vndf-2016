#![feature(slicing_syntax)]


extern crate serialize;


pub use action::{
	Action,
	Step,
};
pub use encoder::{
	buf_writer,
	Encoder,
	MessageEncoder,
};
pub use perception::{
	Percept,
	Perception,
};


mod action;
mod encoder;
mod perception;


pub const MAX_PACKET_SIZE: uint = 512;


pub type Seq = u64;

#![feature(slicing_syntax)]


extern crate serialize;

extern crate acpe;


pub use action::{
	Action,
	Step,
};
pub use encoder::{
	Encoder,
	decode,
	MessageEncoder,
	MessagePart,
};
pub use perception::{
	Percept,
	Perception,
};


mod action;
mod encoder;
mod perception;

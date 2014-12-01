#![feature(slicing_syntax)]


extern crate serialize;

extern crate acpe;


pub use action::{
	Action,
	Step,
};
pub use perception::{
	Percept,
	Perception,
};


mod action;
mod perception;

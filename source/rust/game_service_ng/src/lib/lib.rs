#![feature(slicing_syntax)]


extern crate acpe;

extern crate common;


pub use socket::{
	ReceiveResult,
	Socket,
	SocketSender,
};


mod socket;
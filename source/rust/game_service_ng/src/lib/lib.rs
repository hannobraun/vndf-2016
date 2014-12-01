#![feature(slicing_syntax)]


extern crate acpe;

extern crate protocol_ng;


pub use socket::{
	ReceiveResult,
	Socket,
	SocketSender,
};


mod socket;
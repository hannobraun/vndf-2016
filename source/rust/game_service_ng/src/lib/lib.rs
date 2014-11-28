#![feature(slicing_syntax)]


extern crate protocol_ng;


pub use socket::{
	ReceiveResult,
	Socket,
	SocketSender,
};


mod socket;
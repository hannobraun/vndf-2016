#![feature(slicing_syntax)]


extern crate protocol_ng;


pub use socket::{
	Socket,
	SocketSender,
};


mod socket;
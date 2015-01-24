#![allow(unstable)]


extern crate acpe;

extern crate common;


pub use socket::{
	ReceiveResult,
	Socket,
};


pub mod network;
mod socket;

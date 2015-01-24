#![allow(unstable)]


extern crate acpe;

extern crate common;


pub mod network;
mod socket;


pub use socket::{
	ReceiveResult,
	Socket,
};

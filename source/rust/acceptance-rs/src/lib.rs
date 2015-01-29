#![feature(collections, core, io, os, path, rand)]


use std::old_io::net::ip::Port;
use std::rand::random;


pub use self::process::Process;
pub use tree::{
	Tree,
	TreeBuilder,
};


pub mod process;

mod tree;


pub fn random_port(min: Port, max: Port) -> Port {
	random::<Port>() % (max - min) + min
}

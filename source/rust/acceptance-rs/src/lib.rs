#![feature(core, env, io, path)]


extern crate rand;


pub mod process;
pub mod tree;


use std::old_io::net::ip::Port;

use rand::random;


pub use self::process::Process;
pub use tree::{
	Tree,
	TreeBuilder,
};


pub fn random_port(min: Port, max: Port) -> Port {
	random() % (max - min) + min
}

#![crate_id   = "common"]
#![crate_type = "rlib"]

#![feature(macro_rules)]
#![feature(phase)]


extern crate libc;
extern crate rand;
extern crate serialize;

extern crate rustecs;
#[phase(plugin)]
extern crate rustecs_macros;


pub mod ecs;
pub mod io;
pub mod json;
pub mod net;
pub mod physics;
pub mod protocol;
pub mod testing;

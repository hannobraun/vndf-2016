#![feature(phase)]


extern crate serialize;

extern crate cgmath;

extern crate rustecs;
#[phase(plugin)] extern crate rustecs_macros;


pub mod ecs;
pub mod physics;
pub mod util;

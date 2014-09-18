#![feature(phase)]


extern crate serialize;

extern crate cgmath;

extern crate net;
extern crate physics;
extern crate rustecs;
#[phase(plugin)] extern crate rustecs_macros;


pub mod ecs;

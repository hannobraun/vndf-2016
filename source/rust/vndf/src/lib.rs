#![feature(macro_rules)]
#![feature(phase)]


extern crate getopts;
extern crate libc;
extern crate serialize;
extern crate time;

extern crate freetype;
extern crate gl;
extern crate glfw;
extern crate stb_image;

extern crate epoll;
extern crate physics;
extern crate rustecs;
#[phase(plugin)] extern crate rustecs_macros;


pub mod client;
pub mod common;
pub mod gameservice;
pub mod net;

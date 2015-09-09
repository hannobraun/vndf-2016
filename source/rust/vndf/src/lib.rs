#![feature(drain)]


extern crate freetype;
extern crate getopts;
#[macro_use] extern crate gfx;
extern crate gfx_device_gl;
extern crate glutin;
extern crate libc;
#[macro_use] extern crate log;
extern crate nalgebra;
extern crate ncollide;
extern crate num;
extern crate rand;
extern crate rustc_serialize;
#[macro_use] extern crate scan_fmt;
extern crate time;
extern crate toml;


pub mod client;
pub mod server;
pub mod shared;
pub mod testing;

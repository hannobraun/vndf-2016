#![feature(drain)]

#[cfg(feature="default")] extern crate freetype;
extern crate getopts;
#[cfg(feature="default")] #[macro_use] extern crate gfx;
#[cfg(feature="default")] extern crate gfx_device_gl;
#[cfg(feature="default")] extern crate glutin;
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

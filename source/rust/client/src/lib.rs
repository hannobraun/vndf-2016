#![feature(drain)]


extern crate freetype;
#[macro_use]
extern crate gfx;
extern crate gfx_device_gl;
extern crate glutin;
extern crate getopts;
extern crate libc;
extern crate nalgebra;
extern crate rustc_serialize;
#[macro_use]
extern crate scan_fmt;


extern crate shared;


pub mod args;
pub mod cli;
pub mod debug;
pub mod font;
pub mod interface;
pub mod network;
pub mod render;
pub mod window;

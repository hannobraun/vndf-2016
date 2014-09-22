#![feature(phase)]


extern crate getopts;
extern crate libc;
extern crate serialize;
extern crate time;

extern crate cgmath;

extern crate game;
extern crate net;
extern crate physics;
extern crate platform;
extern crate platform_cli;
extern crate platform_desktop;
extern crate protocol;
extern crate rustecs;
#[phase(plugin)] extern crate rustecs_macros;


pub mod args;
pub mod ecs;
pub mod error;
pub mod gamestate;
pub mod inputsender;
pub mod network;
pub mod receiver;
pub mod run;

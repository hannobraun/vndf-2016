#![feature(phase)]


extern crate getopts;
extern crate serialize;

extern crate cgmath;

extern crate epoll;
extern crate game;
extern crate net;
extern crate physics;
extern crate protocol;
extern crate rustecs;
#[phase(plugin)] extern crate rustecs_macros;


pub mod args;
pub mod ecs;
pub mod events;
pub mod gamestate;
pub mod network;
pub mod run;
pub mod updater;

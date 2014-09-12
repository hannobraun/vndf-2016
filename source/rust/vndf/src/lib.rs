#![feature(macro_rules)]
#![feature(phase)]


extern crate getopts;
extern crate libc;
extern crate serialize;
extern crate time;

extern crate epoll;
extern crate game;
extern crate net;
extern crate physics;
extern crate platform;
extern crate platform_cli;
extern crate platform_desktop;
extern crate protocol;
extern crate rustecs;
#[phase(plugin)] extern crate rustecs_macros;
extern crate test_infra;


pub mod client;
pub mod game_service;
pub mod test_tools;

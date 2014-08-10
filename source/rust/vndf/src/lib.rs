#![feature(macro_rules)]
#![feature(phase)]


extern crate getopts;
extern crate libc;
extern crate serialize;
extern crate time;

extern crate epoll;
extern crate net;
extern crate physics;
extern crate platform;
extern crate platform_cli;
extern crate rustecs;
#[phase(plugin)] extern crate rustecs_macros;


pub mod client;
pub mod game;
pub mod game_service;
pub mod protocol;
pub mod test_infra;
pub mod test_tools;

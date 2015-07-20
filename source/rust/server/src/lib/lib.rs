#![feature(drain)]


extern crate getopts;
#[macro_use]
extern crate log;
extern crate nalgebra;

extern crate shared;


pub mod args;
pub mod clients;
pub mod game;
pub mod incoming_events;
pub mod network;

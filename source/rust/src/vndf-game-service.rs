extern crate getopts;
extern crate libc;
extern crate time;

extern crate rustecs;
extern crate common;


mod gameservice;

fn main() {
	gameservice::run::run();
}

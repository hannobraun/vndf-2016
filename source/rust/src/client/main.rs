extern crate getopts;
extern crate libc;
extern crate time;

extern crate freetype;
extern crate gl;
extern crate glfw;
extern crate stb_image;

extern crate rustecs;
extern crate common;


mod args;
mod error;
mod game;
mod headless;
mod inputsender;
mod network;
mod run;
mod ui;


#[link(name = "stb-image", kind = "static")]
extern {}


fn main() {
	run::run();
}

extern crate getopts;
extern crate libc;
extern crate time;

extern crate freetype;
extern crate gl;
extern crate glfw;
extern crate stb_image;

extern crate rustecs;
extern crate common;


mod client;


fn main() {
	client::run::run();
}

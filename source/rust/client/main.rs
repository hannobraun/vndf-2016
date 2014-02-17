extern crate common;

extern crate gl;
extern crate glfw;
extern crate stb_image;


use std::hashmap::HashMap;
use std::io;
use std::os;
use std::path;
use std::str;

use protocol::Connection;


mod camera;
mod display;
mod images;
mod input;
mod net;
mod protocol;
mod texture;


#[link(name = "stb-image", kind = "static")]
extern {}


fn main() {
	let screenWidth  = 800;
	let screenHeight = 600;

	let args = os::args();

	if args.len() > 2 {
		print!("Usage: {:s} serverAddress\n", args[0]);
		return
	}

	let serverAddress = if args.len() == 2 {
		args[1]
	}
	else {
		let mut file = match io::File::open(&path::posix::Path::new("server")) {
			Ok(file) => file,
			Err(e)   => {
				print!("ERROR {}\n", e);
				fail!();
			}
		};

		let contents = match file.read_to_end() {
			Ok(contents) => contents,
			Err(e)       => {
				print!("ERROR {}\n", e);
				fail!();
			}
		};

		str::from_utf8(contents).unwrap_or_else(|| { fail!() }).to_owned()
	};

	let window  = display::init(screenWidth, screenHeight);
	let texture = images::load();

	let socket_fd = net::connect(serverAddress, ~"34481");

	let mut c = Connection {
		socket_fd : socket_fd,
		buffer    : [0, ..protocol::BUFFER_SIZE],
		buffer_pos: 0 };

	unsafe {
		let mut positions = HashMap::new();

		let mut cam = camera::Camera {
			v: 0.0f32,
			h: 0.0f32 };

		while glfw::ffi::glfwWindowShouldClose(window.ptr) == 0 &&
			glfw::ffi::glfwGetKey(window.ptr, glfw::ffi::KEY_ESCAPE) == glfw::ffi::RELEASE {
			protocol::receive_positions(&mut c, &mut positions);
			input::apply(&window, &mut cam);
			display::render(&window, cam, &positions, texture);

			glfw::ffi::glfwPollEvents();
		}
	};
}

extern mod common;

extern mod gl;
extern mod glfw;
extern mod stb_image;


use std::io;
use std::libc;
use std::mem;
use std::os;
use std::path;
use std::ptr;
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

	let socketFD = serverAddress.to_c_str().with_ref(|c_address| {
		"34481".to_c_str().with_ref(|c_port| {
			net::net_connect(c_address, c_port)
		})
	});

	let mut c = Connection {
		socketFD : socketFD,
		buffer   : [0, ..protocol::BUFFER_SIZE],
		bufferPos: 0 };

	unsafe {
		let positions = display::PosMap {
			cap  : 4,
			elems: libc::malloc(4 * mem::size_of::<display::PosMapEntry>() as u64) as *mut display::PosMapEntry };
		ptr::zero_memory(positions.elems, 4);

		let mut cam = camera::Camera {
			v: 0.0f32,
			h: 0.0f32 };

		while glfw::ffi::glfwWindowShouldClose(window.ptr) == 0 &&
			glfw::ffi::glfwGetKey(window.ptr, glfw::ffi::KEY_ESCAPE) == glfw::ffi::RELEASE {
			protocol::receive_positions(&mut c, positions);
			input::apply(&window, &mut cam);
			display::render(&window, cam, positions, texture);

			glfw::ffi::glfwPollEvents();
		}
	};
}

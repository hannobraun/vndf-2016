extern mod common;

extern mod gl;
extern mod glfw;
extern mod stb_image;


use std::from_str;
use std::io;
use std::libc;
use std::mem;
use std::os;
use std::path;
use std::ptr;
use std::str;


pub mod camera;
pub mod display;
pub mod images;
pub mod input;
pub mod net;
pub mod texture;


#[link(name = "stb-image", kind = "static")]
extern {}


static BUFFER_SIZE : libc::c_int = 256;


struct Connection {
	socketFD : libc::c_int,
	buffer   : [libc::c_char, ..BUFFER_SIZE],
	bufferPos: libc::c_int
}


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

	let window = glfw::Window {
		ptr      : display::display_init(screenWidth, screenHeight),
		is_shared: true };
	let texture = images::images_load();

	let socketFD = serverAddress.to_c_str().with_ref(|c_address| {
		"34481".to_c_str().with_ref(|c_port| {
			net::net_connect(c_address, c_port)
		})
	});

	let mut c = Connection {
		socketFD : socketFD,
		buffer   : [0, ..BUFFER_SIZE],
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
			receivePositions(&mut c, positions);
			input::apply(&window, &mut cam);
			display::display_render(window.ptr, cam, positions, texture);
		}
	};
}

#[no_mangle]
pub extern fn receivePositions(c: *mut Connection, positions: display::PosMap) {
	unsafe {
		let bytesReceived = net::net_receive(
			(*c).socketFD,
			ptr::offset((*c).buffer.as_ptr(), (*c).bufferPos as int),
			(BUFFER_SIZE - (*c).bufferPos) as u64);

		(*c).bufferPos += bytesReceived as i32;

		while (*c).bufferPos > 0 && (*c).buffer[0] as i32 <= (*c).bufferPos {
			let messageSize = (*c).buffer[0];
			assert!(messageSize >= 0);

			let message = str::raw::from_buf_len(
				ptr::offset((*c).buffer.as_ptr() as *u8, 1),
				(messageSize - 1) as uint);

			if message.starts_with("UPDATE") {
				let parts: ~[&str] = message.words().collect();

				let id_str = parts[2].trim_chars(&',');
				let x_str  = parts[4].trim_chars(&',').trim_chars(&'(');
				let y_str  = parts[5].trim_chars(&')');

				let id: int = from_str::from_str(id_str).unwrap_or_else(|| { fail!() });

				let x: f32 = from_str::from_str(x_str).unwrap_or_else(|| { fail!() });
				let y: f32 = from_str::from_str(y_str).unwrap_or_else(|| { fail!() });


				(*ptr::mut_offset(positions.elems, id)).isOccupied = 1;
				(*ptr::mut_offset(positions.elems, id)).value = display::Pos { x: x, y: y };
			}
			else if message.starts_with("REMOVE") {
				let parts: ~[&str] = message.words().collect();

				let id_str = parts[2];

				let id: int = from_str::from_str(id_str).unwrap_or_else(|| { fail!() });

				(*ptr::mut_offset(positions.elems, id)).isOccupied = 0;
			}
			else {
				print!("Unknown message type in message: {:s}\n", message);
				fail!();
			}

			ptr::copy_memory(
				(*c).buffer.as_mut_ptr(),
				ptr::offset((*c).buffer.as_ptr(), messageSize as int),
				(BUFFER_SIZE - messageSize as i32) as uint);
			(*c).bufferPos -= messageSize as i32;
		}
	}
}

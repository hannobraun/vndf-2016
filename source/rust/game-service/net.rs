use libc;
use std::ptr;

use common::net;
use common::net::epoll;
use common::net::epoll::EPoll;


pub struct Net {
	pub epoll   : EPoll,
	pub serverFD: libc::c_int
}


pub fn init(port: &str) -> Net {
	let server_fd = net::init_socket(port);

	let epoll = match EPoll::create() {
		Ok(epoll)  => epoll,
		Err(error) => fail!("Error initializing epoll: {}", error)
	};

	match epoll.add(server_fd, epoll::ffi::EPOLLIN) {
		Err(error) =>
			fail!("Error registering server socket with epoll: {}", error),

		_ => ()
	}

	print!("Listening on port {}\n", port);

	Net {
		epoll   : epoll,
		serverFD: server_fd }
}

pub fn number_of_events(net: &Net, frameTimeInMs: i32) -> i32 {
	match net.epoll.wait(frameTimeInMs as u32) {
		Ok(number_of_events) => number_of_events as i32,

		Err(error) => fail!("Error while waiting for events: {}", error)
	}
}

pub fn accept_client(serverFD: libc::c_int) -> libc::c_int {
	unsafe {
		net::ffi::accept(
			serverFD,
			ptr::mut_null(),
			ptr::mut_null())
	}
}

pub fn send_message(clientFD: libc::c_int, message: &str) -> libc::c_int {
	let mut buffer: [libc::c_char, ..256] = [0, ..256];

	unsafe {
		message.to_c_str().with_ref(|c_message| {
			let messageLength = libc::strlen(c_message);

			ptr::set_memory(
				buffer.as_mut_ptr(),
				(messageLength + 1) as u8,
				1);

			ptr::copy_memory(
				buffer.as_mut_ptr().offset(1),
				c_message,
				messageLength as uint);

			let buffer_length = messageLength + 1;

			let bytesSent = net::ffi::send(
				clientFD,
				buffer.as_ptr() as *mut libc::c_void,
				buffer_length,
				net::ffi::MSG_NOSIGNAL);

			if bytesSent < 0 {
				"Error sending message".to_c_str().with_ref(|c_str| {
					libc::perror(c_str);
				});

				-1
			}
			else if bytesSent as u64 != buffer_length {
				format!(
					"Only sent {:d} of {:u} bytes.\n",
					bytesSent,
					buffer_length);
				libc::exit(1)
			}
			else {
				0
			}
		})
	}
}

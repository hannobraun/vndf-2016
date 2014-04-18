use libc;
use std::os::errno;

use common::net;


pub fn receive(socketFD: libc::c_int, buffer: &[u8]) -> libc::ssize_t {
	unsafe {
		let bytesReceived = net::ffi::recv(
			socketFD,
			buffer.as_ptr() as *mut libc::c_void,
			buffer.len() as u64,
			net::ffi::MSG_DONTWAIT);

		if bytesReceived == -1 && (errno() as i32 == net::ffi::EAGAIN || errno() as i32 == net::ffi::EWOULDBLOCK) {
			return 0;
		}
		if bytesReceived == -1 {
			"Error receiving message".to_c_str().with_ref(|c_str| {
				libc::perror(c_str);
				libc::exit(1);
			})
		}
		if bytesReceived == 0 {
			fail!("Connection closed while receiving");
		}

		bytesReceived
	}
}

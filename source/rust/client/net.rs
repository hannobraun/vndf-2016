use std::libc;


extern {
	fn __errno_location() -> *libc::c_int;
	fn recv(sockfd: libc::c_int, buf: *libc::c_void, len: libc::size_t, flags: libc::c_int) -> libc::ssize_t;
}

fn errno() -> libc::c_int {
	unsafe {
		*__errno_location()
	}
}


#[no_mangle]
pub extern fn net_receive(socketFD: libc::c_int, buffer: *libc::c_char, bufferSize: libc::size_t) -> libc::ssize_t {
	let MSG_DONTWAIT = 0x40;
	let EAGAIN       = 11;
	let EWOULDBLOCK  = 140;

	unsafe {
		let bytesReceived = recv(
			socketFD,
			buffer as *libc::c_void,
			bufferSize,
			MSG_DONTWAIT);

		if bytesReceived == -1 && (errno() == EAGAIN || errno() == EWOULDBLOCK) {
			return 0;
		}
		if bytesReceived == -1 {
			"Error receiving message".to_c_str().with_ref(|c_str| {
				libc::perror(c_str);
				libc::exit(1);
			})
		}
		if bytesReceived == 0 {
			print("Connection closed while receiving.\n");
			libc::exit(1);
		}

		bytesReceived
	}
}

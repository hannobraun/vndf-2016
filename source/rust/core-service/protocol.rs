use std::libc;
use std::ptr;


pub fn send_update(
	clientFD: libc::c_int,
	id      : libc::size_t,
	xPos    : libc::c_double,
	yPos    : libc::c_double) -> libc::c_int {

	unsafe {
		let mut buffer: [libc::c_char, ..256] = [0, ..256];

		let message =
			format!("UPDATE id: {:u}, pos: ({:f}, {:f})", id, xPos, yPos);

		message.to_c_str().with_ref(|c_str| {
			let messageLength = libc::strlen(c_str);

			ptr::set_memory(
				buffer.as_mut_ptr(),
				(messageLength + 1) as u8,
				1);

			ptr::copy_memory(
				ptr::mut_offset(buffer.as_mut_ptr(), 1),
				c_str,
				messageLength as uint);

			::net::send_message(clientFD, buffer.as_ptr(), messageLength + 1)
		})
	}
}

pub fn send_remove(clientFD: libc::c_int, id: libc::size_t) -> libc::c_int {
	unsafe {
		let mut buffer: [libc::c_char, ..256] = [0, ..256];

		("REMOVE id: " + id.to_str()).to_c_str().with_ref(|c_str| {
			let messageLength = libc::strlen(c_str);

			ptr::set_memory(
				buffer.as_mut_ptr(),
				(messageLength + 1) as u8,
				1);

			ptr::copy_memory(
				ptr::mut_offset(buffer.as_mut_ptr(), 1),
				c_str,
				messageLength as uint);

			::net::send_message(clientFD, buffer.as_ptr(), messageLength + 1)
		})
	}
}

#[crate_type = "rlib"];
#[crate_type = "staticlib"];
#[link(name = "protocol", package_id = "protocol", vers = "0.0")];


extern mod net;


#[no_mangle]
pub extern fn sendRemove(clientFD: std::libc::c_int, id: std::libc::size_t) -> std::libc::c_int {
	unsafe {
		let mut message: [std::libc::c_char, ..256] = [0, ..256];

		("REMOVE id: " + id.to_str()).to_c_str().with_ref(|c_str| {
			let messageLength = std::libc::strlen(c_str);

			std::ptr::set_memory(
				message.as_mut_ptr(),
				(messageLength + 1) as u8,
				1);

			std::ptr::copy_memory(
				std::ptr::mut_offset(message.as_mut_ptr(), 1),
				c_str,
				messageLength as uint);

			net::net_send(clientFD, message.as_ptr(), messageLength + 1)
		})
	}
}

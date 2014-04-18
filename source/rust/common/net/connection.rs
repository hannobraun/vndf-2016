use libc::c_int;


pub struct Connection {
	pub fd: c_int
}

impl Connection {
	pub fn create(fd: c_int) -> Connection {
		Connection {
			fd: fd
		}
	}
}

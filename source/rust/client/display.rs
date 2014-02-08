use std::libc;
use std::ptr;

use glfw;


#[no_mangle]
pub extern fn createWindow(width: libc::c_int, height: libc::c_int) -> *glfw::ffi::GLFWwindow {
	match glfw::init() {
		Err(_) => fail!("Could not initialize GLFW."),
		_      => ()
	}


	unsafe {
		let window = "Von Neumann Defense Force".with_c_str(|c_str| {
			glfw::ffi::glfwCreateWindow(
				width, height,
				c_str,
				ptr::null(), ptr::null())
		});
		assert!(window != ptr::null());

		glfw::ffi::glfwMakeContextCurrent(window);

		window
	}
}

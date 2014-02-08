use std::f64;
use std::libc;
use std::ptr;

use gl;
use glfw;


#[no_mangle]
pub extern fn display_init(screenWidth: libc::c_int, screenHeight: libc::c_int) -> *glfw::ffi::GLFWwindow {
	let window = createWindow(screenWidth, screenHeight);

	gl::load_with(glfw::get_proc_address);

	gl::Enable(gl::TEXTURE_2D);

	gl::Enable(gl::BLEND);
	gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

	gl::LoadIdentity();

	// I'm not a 100% sure what this does, but it has to do with using textures
	// that are not power of two. Before I added this call, glTexture2D wouldn't
	// work correctly on an 11x11 texture, causing memory access errors and not
	// displaying it correctly.
	gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);

	let zNear = 0.1;
	let fovAngleY = 45.0;
	let halfHeight = f64::tan( fovAngleY / 360.0 * f64::consts::PI ) * zNear;
	let halfWidth = halfHeight * screenWidth as f64 / screenHeight as f64;
	gl::Frustum(
		-halfWidth, halfWidth,
		-halfHeight, halfHeight,
		zNear, 1000.0);

	window
}

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

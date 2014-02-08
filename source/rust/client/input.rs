use glfw;

use camera;


static cameraSpeed: f32 = 1.0;


#[no_mangle]
pub extern fn input_apply(ffiWindow: &glfw::ffi::GLFWwindow, camera: &mut camera::Camera) {
	let window = glfw::Window {
		ptr      : ffiWindow,
		is_shared: true };

	if window.get_key(glfw::KeyRight) == glfw::Press {
		camera.h -= cameraSpeed;
	}
	if window.get_key(glfw::KeyLeft) == glfw::Press {
		camera.h += cameraSpeed;
	}
	if window.get_key(glfw::KeyUp) == glfw::Press {
		camera.v += cameraSpeed;
	}
	if window.get_key(glfw::KeyDown) == glfw::Press {
		camera.v -= cameraSpeed;
	}

	if camera.v >= 90.0 {
		camera.v = 90.0;
	}
	if camera.v <= -90.0 {
		camera.v = -90.0;
	}
}

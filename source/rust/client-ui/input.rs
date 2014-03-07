use glfw;

use camera::Camera;


static CAMERA_SPEED: f64 = 4.0;


pub fn apply(window: &glfw::Window, camera: &mut Camera) {
	if window.get_key(glfw::KeyRight) == glfw::Press {
		camera.h -= CAMERA_SPEED;
	}
	if window.get_key(glfw::KeyLeft) == glfw::Press {
		camera.h += CAMERA_SPEED;
	}
	if window.get_key(glfw::KeyUp) == glfw::Press {
		camera.v += CAMERA_SPEED;
	}
	if window.get_key(glfw::KeyDown) == glfw::Press {
		camera.v -= CAMERA_SPEED;
	}

	if camera.v >= 90.0 {
		camera.v = 90.0;
	}
	if camera.v <= -90.0 {
		camera.v = -90.0;
	}
}

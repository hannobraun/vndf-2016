use glfw;

use camera::Camera;


static cameraSpeed: f32 = 1.0;


pub fn apply(window: &glfw::Window, camera: &mut Camera) {
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

use std::f64;
use std::libc;
use std::ptr;

use gl;
use glfw;

use camera;
use texture;


pub struct PosMap {
	cap: libc::size_t,
	elems: *mut PosMapEntry
}

pub struct PosMapEntry {
	isOccupied: libc::c_int,
	value     : Pos
}

pub struct Pos {
	x: f32,
	y: f32
}


pub fn init(screenWidth: libc::c_int, screenHeight: libc::c_int) -> *glfw::ffi::GLFWwindow {
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

#[no_mangle]
pub extern fn display_render(window: *glfw::ffi::GLFWwindow, camera: camera::Camera, positions: PosMap, texture: texture::Texture) {
	gl::Clear(gl::COLOR_BUFFER_BIT);

	gl::PushMatrix();

	gl::Translatef(0.0f32, 0.0f32, -500.0f32);
	gl::Rotatef(camera.v, 1.0f32, 0.0f32, 0.0f32);
	gl::Rotatef(camera.h, 0.0f32, 1.0f32, 0.0f32);

	gl::BindTexture(
		gl::TEXTURE_2D,
		texture.name);

	gl::Color4f(1.0f32, 1.0f32, 1.0f32, 1.0f32);

	unsafe {
		let mut i: int = 0;
		while i < positions.cap as int {

			if (*ptr::mut_offset(positions.elems, i)).isOccupied > 0 {
				gl::PushMatrix();

				gl::Translatef(
					(*ptr::mut_offset(positions.elems, i)).value.x - texture.width as f32 / 2f32,
					(*ptr::mut_offset(positions.elems, i)).value.y - texture.height as f32 / 2f32,
					0.0f32);

				gl::Begin(gl::TRIANGLE_STRIP);
					gl::TexCoord2f(1.0f32, 0.0f32);
					gl::Vertex3f(
						texture.width as f32,
						texture.height as f32,
						0.0f32);

					gl::TexCoord2f(1.0f32, 1.0f32);
					gl::Vertex3f(texture.width as f32, 0.0f32, 0.0f32);

					gl::TexCoord2f(0.0f32, 0.0f32);
					gl::Vertex3f(0.0f32, texture.height as f32, 0.0f32);

					gl::TexCoord2f(0.0f32, 1.0f32);
					gl::Vertex3f(0.0f32, 0.0f32, 0.0f32);
				gl::End();

				gl::PopMatrix();
			}
			i += 1
		}

		gl::PopMatrix();

		glfw::ffi::glfwSwapBuffers(window);
		glfw::ffi::glfwPollEvents();
	}
}

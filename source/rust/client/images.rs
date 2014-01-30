use std::libc;

use gl;
use glfw;
use stb_image::image;


struct Image {
	data  : *libc::c_uchar,
	width : libc::c_int,
	height: libc::c_int
}

struct Texture {
	name  : gl::types::GLuint,
	width : libc::c_int,
	height: libc::c_int
}


#[no_mangle]
pub extern fn images_load() -> Texture {
	let image       = load_image();
	let textureName = createTexture(image);

	Texture {
		name  : textureName,
		width : image.width,
		height: image.height }
}

fn load_image() -> Image {
	match image::load(~"images/spaceship.png") {
		image::ImageU8(image) => {
			Image {
				data  : image.data.as_ptr(),
				width : image.width  as libc::c_int,
				height: image.height as libc::c_int }
		},

		image::ImageF32(_)    => fail!("Unexpected image type: ImageF32"),
		image::Error(message) => fail!(message)
	}
}

fn createTexture(image: Image) -> gl::types::GLuint {
	gl::load_with(glfw::get_proc_address);

	let mut textureName: gl::types::GLuint = 0;
	unsafe {
		// Generate texture names.
		gl::GenTextures(1, &mut textureName);

		gl::BindTexture(
			gl::TEXTURE_2D,
			textureName);

		// Configure texture.
		gl::TexParameteri(
			gl::TEXTURE_2D,
			gl::TEXTURE_MIN_FILTER,
			gl::NEAREST as i32);

		// Bind image data to texture name.
		gl::TexImage2D(
			gl::TEXTURE_2D,
			0,
			gl::RGBA8 as i32,
			image.width,
			image.height,
			0,
			gl::RGBA,
			gl::UNSIGNED_BYTE,
			image.data as *libc::c_void);
	}

	textureName
}

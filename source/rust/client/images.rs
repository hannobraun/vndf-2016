use std::libc;

use gl;
use stb_image::image;

use texture::Texture;


struct Image {
	data  : ~[u8],
	width : libc::c_int,
	height: libc::c_int
}


pub fn load() -> Texture {
	let image       = load_image();
	let textureName = create_texture(&image);

	Texture {
		name  : textureName,
		width : image.width as uint,
		height: image.height as uint }
}

fn load_image() -> Image {
	match image::load(~"images/spaceship.png") {
		image::ImageU8(image) => {
			let width  = image.width;
			let height = image.height;

			Image {
				data  : image.data,
				width : width  as libc::c_int,
				height: height as libc::c_int }
		},

		image::ImageF32(_)    => fail!("Unexpected image type: ImageF32"),
		image::Error(message) => fail!(message)
	}
}

fn create_texture(image: &Image) -> gl::types::GLuint {
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
			image.data.as_ptr() as *libc::c_void);
	}

	textureName
}

use std::hashmap::HashMap;
use std::libc;

use gl;
use stb_image::image;

use texture::Texture;


struct Image {
	data  : ~[u8],
	width : uint,
	height: uint
}


pub fn load() -> HashMap<~str, Texture> {
	let image_path = ~"images/spaceship.png";

	let image        = load_image(image_path);
	let texture_name = create_texture(&image);

	let texture = Texture {
		name  : texture_name,
		width : image.width,
		height: image.height };

	let mut images = HashMap::new();
	images.insert(image_path, texture);

	images
}

fn load_image(image_path: &str) -> Image {
	match image::load(image_path.into_owned()) {
		image::ImageU8(image) => {
			let width  = image.width;
			let height = image.height;

			Image {
				data  : image.data,
				width : width,
				height: height }
		},

		image::ImageF32(_)    => fail!("Unexpected image type: ImageF32"),
		image::Error(message) => fail!(message)
	}
}

fn create_texture(image: &Image) -> gl::types::GLuint {
	let mut texture_name: gl::types::GLuint = 0;

	unsafe {
		// Generate texture names.
		gl::GenTextures(1, &mut texture_name);

		gl::BindTexture(
			gl::TEXTURE_2D,
			texture_name);

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
			image.width as i32,
			image.height as i32,
			0,
			gl::RGBA,
			gl::UNSIGNED_BYTE,
			image.data.as_ptr() as *libc::c_void);
	}

	texture_name
}

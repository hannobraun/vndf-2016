use std::libc;

use gl;


pub type Name = gl::types::GLuint;


pub struct Texture {
	name  : Name,
	width : uint,
	height: uint
}

impl Texture {
	pub fn new_rgb(data: &[u8], width: uint, height: uint) -> Texture {
		create_texture(data, width, height)
	}
}


fn create_texture(data: &[u8], width: uint, height: uint) -> Texture {
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
			width as i32,
			height as i32,
			0,
			gl::RGBA,
			gl::UNSIGNED_BYTE,
			data.as_ptr() as *libc::c_void);
	}

	Texture {
		name  : texture_name,
		width : width,
		height: height }
}

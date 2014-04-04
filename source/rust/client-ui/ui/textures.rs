use collections::HashMap;
use std::libc;

use gl;

use common::vec::Vec2;

use ui::Window;


pub struct Textures {
	map: HashMap<~str, Texture>
}

pub struct Texture {
	name: Name,
	size: Vec2
}

pub type Name = gl::types::GLuint;


impl Textures {
	pub fn init(_: &Window) -> Textures {
		// The window argument isn't actually used. It's here to document this
		// function's dependence on an OpenGL context, which is implied by
		// Window.

		gl::Enable(gl::TEXTURE_2D);

		gl::Enable(gl::BLEND);
		gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

		// I'm not a 100% sure what this does, but it has to do with using
		// textures that are not power of two. Before I added this call,
		// glTexture2D wouldn't work correctly on an 11x11 texture, causing
		// memory access errors and not displaying it correctly.
		gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);

		Textures {
			map: HashMap::new()
		}
	}

	pub fn get<'a>(&'a self, key: &~str) -> &'a Texture {
		self.map.get(key)
	}

	pub fn add(&mut self, key: ~str, texture: Texture) {
		if self.map.contains_key(&key) {
			fail!("texture already present ({})", key);
		}

		self.map.insert(key, texture);
	}
}


impl Texture {
	pub fn new_alpha(data: &[u8], size: Vec2) -> Texture {
		create_texture(data, size, gl::ALPHA, gl::ALPHA8)
	}

	pub fn new_rgb(data: &[u8], width: uint, height: uint) -> Texture {
		let size = Vec2 { x: width as f64, y: height as f64 };
		create_texture(data, size, gl::RGBA, gl::RGBA8)
	}
}


fn create_texture(
	data           : &[u8],
	size           : Vec2,
	format         : gl::types::GLenum,
	internal_format: gl::types::GLenum
	) -> Texture {

	let mut texture_name: gl::types::GLuint = 0;

	// Generate texture names.
	unsafe {
		gl::GenTextures(1, &mut texture_name);
	}

	// Not sure what these to do, but they're required.
	gl::BindTexture(
		gl::TEXTURE_2D,
		texture_name);
	gl::TexParameteri(
		gl::TEXTURE_2D,
		gl::TEXTURE_MIN_FILTER,
		gl::NEAREST as i32);

	// By default, textures are configured to repeat. Since our textures fill
	// the complete shapes they're rendered on, they won't repeat. However, the
	// setting leads to visual artifacts around the texture border. The
	// following configuration prevents that.
	gl::TexParameteri(
		gl::TEXTURE_2D,
		gl::TEXTURE_WRAP_S,
		gl::CLAMP_TO_EDGE as i32);
	gl::TexParameteri(
		gl::TEXTURE_2D,
		gl::TEXTURE_WRAP_T,
		gl::CLAMP_TO_EDGE as i32);

	// Bind image data to texture name.
	unsafe {
		gl::TexImage2D(
			gl::TEXTURE_2D,
			0,
			internal_format as i32,
			size.x as i32,
			size.y as i32,
			0,
			format,
			gl::UNSIGNED_BYTE,
			data.as_ptr() as *libc::c_void);
	}

	Texture {
		name: texture_name,
		size: size
	}
}

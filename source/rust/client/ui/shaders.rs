use collections::HashMap;
use std::io::File;
use std::ptr;

use gl;
use gl::types::{
	GLenum,
	GLuint
};

use error::exit;
use ui::Window;


pub struct Shaders {
	shaders: ShaderMap
}

type ShaderMap = HashMap<~str, Shader>;
type Shader    = GLuint;


impl Shaders {
	pub fn new(_: &Window) -> Shaders {
		// The window argument isn't actually used. It's here to document this
		// function's dependence on an OpenGL context, which is implied by
		// Window.

		let mut shaders = HashMap::new();
		create_shader(gl::VERTEX_SHADER, "glsl/ui-overlay.vert", &mut shaders);
		create_shader(gl::FRAGMENT_SHADER, "glsl/ui-overlay.frag", &mut shaders);

		Shaders {
			shaders: shaders
		}
	}

	pub fn shader<'a>(&self, key: &str) -> Shader {
		match self.shaders.find(&key.to_owned()) {
			Some(&shader) => shader,
			None          => exit(format!("Shader not found: {}", key))
		}
	}
}


fn create_shader(kind: GLenum, path: &str, shaders: &mut ShaderMap) {
	let shader = gl::CreateShader(kind);
	unsafe {
		gl::ShaderSource(
			shader,
			1,
			&load_shader(path).to_c_str().unwrap(),
			ptr::null());
	}
	gl::CompileShader(shader);
	shaders.insert(path.to_owned(), shader);
}

fn load_shader(path: &str) -> ~str {
	match File::open(&Path::new(path)).read_to_str() {
		Ok(string) => string,
		Err(error) => fail!("Error loading shader: {}", error)
	}
}

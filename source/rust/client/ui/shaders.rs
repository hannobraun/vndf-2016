use collections::HashMap;
use std::io::File;

use gl;

use error::exit;
use ui::Window;


pub struct Shaders {
	shaders: HashMap<~str, Shader>
}

type Shader = gl::types::GLuint;


impl Shaders {
	pub fn new(_: &Window) -> Shaders {
		// The window argument isn't actually used. It's here to document this
		// function's dependence on an OpenGL context, which is implied by
		// Window.

		let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
		unsafe {
			gl::ShaderSource(
				vertex_shader,
				1,
				&load_shader("glsl/ui-overlay.vert").to_c_str().unwrap(),
				::std::ptr::null());
		}
		gl::CompileShader(vertex_shader);

		let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
		unsafe {
			gl::ShaderSource(
				fragment_shader,
				1,
				&load_shader("glsl/ui-overlay.frag").to_c_str().unwrap(),
				::std::ptr::null());
		}
		gl::CompileShader(fragment_shader);

		let mut shaders = HashMap::new();
		shaders.insert("glsl/ui-overlay.vert".to_owned(), vertex_shader);
		shaders.insert("glsl/ui-overlay.frag".to_owned(), fragment_shader);

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


fn load_shader(path: &str) -> ~str {
	match File::open(&Path::new(path)).read_to_str() {
		Ok(string) => string,
		Err(error) => fail!("Error loading shader: {}", error)
	}
}
